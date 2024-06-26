use std::sync::Arc;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_schema::migration::MigratorTrait;
use testcontainers_async::{DatabaseContainer, Image};
use testcontainers_async::modules::postgresql::{PostgresContainer, PostgresImage};
use crate::migrations;
use crate::settings::PersistenceSettings;

pub type DbResult<T> = Result<T, DbErr>;

#[derive(Clone, Debug)]
pub struct {{ ProjectName }}Persistence {
    connection: DatabaseConnection,
    #[allow(dead_code)]
    temp_db: Option<Arc<PostgresContainer>>,
}

impl {{ ProjectName }}Persistence {
    pub fn builder() -> Builder {
        Builder {
            settings: PersistenceSettings::default(),
        }
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn migrate_up(&self, steps: Option<u32>) -> DbResult<()> {
        migrations::Migrator::up(self.connection(), steps).await
    }

    pub async fn migrate_down(&self, steps: Option<u32>) -> DbResult<()> {
        migrations::Migrator::down(self.connection(), steps).await
    }
}

pub struct Builder {
    settings: PersistenceSettings,
}

impl Builder {
    pub fn with_settings(mut self, settings: &PersistenceSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub fn with_temp_db(mut self) -> Self {
        self.settings.set_temp_db(true);
        self
    }

    pub async fn build(self) -> anyhow::Result<{{ ProjectName }}Persistence> {
        let (connect_url, temp_db) = if let Some(true) = self.settings.temp_db() {
            let temp_db = PostgresImage::default()
                .with_database("{{ project-name }}")
                .with_username("test")
                .start_container()
                .await?;

            let connect_url = temp_db.connect_url().await?;
            tracing::info!("TestContainer RDBC URL: {connect_url}");
            let jdbc_url = temp_db.jdbc_url().await?;
            tracing::info!("TestContainer JDBC URL: {jdbc_url}");
            let connect_cli = temp_db.connect_cli().await?;
            tracing::info!("TestContainer Connect CLI: {connect_cli}");

            (connect_url, Some(Arc::new(temp_db)))
        } else {
            (self.settings.database().url().to_string(), None)
        };

        let mut options = ConnectOptions::new(connect_url);
        if let Some(value) = self.settings.database().max_connections() {
            options.max_connections(value);
        }
        if let Some(value) = self.settings.database().min_connections() {
            options.min_connections(value);
        }
        if let Some(value) = self.settings.database().connect_timeout() {
            options.connect_timeout(value);
        }
        if let Some(value) = self.settings.database().idle_timeout() {
            options.idle_timeout(value);
        }
        if let Some(value) = self.settings.database().max_lifetime() {
            options.max_lifetime(value);
        }
        options.sqlx_logging(self.settings.database().log_sql());

        let connection: DatabaseConnection = Database::connect(options).await?;

        if self.settings.migrate().or(Some(false)).unwrap() || temp_db.is_some() {
            migrations::Migrator::up(&connection, None).await?;
        }

        Ok({{ ProjectName }}Persistence { connection, temp_db })
    }
}

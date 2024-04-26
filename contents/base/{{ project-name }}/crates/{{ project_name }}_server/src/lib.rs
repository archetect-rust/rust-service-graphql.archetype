use std::sync::Arc;

use anyhow::Result;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    Router,
    routing::{get, post_service },
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use {{ project_name }}_core::{{ ProjectName }}Core;

use crate::settings::ServerSettings;

pub mod settings;

#[derive(Clone)]
pub struct {{ ProjectName }}Server {
    core: {{ ProjectName }}Core,
    service_port: u16,
    listener: Arc<Mutex<Option<TcpListener>>>,
}

pub struct Builder {
    settings: ServerSettings,
    core: {{ ProjectName }}Core,
}

impl Builder {
    pub fn new(core: {{ ProjectName }}Core) -> Builder {
        Builder {
            settings: ServerSettings::default(),
            core,
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Builder {
        self.settings = settings.clone();
        self
    }

    pub fn with_random_port(mut self) -> Builder {
        self.settings.service_mut().set_port(0);
        self
    }

    pub async fn build(self) -> Result<{{ ProjectName }}Server> {
        let listener = TcpListener::bind((self.settings.host(), self.settings.service().port())).await?;
        let addr = listener.local_addr()?;

        Ok({{ ProjectName }}Server {
            core: self.core,
            service_port: addr.port(),
            listener: Arc::new(Mutex::new(Some(listener))),
        })
    }

}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

impl {{ ProjectName }}Server {
    pub fn builder(core: {{ ProjectName }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

pub async fn serve(&self) -> Result<()> {
    let schema = {{ project_name }}_graphql::schema::create_schema();

    let router = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(graphql_playground));

    let listener = self.listener.lock().await.take().expect("Listener Expected");

    tracing::info!("{{ ProjectName }} starting on {}", listener.local_addr()?);
    tracing::info!("GraphiQL: http://{}/graphiql", listener.local_addr()?);
    tracing::info!("Playground: http://{}/playground", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
}

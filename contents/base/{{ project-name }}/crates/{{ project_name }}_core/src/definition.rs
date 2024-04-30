use anyhow::Result;
use {{ project_name }}_persistence::{{ ProjectName }}Persistence;
use crate::settings::CoreSettings;

#[derive(Clone, Debug)]
pub struct {{ ProjectName }}Core {
    persistence:{{ ProjectName }}Persistence,
}

impl {{ ProjectName }}Core {
    pub fn builder(persistence: {{ ProjectName }}Persistence) -> Builder {
        Builder::new(persistence)
    }
}

pub struct Builder {
    persistence: {{ ProjectName }}Persistence,
    settings: CoreSettings,
}

impl Builder {
    pub fn new(persistence: {{ ProjectName }}Persistence) -> Self {
        Self {
            persistence,
            settings: Default::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ ProjectName }}Core> {
        Ok({{ ProjectName }}Core {
            persistence: self.persistence,
        })
    }
}
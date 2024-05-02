use anyhow::{Context, Result};
use tonic::transport::Channel;
use tracing::info;

use {{ project_name }}_persistence::{{ ProjectName }}Persistence;
{%- for application_key in applications %}
{%- set application = applications[application_key] %}
use crate::proto::{{ application["project_name"] }}_client::{{ application["ProjectName"] }}Client;
{%- endfor %}
use crate::settings::CoreSettings;

#[derive(Clone, Debug)]
pub struct {{ ProjectName }}Core {
    persistence:{{ ProjectName }}Persistence,
{%- for application_key in applications %}
{%- set application = applications[application_key] %}
    {{ application["project_name"] }}:{{ application["ProjectName"] }}Client<Channel>,
{%- endfor %}
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
        {%- for application_key in applications %}
        {%- set application = applications[application_key] %}
        info!("Connecting to {{ application['ProjectName'] }} at {}", self.settings.{{ application["project_name"] }}().url());
        let {{ application["project_name"] }} = {{ application["ProjectName"] }}Client::connect(
            self.settings.{{ application["project_name"] }}().url().to_string()
        ).await.context("Unable to connect to {{ application['ProjectName'] }}")?;

        {% endfor -%}
        Ok({{ ProjectName }}Core {
            persistence: self.persistence,
        {%- for application_key in applications %}
        {%- set application = applications[application_key] %}
            {{ application["project_name"] }},
        {%- endfor %}
        })
    }
}
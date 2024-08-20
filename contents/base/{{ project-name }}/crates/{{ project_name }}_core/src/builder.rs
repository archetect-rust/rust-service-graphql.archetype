use anyhow::{Context, Result};
use tonic::transport::Channel;

{% if persistence != "None" %}use {{ project_name }}_persistence::{{ ProjectName }}Persistence;{% endif %}
{%- for application_key in applications %}
{%- set application = applications[application_key] %}
use crate::proto::{{ application["project_name"] }}_client::{{ application["ProjectName"] }}Client;
{%- endfor %}
use crate::settings::CoreSettings;

#[derive(Clone, Debug)]
pub struct {{ ProjectName }}Core {{'{'}}{%if persistence != "None" %}
    persistence:{{ ProjectName }}Persistence,{% endif %}
{%- for application_key in applications %}
{%- set application = applications[application_key] %}
    {{ application["project_name"] }}:{{ application["ProjectName"] }}Client<Channel>,
{%- endfor %}
}

impl {{ ProjectName }}Core {
    pub fn builder({% if persistence != "None" %}persistence: {{ ProjectName }}Persistence{% endif %}) -> Builder {
        Builder::new({% if persistence != "None" %}persistence{% endif %})
    }{% if persistence != "None" %}

    pub fn persistence(&self) -> &{{ ProjectName }}Persistence {
        &self.persistence
    }{% endif %}
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}

    pub fn {{ application["project_name"] }}(&self) -> {{ application["ProjectName"] }}Client<Channel> {
        self.{{ application["project_name"] }}.clone()
    }
    {%- endfor %}
}

pub struct Builder {{'{'}}{% if persistence != "None" %}
    persistence: {{ ProjectName }}Persistence,{% endif %}
    settings: CoreSettings,
}

impl Builder {
    pub fn new({% if persistence != "None" %}persistence: {{ ProjectName }}Persistence{% endif %}) -> Self {
        Self {{'{'}}{% if persistence != "None" %}
            persistence,{% endif %}
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
        Ok({{ ProjectName }}Core  {{'{'}}{% if persistence != "None" %}
            persistence: self.persistence,{% endif %}
        {%- for application_key in applications %}
        {%- set application = applications[application_key] %}
            {{ application["project_name"] }},
        {%- endfor %}
        })
    }
}
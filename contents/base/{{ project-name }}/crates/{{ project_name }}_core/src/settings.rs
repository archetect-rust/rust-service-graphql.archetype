use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoreSettings {
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
    {{ application['project_name'] }}: ClientConfig,
    {%- endfor %}
}

impl CoreSettings {
    pub fn new(customer_service: ClientConfig, account_service: ClientConfig) -> CoreSettings {
        CoreSettings {
            {%- for application_key in applications %}
            {%- set application = applications[application_key] %}
            {{ application['project_name'] }},
            {%- endfor %}
        }
    }
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}

    pub fn {{ application['project_name'] }}(&self) -> &ClientConfig {
        &self.{{ application['project_name'] }}
    }


    pub fn set_{{ application['project_name'] }}(&mut self, {{ application['project_name'] }}: ClientConfig) {
        self.{{ application['project_name'] }} = {{ application['project_name'] }};
    }
    {%- endfor %}

}

impl Default for CoreSettings {
    fn default() -> Self {
        CoreSettings {
            {%- for application_key in applications %}
            {%- set application = applications[application_key] %}
            {{ application['project_name'] }}: Default::default(),
            {%- endfor %}
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientConfig {
    #[serde(default = "default_client_url")]
    url: Url,
}

impl ClientConfig {
    pub fn new(url: Url) -> ClientConfig {
        ClientConfig {
            url,
        }
    }


    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            url: default_client_url(),
        }
    }
}

fn default_client_url() -> Url {
    Url::parse("http://localhost:8080").expect("Valid Url")
}
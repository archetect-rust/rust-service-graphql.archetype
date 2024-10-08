{% import "macros/rust" as rust -%}
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoreSettings {
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
    {{ application['project_name'] }}: ClientConfig,
    {%- endfor %}
}

impl CoreSettings {
    pub fn new({{ rust.core_settings_args(applications) }}) -> CoreSettings {
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
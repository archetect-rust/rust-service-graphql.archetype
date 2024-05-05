mod common;

{%- for application_key in applications %}
{%- set application = applications[application_key] %}

{% for entity_key in application.model.entities %}
{%- set entity = application.model.entities[entity_key] %}
mod {{ entity["entity_name"] }};
{%- endfor %}
{%- endfor %}

pub use common::*;
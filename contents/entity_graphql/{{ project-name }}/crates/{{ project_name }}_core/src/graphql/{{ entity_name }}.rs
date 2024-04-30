{% import "macros/rust" as rust -%}
use async_graphql::{Description, ID, Object};

/// Shopping Cart
#[derive(Description)]
pub struct {{ EntityName }} {
    pub id: ID,
{%- for field_key in fields -%}
{%- set field = fields[field_key] %}
    pub {{ field["field_name"] }}: {{ rust.field_rust_type(field) }},
{%- endfor %}
}

#[Object(use_type_description)]
impl {{ EntityName  }} {
    /// Get ID
    async fn id(&self) -> ID {
        self.id.clone()
    }

{%- for field_key in fields %}
{%- set field = fields[field_key] %}

    async fn {{ field["field_name"] }}(&self) -> &{{ rust.field_rust_type(field) }} {
        &self.{{ field["field_name"] }}
    }
{%- endfor %}
}
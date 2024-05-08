{% import "macros/rust" as rust -%}
use async_graphql::{Description, ID, InputObject, Object, SimpleObject};

/// {{ EntityName }}
#[derive(Description)]
pub struct {{ EntityName }} {
    pub id: Option<ID>,
{%- for field_key in fields -%}
{%- set field = fields[field_key] %}
    pub {{ field["field_name"] }}: {{ rust.field_rust_type(field) }},
{%- endfor %}
}

/// {{ EntityName }} Input
#[derive(Description, InputObject)]
pub struct {{ EntityName }}Input {
    pub id: Option<ID>,
    {%- for field_key in fields -%}
    {%- set field = fields[field_key] %}
    pub {{ field["field_name"] }}: {{ rust.field_rust_type(field) }},
    {%- endfor %}
}

#[Object(use_type_description)]
impl {{ EntityName  }} {
    /// Get ID
    async fn id(&self) -> &Option<ID> {
        &self.id
    }

{%- for field_key in fields %}
{%- set field = fields[field_key] %}

    async fn {{ field["field_name"] }}(&self) -> &{{ rust.field_rust_type(field) }} {
        &self.{{ field["field_name"] }}
    }
{%- endfor %}
}

#[derive(SimpleObject)]
pub struct {{ EntityName | pluralize }}Page {
    /// Account Records
    pub records: Vec<{{ EntityName }}>,
    /// Page Index
    pub index: u32,
    /// Next Page Index
    pub next: u32,
    /// Has Next Page
    pub has_next: bool,
    /// Previous Page Index
    pub previous: u32,
    /// Has Previous Page
    pub has_previous: bool,
    /// Total Pages
    pub total: u32,
    /// Total Customer Records
    pub total_records: u64,
}
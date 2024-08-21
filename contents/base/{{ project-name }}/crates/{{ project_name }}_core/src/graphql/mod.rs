use async_graphql::{EmptySubscription, Schema, SchemaBuilder};

mod mutation;
mod query;

{%- for application_key in applications %}
{%- set application = applications[application_key] %}
{%- for entity_key in application.model.entities %}
{%- set entity = application.model.entities[entity_key] %}
mod {{ entity["entity_name"] }};
{%- endfor %}
{%- endfor %}


pub use mutation::MutationRoot;
pub use query::QueryRoot;

{%- for application_key in applications %}
{%- set application = applications[application_key] %}
{%- for entity_key in application.model.entities %}
{%- set entity = application.model.entities[entity_key] %}
pub use {{ entity["entity_name"] }}::*;
{%- endfor %}
{%- endfor %}

pub fn create_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}

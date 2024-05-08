use async_graphql::{Context, Object, Result};

use crate::{ConvertFrom, ConvertTo, {{ ProjectName }}Core, proto, graphql};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
    {%- for entity_key in application.model.entities %}
    {%- set entity = application.model.entities[entity_key] %}
    #[graphql(name = "create{{ entity['EntityName'] }}")]
    async fn create_{{ entity["entity_name"] }}<'ctx>(
        &self,
        context: &Context<'ctx>,
        #[graphql(desc = "{{ entity['EntityName'] }} Input")] {{ entity["entity_name"] }}: graphql::{{ entity["EntityName"] }}Input,
    ) -> Result<graphql::{{ entity["EntityName"] }}> {
        let mut {{ application["project_name"] }} = context.data::<{{ ProjectName }}Core>()?.clone().{{ application["project_name"] }}();
        let {{ entity["entity_name"] }} = {{ application["project_name"] }}
            .create_{{ entity["entity_name"] }}(proto::{{ entity["EntityName"] }}::convert_from({{ entity["entity_name"] }}))
            .await?
            .into_inner()
            .convert_to();
        Ok({{ entity["entity_name"] }})
    }

    #[graphql(name = "update{{ entity['EntityName'] }}")]
    async fn update_{{ entity["entity_name"] }}<'ctx>(
        &self,
        context: &Context<'ctx>,
        #[graphql(desc = "{{ entity['EntityName'] }} Input")] {{ entity["entity_name"] }}: graphql::{{ entity["EntityName"] }}Input,
    ) -> Result<graphql::{{ entity["EntityName"] }}> {
        let mut {{ application["project_name"] }} = context.data::<{{ ProjectName }}Core>()?.clone().{{ application["project_name"] }}();
        let {{ entity["entity_name"] }} = {{ application["project_name"] }}
            .update_{{ entity["entity_name"] }}(proto::{{ entity["EntityName"] }}::convert_from({{ entity["entity_name"] }}))
            .await?
            .into_inner()
            .convert_to();
        Ok({{ entity["entity_name"] }})
    }
    {% endfor %}
    {%- endfor %}
    /// Create Echo
    async fn create_echo(&self, #[graphql(desc = "Echo Message")] message: String) -> String {
        message
    }
}

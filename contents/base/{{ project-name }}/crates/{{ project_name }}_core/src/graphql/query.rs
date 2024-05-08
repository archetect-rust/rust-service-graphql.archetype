use async_graphql::{Context, ID, Object, Result};

use crate::{ConvertTo, {{ ProjectName }}Core, graphql, proto};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
    {%- for entity_key in application.model.entities %}
    {%- set entity = application.model.entities[entity_key] %}
    /// Get {{ entity["EntityName"] }} by ID
    #[graphql(name = "{{ entity['entityName'] }}")]
    async fn {{ entity["entity_name"] }}<'ctx>(
            &self,
            context: &Context<'ctx>,
            #[graphql(desc = "{{ entity['entityName'] }} Id")] id: ID,
    ) -> Result<graphql::{{ entity["EntityName"] }}> {
        let mut {{ application["project_name"] }} = context.data::<{{ ProjectName }}Core>()?.clone().{{ application["project_name"] }}();
        let result = {{ application["project_name"] }}
            .get_{{ entity["entity_name"] }}(proto::Get{{ entity["EntityName"] }}Request { id: id.0, })
            .await?
            .into_inner()
            .convert_to()
        ;
        Ok(result)
    }

    /// Get {{ entity["EntityName"] | pluralize }}
    #[graphql(name = "{{ entity['entityName'] | pluralize }}")]
    async fn {{ entity['entity_name'] | pluralize }}<'ctx>(
        &self,
        context: &Context<'ctx>,
        #[graphql(desc = "Page Index")] page_index: u32,
        #[graphql(desc = "Page Size")] page_size: u32,
    ) -> Result<graphql::{{ entity["EntityName"] | pluralize }}Page> {
        let mut {{ application["project_name"] }} = context.data::<{{ ProjectName }}Core>()?.clone().{{ application["project_name"] }}();
        let result = {{ application["project_name"] }}
            .get_{{ entity["entity_name"] |pluralize }}(proto::Get{{ entity["EntityName"] | pluralize }}Request { page_index, page_size })
            .await?
            .into_inner()
            .convert_to()
        ;
        Ok(result)
    }
    {% endfor %}
    {%- endfor %}
    /// Echo
    async fn echo(&self, #[graphql(desc = "Echo Message")] message: String) -> String {
        message
    }
}
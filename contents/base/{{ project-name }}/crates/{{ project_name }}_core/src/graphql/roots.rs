use async_graphql::{Context, ID, Object, Result};
use crate::{ConvertTo, {{ ProjectName }}Core, graphql, proto};



pub struct QueryRoot;

#[Object]
impl QueryRoot {
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
    {%- for entity_key in application.model.entities %}
    {%- set entity = application.model.entities[entity_key] %}
    async fn {{ entity["entity_name"] }}<'ctx>(&self, context: &Context<'ctx>, id: ID) -> Result<graphql::{{ entity["EntityName"] }}> {
        let mut {{ application["project_name"] }} = context.data::<{{ ProjectName }}Core>()?
            .clone().{{ application["project_name"] }}();
        let result = {{ application["project_name"] }}.get_{{ entity["entity_name"] }}(proto::Get{{ entity["EntityName"] }}Request {
            id: id.0,
        }).await?.into_inner();
        let result = result.convert_to()?;
        Ok(result)
    }

    {% endfor %}
    {%- endfor %}

    /// Get Simple Value
    async fn value(&self, #[graphql(desc = "Id of object")] id: i64) -> String {
        id.to_string()
    }

    /// Get Carts
    async fn carts(&self) -> Vec<graphql::Cart> {
        let mut carts = vec![];
        carts.push(graphql::Cart {
            id: ID::from("1"),
            contents: "One".to_string(),
        });
        carts.push(graphql::Cart {
            id: ID::from("2"),
            contents: "Two".to_string(),
        });
        carts.push(graphql::Cart {
            id: ID::from("3"),
            contents: "Three".to_string(),
        });
        carts
    }

    /// Expand a bunch of objects
    async fn my_objects(&self, #[graphql(desc = "Count of Objects")] count: i32) -> Vec<graphql::MyObj> {
        let mut results = vec![];
        for i in 0..count {
            results.push(graphql::MyObj { a: i, b: i });
        }
        results
    }

    /// Get MyObj
    #[graphql(name = "myObject")]
    async fn my_object(&self, a: i32, b: i32) -> graphql::MyObj {
       graphql::MyObj { a, b }
    }
}
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SchemaBuilder};
use crate::graphql::QueryRoot;


pub fn create_schema() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
}

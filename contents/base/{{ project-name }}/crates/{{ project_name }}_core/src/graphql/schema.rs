use async_graphql::{EmptySubscription, Schema, SchemaBuilder};

use crate::graphql::{MutationRoot, QueryRoot};

pub fn create_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}

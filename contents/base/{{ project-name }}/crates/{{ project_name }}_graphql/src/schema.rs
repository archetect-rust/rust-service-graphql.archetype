use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::model::roots::QueryRoot;

pub fn create_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish()
}


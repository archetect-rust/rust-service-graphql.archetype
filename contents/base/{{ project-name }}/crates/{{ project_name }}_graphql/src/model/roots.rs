use async_graphql::{ID, Object};
use crate::model::types::cart::{Cart};
use crate::model::types::my_obj::{MyObj};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get Simple Value
    async fn value(&self, #[graphql(desc = "Id of object")] id: i64) -> String {
        id.to_string()
    }

    /// Get Carts
    async fn carts(&self) -> Vec<Cart> {
        let mut carts = vec![];
        carts.push(Cart {
            id: ID::from("1"),
            contents: "One".to_string(),
        });
        carts.push(Cart {
            id: ID::from("2"),
            contents: "Two".to_string(),
        });
        carts.push(Cart {
            id: ID::from("3"),
            contents: "Three".to_string(),
        });
        carts
    }

    /// Expand a bunch of objects
    async fn my_objects(&self, #[graphql(desc = "Count of Objects")] count: i32) -> Vec<MyObj> {
        let mut results = vec![];
        for i in 0..count {
            results.push(MyObj { a: i, b: i });
        }
        results
    }

    /// Get MyObj
    #[graphql(name = "myObject")]
    async fn my_object(&self, a: i32, b: i32) -> MyObj {
       MyObj { a, b }
    }
}
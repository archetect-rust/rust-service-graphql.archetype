use async_graphql::{Description, ID, Object};

/// Shopping Cart
#[derive(Description)]
pub struct Cart {
    pub id: ID,
    pub contents: String,
}

#[Object(use_type_description)]
impl Cart {
    /// Get ID
    async fn id(&self) -> ID {
        self.id.clone()
    }

    async fn contents(&self) -> &String {
        &self.contents
    }

    #[graphql(name = "myObject")]
    async fn my_object(
        &self,
        #[graphql(desc = "Id of object", default = 5)] a: i32,
        #[graphql(desc = "Id of object", default = 5)] b: i32,
    ) -> crate::graphql::MyObj {
        crate::graphql::MyObj { a, b }
    }
}
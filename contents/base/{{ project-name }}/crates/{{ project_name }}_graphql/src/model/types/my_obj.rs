use async_graphql::{ComplexObject, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
pub struct MyObj {
    pub a: i32,
    pub b: i32,
}

#[ComplexObject]
impl MyObj {
    async fn c(&self) -> i32 {
        self.a + self.b
    }
}


use crate::{Node, SchemaNodeTypes, ID};
use async_graphql::{ComplexObject, SimpleObject};
use async_graphql_relay::RelayContext;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub role: String,
}

impl User {
    pub async fn get(ctx: RelayContext, id: String) -> Option<Node> {
        let ctx_str = ctx.get::<String>().unwrap();
        println!("Getting User: {} with context {}", id, ctx_str);

        Some(
            User {
                id: ID(id, SchemaNodeTypes::User),
                name: "Oscar".to_string(),
                role: "Testing123".to_string(),
            }
            .into(),
        )
    }
}

#[ComplexObject]
impl User {
    pub async fn test(&self) -> String {
        "testing".to_string()
    }
}

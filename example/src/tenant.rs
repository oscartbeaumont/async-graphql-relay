use crate::{Node, SchemaNodeTypes, ID};
use async_graphql::SimpleObject;
use async_graphql_relay::RelayContext;

#[derive(SimpleObject)]
pub struct Tenant {
    pub id: ID,
    pub name: String,
    pub description: String,
}

impl Tenant {
    pub async fn get(ctx: RelayContext, id: String) -> Option<Node> {
        let ctx_str = ctx.get::<String>().unwrap();
        println!("Getting Tenant: {} with context {}", id, ctx_str);

        Some(
            Tenant {
                id: ID(id, SchemaNodeTypes::Tenant),
                name: "My Company".to_string(),
                description: "Testing123".to_string(),
            }
            .into(),
        )
    }
}

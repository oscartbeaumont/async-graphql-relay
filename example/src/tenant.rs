use crate::{Node, SchemaNodeTypes, ID};
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Tenant {
    pub id: ID,
    pub name: String,
    pub description: String,
}

impl Tenant {
    pub async fn get(id: String) -> Option<Node> {
        println!("Getting Tenant: {}", id);

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

use async_graphql::{Error, SimpleObject};
use async_graphql_relay::{RelayContext, RelayNode, RelayNodeID, RelayNodeObject};

use crate::Node;

#[derive(Debug, SimpleObject, RelayNodeObject)]
#[relay(node_suffix = "t")]
pub struct Tenant {
    pub id: RelayNodeID<Self>,
    pub name: String,
    pub description: String,
}

impl RelayNode for Tenant {
    type TNode = Node;

    async fn get(ctx: RelayContext, id: RelayNodeID<Self>) -> Result<Option<Self::TNode>, Error> {
        let ctx_str = ctx.get::<String>().unwrap();
        println!("Getting Tenant: {:?} with context {}", id, ctx_str);

        Ok(Some(
            Tenant {
                id: RelayNodeID::new_from_str("92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff").unwrap(),
                name: "My Company".to_string(),
                description: "Testing123".to_string(),
            }
            .into(),
        ))
    }
}

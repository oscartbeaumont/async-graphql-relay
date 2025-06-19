use async_graphql::{ComplexObject, Error, SimpleObject};
use async_graphql_relay::{RelayContext, RelayNode, RelayNodeID, RelayNodeObject};

use crate::Node;

#[derive(Debug, SimpleObject, RelayNodeObject)]
#[graphql(complex)]
#[relay(node_suffix = "u")]
pub struct User {
    pub id: RelayNodeID<Self>,
    pub name: String,
    pub role: String,
}

impl RelayNode for User {
    type TNode = Node;

    async fn get(ctx: RelayContext, id: RelayNodeID<Self>) -> Result<Option<Self::TNode>, Error> {
        let ctx_str = ctx.get::<String>().unwrap();
        println!("Getting User: {:?} with context {}", id, ctx_str);

        Ok(Some(
            User {
                id: RelayNodeID::new_from_str("92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff").unwrap(),
                name: "Oscar".to_string(),
                role: "Testing123".to_string(),
            }
            .into(),
        ))
    }
}

#[ComplexObject]
impl User {
    pub async fn test(&self) -> String {
        "testing".to_string()
    }
}

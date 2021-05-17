use crate::Node;
use crate::SchemaNodeTypes;
use async_graphql::{ComplexObject, SimpleObject};
use async_graphql_relay::RelayGlobalID;

#[derive(SimpleObject, RelayGlobalID)]
#[graphql(complex)]
pub struct Tenant {
    #[graphql(skip)]
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Tenant {
    pub async fn get(id: String) -> Node {
        Tenant {
            id: id,
            name: "My Company".to_string(),
            description: "Testing123".to_string(),
        }
        .into()
    }
}

#[ComplexObject]
impl Tenant {
    pub async fn id(&self) -> String {
        self.relay_id()
    }
}

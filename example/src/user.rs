use crate::Node;
use crate::SchemaNodeTypes;
use async_graphql::{ComplexObject, SimpleObject};
use async_graphql_relay::RelayGlobalID;

#[derive(SimpleObject, RelayGlobalID)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: String,
    pub name: String,
    pub role: String,
}

impl User {
    pub async fn get(id: String) -> Node {
        User {
            id: id,
            name: "Oscar".to_string(),
            role: "Testing123".to_string(),
        }
        .into()
    }
}

#[ComplexObject]
impl User {
    pub async fn id(&self) -> String {
        self.relay_id()
    }

    pub async fn test(&self) -> String {
        "testing".to_string()
    }
}

use crate::{Node, SchemaNodeTypes, ID};
use async_graphql::{ComplexObject, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub role: String,
}

impl User {
    pub async fn get(id: String) -> Option<Node> {
        println!("Getting User: {}", id);

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

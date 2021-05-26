use async_graphql::{value, EmptyMutation, EmptySubscription, Interface, Object, SimpleObject};
use async_graphql_relay::{RelayContext, RelayGlobalID, RelayNodeEnum};

#[derive(RelayGlobalID)]
pub struct ID(pub String, pub SchemaNodeTypes);

#[derive(Interface, RelayNodeEnum)]
#[graphql(field(name = "id", type = "String"))]
pub enum Node {
    User(User),
    Tenant(Tenant),
}

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
}

impl User {
    pub async fn get(ctx: RelayContext, id: String) -> Option<Node> {
        let ctx_str = ctx.get::<String>().unwrap();
        if id != "92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff".to_string()
            || *ctx_str != "ThisIsInTheContext".to_string()
        {
            None?
        }

        Some(
            User {
                id: ID(id, SchemaNodeTypes::User),
                name: "Oscar".to_string(),
            }
            .into(),
        )
    }
}

#[derive(SimpleObject)]
pub struct Tenant {
    pub id: ID,
    pub description: String,
}

impl Tenant {
    pub async fn get(ctx: RelayContext, id: String) -> Option<Node> {
        let ctx_str = ctx.get::<String>().unwrap();
        if id != "14b4a5db-b8f0-4bf9-881e-37a9e0d0ae3h".to_string()
            || *ctx_str != "ThisIsInTheContext".to_string()
        {
            None?
        }

        Some(
            Tenant {
                id: ID(id, SchemaNodeTypes::Tenant),
                description: "My Company".to_string(),
            }
            .into(),
        )
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self) -> User {
        User {
            id: ID(
                "92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff".to_string(),
                SchemaNodeTypes::User,
            ),
            name: "Oscar".to_string(),
        }
    }

    async fn tenant(&self) -> Tenant {
        Tenant {
            id: ID(
                "14b4a5db-b8f0-4bf9-881e-37a9e0d0ae3h".to_string(),
                SchemaNodeTypes::Tenant,
            ),
            description: "My Company".to_string(),
        }
    }

    async fn node(&self, id: String) -> Option<Node> {
        let ctx = RelayContext::new::<String>("ThisIsInTheContext".to_string());
        Node::get(ctx, id).await
    }
}

fn schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation, EmptySubscription)
}

#[tokio::test]
async fn test_user_query() {
    let query = "{ user { id, name } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "user": {
                "id": "92ba0c2d4b4e4e2991dd8f96a078c3ff1",
                "name": "Oscar",
            },
        })
    );
}

#[tokio::test]
async fn test_user_node_query() {
    let query = "{ node(id: \"92ba0c2d4b4e4e2991dd8f96a078c3ff1\") { id, ... on User { name } } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "node": {
                "id": "92ba0c2d4b4e4e2991dd8f96a078c3ff1",
                "name": "Oscar",
            },
        })
    );
}

#[tokio::test]
async fn test_tenant_query() {
    let query = "{ tenant { id, description } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "tenant": {
                "id": "14b4a5dbb8f04bf9881e37a9e0d0ae3h2",
                "description": "My Company",
            },
        })
    );
}

#[tokio::test]
async fn test_tenant_node_query() {
    let query =
        "{ node(id: \"14b4a5dbb8f04bf9881e37a9e0d0ae3h2\") { id, ... on Tenant { description } } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "node": {
                "id": "14b4a5dbb8f04bf9881e37a9e0d0ae3h2",
                "description": "My Company",
            },
        })
    );
}

#[tokio::test]
async fn test_tenant_invalid_short_relay_id() {
    let query = "{ node(id: \"invalid\") { id } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "node": null,
        })
    );
}

#[tokio::test]
async fn test_tenant_invalid_long_relay_id() {
    let query =
        "{ node(id: \"2b0669af44fb4949bed4d7786cc0164b2b0669af44fb4949bed4d7786cc0164b\") { id } }";
    assert_eq!(
        schema().execute(query).await.data,
        value!({
            "node": null,
        })
    );
}

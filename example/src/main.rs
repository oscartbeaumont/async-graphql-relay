use crate::tenant::Tenant;
use crate::user::User;
use actix_web::guard;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Error, Interface, Object, ScalarType};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql_relay::{
    RelayContext, RelayInterface, RelayNode, RelayNodeID, RelayNodeInterface,
};

mod tenant;
mod user;

pub struct QueryRoot;

#[derive(Interface, RelayInterface)]
#[graphql(field(name = "id", type = "NodeGlobalID"))] // The 'NodeGlobalID' type comes from the 'RelayInterface' macro.
pub enum Node {
    User(User),
    Tenant(Tenant),
}

#[Object]
impl QueryRoot {
    async fn user(&self) -> User {
        User {
            id: RelayNodeID::new_from_str("92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff").unwrap(),
            name: "Oscar".to_string(),
            role: "Testing123".to_string(),
        }
    }

    async fn tenant(&self) -> Tenant {
        Tenant {
            id: RelayNodeID::new_from_str("4e02ec03-f82f-46da-8572-39975bf97d9d").unwrap(),
            name: "My Company".to_string(),
            description: "Testing123".to_string(),
        }
    }

    async fn node(
        &self,
        #[graphql(validator(min_length = 33, max_length = 33))] id: String, // Ensure the length's of the longest 'node_suffix' plus 32 is validated.
    ) -> Result<Node, Error> {
        let ctx = RelayContext::new::<String>("Hello World".to_string()); // This could include your database connection and/or any other context required in your implementations of the 'RelayNode' trait.
        Node::fetch_node(ctx, id).await
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn handler(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Listening http://localhost:8080/ ...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(handler))
            .service(web::resource("/").guard(guard::Get()).to(playground))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

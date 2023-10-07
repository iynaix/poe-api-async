use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

mod schema;

use schema::QueryRoot;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        // .data(Person::new())
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    let port = 3000;

    println!("GraphiQL IDE: http://localhost:{}", port);

    Server::bind(&format!("127.0.0.1:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

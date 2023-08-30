use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING ON {address}\n");

    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service())
        .await
        .expect("server failed");
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// hello?name=Emilio
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// hello/Emilio
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

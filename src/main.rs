use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};
mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LIS TENING ON {address}\n");

    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service())
        .await
        .expect("server failed");
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<15} - main_response_mapper", "RES_MAPPER");

    println!("");
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// hello?name=Emilio
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<15} - handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// hello/Emilio
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<15} - handler_hello2 {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

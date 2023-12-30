use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

use crate::model::ModelController;

use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let state = ModelController::new().await?;

    let routes = Router::new()
        .merge(hello_routes())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(state.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:       ---START SERVER
    let address = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("->> LISTENNING ON {address}\n");

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
    // endregion:   --- START SERVER

    Ok(())
}

async fn main_response_mapper(response: Response) -> Response {
    println!("->> {:<15} - main_reponse_mapper", "RESPONSE MAPPER");

    println!();
    response
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:          ---Routes hello
fn hello_routes() -> Router {
    Router::new()
        .route("/hello-query", get(hello_query_handler))
        .route("/hello-path/:name", get(hello_path_handler))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g. /hello-query?name=Emilio
async fn hello_query_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<15} - handler hello by query", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. /hello-path/Emilio
async fn hello_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<15} - handler hello by path", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion:       ---Routes hello

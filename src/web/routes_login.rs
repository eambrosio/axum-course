use axum::{
    body,
    routing::{post, Route},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<15} - api_login", "HANDLER");

    // TODO implement auth logic
    if payload.username != "test" || payload.pwd != "test" {
        return Err(Error::LoginFail);
    }

    // TODO set cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

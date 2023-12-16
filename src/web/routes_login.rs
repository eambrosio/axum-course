use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<20} - api_login", "HANDLER");

    //TODO: implement real db/auth login
    if payload.username == "error" || payload.password == "error" {
        return Err(Error::UnexpectedError);
    } else if payload.username != "test" || payload.password != "test" {
        return Err(Error::LoginError);
    }

    //TODO: set cookies

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
    password: String,
}

use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub enum Error {
    LoginError,
    UnexpectedError,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<15} - {self:?}", "INTO RESPONSE");

        match self {
            Error::LoginError => (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR").into_response(),
            Error::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_ERROR").into_response()
            }
        }
    }
}

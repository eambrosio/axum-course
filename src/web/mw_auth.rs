use axum::{extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::error::Result;

use crate::{error::Error, web::AUTH_TOKEN};

pub async fn md_required_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<15} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //TODO Real auth-token parsing & validation
    auth_token.ok_or(Error::AuthFailedNoTokenCookie)?;

    Ok(next.run(req).await)
}

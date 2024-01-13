use axum::{extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::error::Result;

use crate::{error::Error, web::AUTH_TOKEN};

pub async fn md_required_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<15} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //TODO Real auth-token parsing & validation
    let (_user_id, _exp, _sig): (u64, String, String) = auth_token
        .ok_or(Error::AuthFailedNoTokenCookie)
        .and_then(parse_token)?;

    //TODO TOken components validation

    Ok(next.run(req).await)
}

// Parse token of format 'user-[USER_ID]-[EXPIRATION].[SIGNATURE]'
pub fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sig) = regex_captures!(
        r#"^user-(\d+)\.(.+).(.+)"#, //regexp
        &token
    )
    .ok_or(Error::AuthFailedWrongTokenFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailedWrongTokenFormat)?;

    Ok((user_id, exp.to_string(), sig.to_string()))
}

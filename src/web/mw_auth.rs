use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum::{extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::error::Result;

use crate::{error::Error, web::AUTH_TOKEN};

pub async fn md_required_auth(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<15} - mw_require_auth {ctx:?}", "MIDDLEWARE");

    // ctx?;

    Ok(next.run(req).await)
}

// region:
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        //Uses the cookies extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        //TODO Real auth-token parsing & validation
        let (user_id, _exp, _sig): (u64, String, String) = auth_token
            .ok_or(Error::AuthFailedNoTokenCookie)
            .and_then(parse_token)?;

        //TODO TOken components validation

        Ok(Ctx::new(user_id))
    }
}
// endregion:

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

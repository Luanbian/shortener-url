use axum::body::Body;
use axum::extract::Request;
use axum::http::Response;
use axum::middleware::Next;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn middleware_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>> {
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    auth_token.ok_or(Error::AuthFailNoTokenProvided)?;
    Ok(next.run(req).await)
}

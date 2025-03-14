use axum::body::Body;
use axum::extract::Request;
use axum::http::Response;
use axum::middleware::Next;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::rest,
    IResult,
};
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

    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoTokenProvided)
        .and_then(parse_token)?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    match parser(&token) {
        Ok((_, result)) => Ok(result),
        Err(_) => Err(Error::AuthTokenWrongFormat),
    }
}

fn parser(input: &str) -> IResult<&str, (u64, String, String)> {
    let (input, _) = tag("user-")(input)?;

    let (input, user_id) = digit1(input)?;
    let user_id = user_id.parse::<u64>().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    let (input, _) = tag(".")(input)?;
    let (input, exp) = take_until(".")(input)?;

    let (input, _) = tag(".")(input)?;
    let (input, sign) = rest(input)?;

    Ok((input, (user_id, exp.to_string(), sign.to_string())))
}

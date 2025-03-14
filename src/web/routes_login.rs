use crate::web;
use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("{:<12} - api_login", "HANDLER LOGIN");

    // TODO: implement real db/auth logic
    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    // TODO implement real auth-token signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body: Json<Value> = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}

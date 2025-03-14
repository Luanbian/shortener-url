pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    let address_server: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on port {address_server}\n");

    let listener: TcpListener = TcpListener::bind(&address_server).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("{:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name="Luan"
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("{:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

// /hello/Luan
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("{:<12} - handler_hello2 - {name:?}", "HANDLER2");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

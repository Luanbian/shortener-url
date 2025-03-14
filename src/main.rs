use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_all: Router = Router::new().merge(routes_hello());
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

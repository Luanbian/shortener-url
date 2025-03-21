pub use self::error::{Error, Result};

use axum::response::Response;
use axum::{middleware, Router};
use model::ModelController;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod constants;
mod error;
mod features;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    constants::postgres::load_env().await;

    let pool = connect_database().await?;

    let model_controller = ModelController::new(pool.clone()).await?;

    let routes_apis = web::routes_shortener::routes(model_controller.clone());

    let routes_all: Router = Router::new()
        .merge(web::routes_login::routes())
        .merge(web::routes_redirect::routes(model_controller.clone()))
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    let address_server: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on port {address_server}\n");

    let listener: TcpListener = TcpListener::bind(&address_server).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    res
}

async fn connect_database() -> Result<sqlx::PgPool> {
    let db_connection_str = &constants::postgres::get_postgres_url().await;
    match sqlx::PgPool::connect(db_connection_str).await {
        Ok(pool) => {
            println!("Conectado ao banco de dados");
            Ok(pool)
        }
        Err(e) => {
            eprintln!("Erro ao conectar ao banco de dados: {:?}", e);
            Err(e.into())
        }
    }
}

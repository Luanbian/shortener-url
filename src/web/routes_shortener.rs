use crate::model::{ModelController, Shortener, ShortenerForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::{
    routing::{delete, post},
    Json, Router,
};

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_tickets))
        .with_state(model_controller)
}

async fn create_ticket(
    State(model_controller): State<ModelController>,
    Json(short_payload): Json<ShortenerForCreate>,
) -> Result<Json<Shortener>> {
    println!("{:<12} - create_ticket", "HANDLER");

    let shortener = model_controller.create_short_link(short_payload).await?;

    Ok(Json(shortener))
}

async fn list_tickets(
    State(model_controller): State<ModelController>,
) -> Result<Json<Vec<Shortener>>> {
    println!("{:<12} - list_tickets", "HANDLER");

    let short_links = model_controller.list_short_links().await?;

    Ok(Json(short_links))
}

async fn delete_tickets(
    State(model_controller): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Shortener>> {
    println!("{:<12} - list_tickets", "HANDLER");

    let shortener = model_controller.delete_short_link(id).await?;

    Ok(Json(shortener))
}

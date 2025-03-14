use crate::model::{ModelController, Ticket, TicketForCreate};
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
    Json(ticket_payload): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("{:<12} - create_ticket", "HANDLER");

    let ticket = model_controller.create_ticket(ticket_payload).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(model_controller): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("{:<12} - list_tickets", "HANDLER");

    let tickets = model_controller.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_tickets(
    State(model_controller): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("{:<12} - list_tickets", "HANDLER");

    let ticket = model_controller.delete_ticket(id).await?;

    Ok(Json(ticket))
}

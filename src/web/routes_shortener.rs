use crate::features::UrlShortener;
use crate::model::{ModelController, Shortener, ShortenerForCreateDb};
use crate::Result;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ShortenerCreateRequest {
    pub user_id: Uuid,
    pub original_url: String,
}

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/shortener", post(create_short_url))
        .with_state(model_controller)
}

async fn create_short_url(
    State(model_controller): State<ModelController>,
    Json(short_payload): Json<ShortenerCreateRequest>,
) -> Result<Json<Shortener>> {
    println!("{:<12} - create_ticket", "HANDLER");

    let mut shorter_url = UrlShortener::new();
    let short_url = shorter_url.shorten_url(&short_payload.original_url);

    println!("code: {short_url}");
    println!("long_url: {:?} ", short_payload.original_url);

    let short_payload_db = ShortenerForCreateDb {
        user_id: short_payload.user_id,
        original_url: short_payload.original_url,
        short_url,
    };

    let shortener = model_controller.create_short_link(short_payload_db).await?;

    Ok(Json(shortener))
}

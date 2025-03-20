use crate::features::UrlShortener;
use crate::model::{ModelController, ShortenerForCreateDb};
use crate::Result;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ShortenerCreateRequest {
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
) -> Result<String> {
    let mut shorter_url = UrlShortener::new();
    let short_url = shorter_url.shorten_url(&short_payload.original_url);

    println!("short_url: http://localhost:8080/{short_url}");

    let short_payload_db = ShortenerForCreateDb {
        user_id: Uuid::new_v4(),
        original_url: short_payload.original_url,
        short_url,
    };

    let shortener = model_controller.create_short_link(short_payload_db).await?;

    let shorted_url = format!("http:localhost:8080/{}", shortener.short_url.to_string());
    Ok(shorted_url)
}

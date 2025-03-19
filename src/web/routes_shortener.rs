use crate::features::UrlShortener;
use crate::model::{ModelController, Shortener, ShortenerCreateRequest, ShortenerForCreateDb};
use crate::Result;
use axum::extract::State;
use axum::{routing::post, Json, Router};

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket))
        .with_state(model_controller)
}

async fn create_ticket(
    State(model_controller): State<ModelController>,
    Json(short_payload): Json<ShortenerCreateRequest>,
) -> Result<Json<Shortener>> {
    println!("{:<12} - create_ticket", "HANDLER");

    let mut shorter_url = UrlShortener::new("http://localhost:8080");
    let short_url = shorter_url.shorten_url(&short_payload.original_url);

    println!("short_url: {short_url}");
    println!("long_url: {:?} ", short_payload.original_url);

    let short_payload_db = ShortenerForCreateDb {
        user_id: short_payload.user_id,
        original_url: short_payload.original_url,
        short_url,
    };

    let shortener = model_controller.create_short_link(short_payload_db).await?;

    Ok(Json(shortener))
}

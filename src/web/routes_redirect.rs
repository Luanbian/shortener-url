use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::model::ModelController;

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/{short_code}", get(redirect))
        .with_state(model_controller)
}

async fn redirect(
    Path(short_code): Path<String>,
    State(model_controller): State<ModelController>,
) -> impl IntoResponse {
    match model_controller.get_original_url(&short_code).await {
        Ok(Some(original_url)) => {
            (StatusCode::FOUND, [(header::LOCATION, original_url)]).into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

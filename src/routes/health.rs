use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

/// Обработчик GET /health
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
    })
}

/// Роутер для health
pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_handler))
}

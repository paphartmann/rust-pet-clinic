use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
    })
}

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_handler))
}

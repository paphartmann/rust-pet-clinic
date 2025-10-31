use axum::Json;
use serde::Serialize;

use axum::{Router, routing::get};
use crate::services::hello_service::GreetingService;

/// DTO (ответ API)
#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    let service = GreetingService;
    let greeting = service.say_hello();

    // Handler адаптирует доменную сущность под формат HTTP-ответа
    let response = HelloResponse {
        message: greeting.message,
    };

    Json(response)
}

pub fn hello_routes() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

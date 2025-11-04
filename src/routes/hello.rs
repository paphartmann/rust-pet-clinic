use axum::Json;
use serde::Serialize;

use crate::services::hello_service::GreetingService;
use axum::{Router, routing::get};

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    let service = GreetingService;
    let greeting = service.say_hello();

    let response = HelloResponse {
        message: greeting.message,
    };

    Json(response)
}

pub fn hello_routes() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

use axum::Router;

mod hello;
mod health;
mod vet;

pub use hello::hello_routes;
pub use health::health_routes;
use crate::services::vet_service::VetService;

/// Собирает все маршруты приложения
pub fn create_routes(vet_service: VetService) -> Router {
    Router::new()
        .merge(hello_routes())
        .merge(health_routes())
        .merge(vet::vet_routes(vet_service))
}

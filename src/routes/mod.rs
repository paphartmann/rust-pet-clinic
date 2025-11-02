use axum::Router;

mod health;
mod hello;
mod vet;
mod visit;

use crate::services::vet_service::VetService;
use crate::services::visit_service::VisitService;
pub use health::health_routes;
pub use hello::hello_routes;

/// Собирает все маршруты приложения
pub fn create_routes(vet_service: VetService, visit_service: VisitService) -> Router {
    Router::new()
        .merge(hello_routes())
        .merge(health_routes())
        .merge(vet::vet_routes(vet_service))
        .merge(visit::visit_routes(visit_service))
}

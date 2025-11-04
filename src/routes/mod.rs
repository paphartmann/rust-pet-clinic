use axum::Router;

mod health;
mod hello;
mod vet;
mod visit;
mod owner;

use crate::services::owner_service::OwnerService;
use crate::services::vet_service::VetService;
use crate::services::visit_service::VisitService;
pub use health::health_routes;
pub use hello::hello_routes;

pub fn create_routes(
    vet_service: VetService,
    visit_service: VisitService,
    owner_service: OwnerService,
) -> Router {
    Router::new()
        .merge(hello_routes())
        .merge(health_routes())
        .merge(vet::vet_routes(vet_service))
        .merge(visit::visit_routes(visit_service))
        .merge(owner::owner_routes(owner_service))
}

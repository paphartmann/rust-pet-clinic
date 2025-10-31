use axum::{Router, routing::get};
use crate::handlers::vet_handler::get_vets_handler;
use crate::services::vet_service::VetService;

pub fn vet_routes(service: VetService) -> Router {
    Router::new()
        .route("/vets", get(get_vets_handler))
        .with_state(service)
}

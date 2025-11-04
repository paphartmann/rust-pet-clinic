use axum::{Router, routing::get};
use axum::routing::post;
use crate::handlers::visit_handler::{add_visit_to_pet_handler, visit_by_pet_handler};
use crate::services::visit_service::VisitService;

pub fn visit_routes(service: VisitService) -> Router {
    Router::new()
        .route("/pet/{pet_id}/visit", get(visit_by_pet_handler))
        .route("/pet/{pet_id}/visit", post(add_visit_to_pet_handler))
        .with_state(service)
}

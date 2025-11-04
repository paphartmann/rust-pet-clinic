use crate::handlers::visit_handler::{add_visit_to_pet_handler, visits_by_pet_handler};
use crate::services::visit_service::VisitService;
use axum::routing::post;
use axum::{Router, routing::get};

pub fn visit_routes(service: VisitService) -> Router {
    Router::new()
        .route("/pet/{pet_id}/visit", get(visits_by_pet_handler))
        .route("/pet/{pet_id}/visit", post(add_visit_to_pet_handler))
        .with_state(service)
}

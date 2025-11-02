use axum::{Router, routing::get};
use crate::handlers::visit_handler::get_visits_handler;
use crate::services::visit_service::VisitService;

pub fn visit_routes(service: VisitService) -> Router {
    Router::new()
        .route("/pet/{pet_id}/visit", get(get_visits_handler))
        .with_state(service)
}

use crate::handlers::owner_handler::get_owner_handler;
use crate::services::owner_service::OwnerService;
use axum::{Router, routing::get};

pub fn owner_routes(service: OwnerService) -> Router {
    Router::new()
        .route("/owner/{owner_id}", get(get_owner_handler))
        .with_state(service)
}

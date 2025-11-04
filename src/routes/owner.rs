use crate::handlers::owner_handler::{add_pet_to_owner_handler, get_owner_handler};
use crate::services::owner_service::OwnerService;
use axum::routing::post;
use axum::{Router, routing::get};

pub fn owner_routes(service: OwnerService) -> Router {
    Router::new()
        .route("/owner/{owner_id}", get(get_owner_handler))
        .route("/owner/{owner_id}/pet", post(add_pet_to_owner_handler))
        .with_state(service)
}

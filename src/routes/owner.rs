use crate::handlers::owner_handler::{
    add_pet_to_owner_handler, create_owner_handler, get_owner_handler, update_owner_pet_handler,
};
use crate::services::owner_service::OwnerService;
use axum::routing::{post, put};
use axum::{Router, routing::get};

pub fn owner_routes(service: OwnerService) -> Router {
    Router::new()
        .route("/owner", post(create_owner_handler))
        .route("/owner/{owner_id}", get(get_owner_handler))
        .route("/owner/{owner_id}/pet", post(add_pet_to_owner_handler))
        .route(
            "/owner/{owner_id}/pet/{pet_id}",
            put(update_owner_pet_handler),
        )
        .with_state(service)
}

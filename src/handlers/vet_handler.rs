use crate::models::vet::Vet;
use crate::services::vet_service::VetService;
use axum::Json;
use axum::extract::State;
use serde::Serialize;

/// DTO для HTTP-ответа
#[derive(Serialize)]
pub struct VetResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

pub async fn get_vets_handler(State(service): State<VetService>) -> Json<Vec<VetResponse>> {
    let vets: Vec<Vet> = service
        .get_all_vets()
        .await
        .unwrap_or_default();

    let response: Vec<VetResponse> = vets
        .into_iter()
        .map(|v| VetResponse {
            id: v.id,
            first_name: v.first_name,
            last_name: v.last_name,
        })
        .collect();

    Json(response)
}

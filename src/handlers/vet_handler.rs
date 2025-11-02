use crate::models::vet::vet::{Vet, VetWithSpecialties};
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
    pub specialties: Vec<String>,
}

pub async fn get_vets_handler(State(service): State<VetService>) -> Json<Vec<VetResponse>> {
    let vets: Vec<VetWithSpecialties> = service.get_all_vets().await.unwrap_or_default();

    let response: Vec<VetResponse> = vets
        .into_iter()
        .map(|v| VetResponse {
            id: v.id,
            first_name: v.first_name,
            last_name: v.last_name,
            specialties: v.specialties
        })
        .collect();

    Json(response)
}

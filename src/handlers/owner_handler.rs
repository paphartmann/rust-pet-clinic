use crate::handlers::visit_handler::VisitResponse;
use crate::services::owner_service::OwnerService;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct OwnerResponse {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub pets: Vec<PetResponse>,
}

#[derive(Serialize)]
pub struct PetResponse {
    pub id: i32,
    pub name: Option<String>,
    pub pet_type: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub visits: Vec<VisitResponse>,
}

pub async fn get_owner_handler(
    Path(owner_id): Path<i32>,
    State(service): State<OwnerService>,
) -> Result<Json<OwnerResponse>, StatusCode> {
    let owner_option = service.get_owner(owner_id).await.unwrap();

    let owner = match owner_option {
        Some(owner) => owner,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let transformed = OwnerResponse {
        id: owner.id,
        first_name: owner.first_name,
        last_name: owner.last_name,
        address: owner.address,
        city: owner.city,
        phone: owner.phone,
        pets: owner.pets
            .into_iter()
            .map(|p| PetResponse {
                id: p.id,
                name: p.name,
                pet_type: p.pet_type,
                birth_date: p.birth_date,
                visits: p.visits
                    .into_iter()
                    .map(|v| VisitResponse {
                        id: v.id,
                        visit_date: v.visit_date,
                        description: v.description,
                    }).collect(),
            })
            .collect(),
    };


    Ok(Json(transformed))
}

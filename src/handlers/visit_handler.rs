use crate::models::owner::visit::VisitAdd;
use crate::services::visit_service::VisitService;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct VisitResponse {
    pub(crate) id: i32,
    pub(crate) visit_date: Option<NaiveDate>,
    pub(crate) description: Option<String>,
}

pub async fn visits_by_pet_handler(
    Path(pet_id): Path<i32>,
    State(service): State<VisitService>,
) -> Json<Vec<VisitResponse>> {
    let visits = service.get_visits(pet_id).await.unwrap();

    let transformed = visits
        .into_iter()
        .map(|v| VisitResponse {
            id: v.id,
            visit_date: v.visit_date,
            description: v.description,
        })
        .collect();

    Json(transformed)
}

#[derive(Deserialize)]
pub struct VisitAddRequest {
    pub(crate) visit_date: Option<NaiveDate>,
    pub(crate) description: Option<String>,
}

pub async fn add_visit_to_pet_handler(
    Path(pet_id): Path<i32>,
    State(service): State<VisitService>,
    Json(body): Json<VisitAddRequest>,
) -> Result<StatusCode, StatusCode> {
    service
        .add_visit(
            pet_id,
            VisitAdd {
                visit_date: body.visit_date,
                description: body.description,
            },
        )
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

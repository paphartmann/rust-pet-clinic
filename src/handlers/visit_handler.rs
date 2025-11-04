use crate::services::visit_service::VisitService;
use axum::Json;
use axum::extract::{Path, State};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct VisitResponse {
    pub(crate) id: i32,
    pub(crate) visit_date: Option<NaiveDate>,
    pub(crate) description: Option<String>,
}

pub async fn get_visits_handler(
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

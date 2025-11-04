use crate::models::owner::visit::{Visit, VisitAdd};
use sqlx::PgPool;

#[derive(Clone)]
pub struct VisitRepository {
    pool: PgPool,
}

impl VisitRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_visits(&self, pet_ids: &Vec<i32>) -> anyhow::Result<Vec<Visit>> {
        let rows = sqlx::query_as!(
            Visit,
            r#"
            SELECT id, visit_date, description, pet_id
            FROM visits
            WHERE pet_id = ANY($1)
            ORDER BY visit_date
            "#,
            pet_ids
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn add_visit(&self, pet_id: i32, visit: VisitAdd) -> anyhow::Result<u64> {
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO visits (pet_id, visit_date, description) VALUES ($1, $2, $3)
            "#,
            pet_id,
            visit.visit_date,
            visit.description
        )
        .execute(&self.pool)
        .await?;

        Ok(insert_result.rows_affected())
    }
}

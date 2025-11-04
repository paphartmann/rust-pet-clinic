use sqlx::PgPool;
use crate::models::owner::visit::Visit;

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
}

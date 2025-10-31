use crate::models::vet::Vet;
use sqlx::PgPool;

#[derive(Clone)]
pub struct VetRepository {
    pool: PgPool,
}

impl VetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Vet>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, first_name, last_name
            FROM vets
            ORDER BY last_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                Vet::new(
                    r.id,
                    r.first_name.unwrap_or_default(),
                    r.last_name.unwrap_or_default(),
                )
            })
            .collect())
    }
}

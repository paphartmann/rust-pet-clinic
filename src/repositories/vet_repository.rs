use crate::models::vet::vet::Vet;
use sqlx::PgPool;
use std::collections::HashMap;

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

    pub async fn get_all_vet_specialty(&self) -> anyhow::Result<HashMap<i32, Vec<String>>> {
        let rows = sqlx::query!(
            r#"
        SELECT vs.vet_id, s.name AS specialty
        FROM vet_specialties vs
        JOIN specialties s ON s.id = vs.specialty_id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut map: HashMap<i32, Vec<String>> = HashMap::new();

        for row in rows {
            map.entry(row.vet_id)
                .or_default()
                .push(row.specialty.unwrap_or_default());
        }

        Ok(map)
    }
}

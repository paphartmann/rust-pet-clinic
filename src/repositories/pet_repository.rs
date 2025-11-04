use sqlx::PgPool;
use crate::models::owner::pet::Pet;

#[derive(Clone)]
pub struct PetRepository {
    pool: PgPool,
}

impl PetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_pets_by_owner_id(&self, owner_id: &i32) -> anyhow::Result<Vec<Pet>> {
        let rows = sqlx::query_as!(
            Pet,
            r#"
            SELECT p.id, p.name, p.birth_date, t.name as pet_type
            FROM pets p INNER JOIN types t ON p.type_id = t.id
            WHERE owner_id = $1
            ORDER BY birth_date
            "#,
            owner_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}

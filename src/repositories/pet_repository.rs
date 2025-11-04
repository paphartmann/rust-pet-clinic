use crate::models::owner::pet::{Pet, PetAdd};
use sqlx::PgPool;

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

    pub async fn add_pet(&self, owner_id: i32, pet: PetAdd) -> anyhow::Result<i32> {
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO pets (owner_id, name, birth_date, type_id)
            VALUES ($1, $2, $3, (select id from types where name = $4))
            RETURNING id
            "#,
            owner_id,
            pet.name,
            pet.birth_date,
            pet.pet_type,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(insert_result.id)
    }

    pub async fn update_pet(&self, owner_id: i32, pet_id: i32, pet: PetAdd) -> anyhow::Result<u64> {
        let insert_result = sqlx::query!(
            r#"
            UPDATE pets set
                            name = $1,
                            birth_date = $2,
                            type_id =  (select id from types where name = $3)
            WHERE owner_id = $4 AND id = $5
            "#,
            pet.name,
            pet.birth_date,
            pet.pet_type,
            owner_id,
            pet_id
        )
        .execute(&self.pool)
        .await?;

        Ok(insert_result.rows_affected())
    }
}

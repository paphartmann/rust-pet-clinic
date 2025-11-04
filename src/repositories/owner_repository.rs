use crate::models::owner::owner::{Owner, OwnerAdd};
use crate::models::owner::pet::PetAdd;
use crate::models::owner::visit::Visit;
use sqlx::PgPool;

#[derive(Clone)]
pub struct OwnerRepository {
    pool: PgPool,
}

impl OwnerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_owner(&self, owner_id: i32) -> anyhow::Result<Option<Owner>> {
        let rows = sqlx::query_as!(
            Owner,
            r#"
            SELECT id, first_name, last_name, address, city, telephone as phone
            FROM owners
            WHERE id = $1
            "#,
            owner_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn add_owner(&self, owner: OwnerAdd) -> anyhow::Result<u64> {
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO owners (first_name, last_name, address, city, telephone)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            owner.first_name,
            owner.last_name,
            owner.address,
            owner.city,
            owner.phone
        )
        .execute(&self.pool)
        .await?;

        Ok(insert_result.rows_affected())
    }
}

use sqlx::PgPool;
use crate::models::owner::owner::Owner;
use crate::models::owner::visit::Visit;

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
}

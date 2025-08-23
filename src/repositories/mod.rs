use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::models::Entity;

pub mod user_repository;

#[async_trait]
pub trait Repository<T>
where
    T: Entity + Send + Sync + Unpin + for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow>,
{
    fn pool(&self) -> &SqlitePool;

    async fn find_by_id(&self, id: i64) -> Result<Option<T>, sqlx::Error> {
        sqlx::query_as::<_, T>(&format!("SELECT * FROM {} WHERE id = ?", T::TABLE,))
            .bind(id)
            .fetch_optional(self.pool())
            .await
    }

    async fn destroy(&self, id: i64) -> Result<(), sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?", T::TABLE);
        sqlx::query(&sql).bind(id).execute(self.pool()).await?;
        Ok(())
    }
}

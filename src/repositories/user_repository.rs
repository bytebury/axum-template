use sqlx::query_as;

use crate::{models::user::User, repositories::Repository};

pub struct UserRepository {
    db: sqlx::SqlitePool,
}

impl UserRepository {
    pub fn new(db: &sqlx::SqlitePool) -> Self {
        UserRepository { db: db.clone() }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE email = ?"#)
            .bind(email)
            .fetch_optional(&self.db)
            .await
    }

    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (email, verified, full_name, first_name, last_name, picture_url)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING *
            "#,
            user.email,
            user.verified,
            user.full_name,
            user.first_name,
            user.last_name,
            user.picture_url,
        )
        .fetch_one(&self.db)
        .await
    }
}

impl Repository<User> for UserRepository {
    fn pool(&self) -> &sqlx::SqlitePool {
        &self.db
    }
}

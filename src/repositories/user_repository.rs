use sqlx::query_as;

use crate::models::user::User;

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
        let inserted_user: User = sqlx::query_as(
            r#"
            INSERT INTO users (email, verified, full_name, first_name, last_name, picture_url)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
        "#,
        )
        .bind(&user.email)
        .bind(user.verified)
        .bind(&user.full_name)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.picture_url)
        .fetch_one(&self.db)
        .await?;

        Ok(inserted_user)
    }
}

use serde::{Deserialize, Serialize};

use crate::infrastructure::auth::google::GoogleUser;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub picture_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub async fn find_by_email(
        email: &str,
        executor: &sqlx::SqlitePool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(r#"SELECT * FROM users WHERE email = ?"#)
            .bind(email)
            .fetch_optional(executor)
            .await
    }

    pub async fn create(&self, executor: &sqlx::SqlitePool) -> Result<Self, sqlx::Error> {
        let inserted_user: User = sqlx::query_as(
            r#"
            INSERT INTO users (email, verified, full_name, first_name, last_name, picture_url)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING * 
        "#,
        )
        .bind(&self.email)
        .bind(&self.verified)
        .bind(&self.full_name)
        .bind(&self.first_name)
        .bind(&self.last_name)
        .bind(&self.picture_url)
        .fetch_one(executor)
        .await?;

        Ok(inserted_user)
    }
}

impl From<GoogleUser> for User {
    fn from(google_user: GoogleUser) -> Self {
        User {
            id: 1,
            email: google_user.email,
            verified: google_user.email_verified,
            first_name: google_user.given_name.unwrap_or(google_user.name.clone()),
            last_name: google_user.family_name,
            full_name: google_user.name,
            picture_url: google_user.picture,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }
}

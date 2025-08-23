use serde::{Deserialize, Serialize};

use crate::infrastructure::auth::google::GoogleUser;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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
    pub stripe_customer_id: Option<String>,
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
            stripe_customer_id: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }
}

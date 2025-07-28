use axum::Router;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    handlers::{auth, homepage},
    infrastructure::database::Database,
};

pub mod extractors;
pub mod handlers;
pub mod infrastructure;
pub mod models;

#[derive(Clone)]
pub struct AppDetails {
    pub name: String,
    pub display_name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub app_details: AppDetails,
    pub db: SqlitePool,
    pub is_dev_mode: bool,
}

pub async fn start() {
    let app = initialize_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn initialize_app() -> Router {
    let db = Database::initialize().await;
    let is_dev_mode = cfg!(debug_assertions);
    let app_details = AppDetails {
        name: "axum-template".to_string(),
        display_name: "Axum Template".to_string(),
    };

    let state = Arc::new(AppState {
        db,
        app_details,
        is_dev_mode,
    });

    Router::new()
        .merge(homepage::routes())
        .merge(auth::routes())
        .with_state(state)
}

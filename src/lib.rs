use std::{env, sync::Arc};

use axum::Router;
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};

use crate::handlers::{auth, homepage};

pub mod handlers;
pub mod infrastructure;
pub mod models;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone)]
pub struct AppDetails {
    pub name: String,
    pub display_name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub app_details: AppDetails,
    pub db: SqlitePool,
}

pub async fn start() {
    let app = initialize_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn initialize_database() -> SqlitePool {
    let db_url = env::var("DATABASE_URL").unwrap().to_string();
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database.");

    // Run Database migrations
    MIGRATOR.run(&pool).await.expect("Failed to run migrations");

    pool
}

async fn initialize_app() -> Router {
    let app_details = AppDetails {
        name: "axum-template".to_string(),
        display_name: "Axum Template".to_string(),
    };
    let db = initialize_database().await;

    let state = Arc::new(AppState { db, app_details });

    Router::new()
        .merge(homepage::routes())
        .merge(auth::routes())
        .with_state(state)
}

use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct Database {}

impl Database {
    pub async fn initialize() -> SqlitePool {
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
}

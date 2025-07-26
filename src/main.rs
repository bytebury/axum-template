#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables from .env

    axum_template::start().await
}

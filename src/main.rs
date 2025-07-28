#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    axum_template::start().await
}

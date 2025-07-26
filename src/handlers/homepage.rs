use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct HomepageTemplate {
    app_display_name: String,
}

async fn homepage(State(state): State<Arc<AppState>>) -> HomepageTemplate {
    HomepageTemplate {
        app_display_name: state.app_details.display_name.to_string(),
    }
}

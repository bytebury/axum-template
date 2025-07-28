use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};

use crate::{AppState, extractors::maybe_current_user::MaybeCurrentUser, models::user::User};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct HomepageTemplate {
    current_user: Option<User>,
    app_display_name: String,
    version: String,
}

async fn homepage(
    State(state): State<Arc<AppState>>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> HomepageTemplate {
    HomepageTemplate {
        app_display_name: state.app_details.display_name.to_string(),
        version: state.app_details.version.to_string(),
        current_user,
    }
}

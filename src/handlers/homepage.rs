use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};

use crate::{
    AppState, extractors::maybe_current_user::MaybeCurrentUser, handlers::SharedTemplateContext,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct HomepageTemplate {
    shared: SharedTemplateContext,
}

async fn homepage(
    State(state): State<Arc<AppState>>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedTemplateContext::new(&state, current_user),
    }
}

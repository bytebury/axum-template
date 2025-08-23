use std::{env, sync::Arc};

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::Redirect, routing::get};

use crate::{
    AppState, extractors::maybe_current_user::MaybeCurrentUser, handlers::SharedTemplateContext,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(homepage))
        .route("/buy-subscription", get(buy_subscription))
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

async fn buy_subscription(
    State(state): State<Arc<AppState>>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    if let Some(user) = current_user {
        let price_id = env::var("STRIPE_PRICE_ID").expect("STRIPE_PRICE_ID must be set");

        match state.stripe.checkout(&user, &price_id).await {
            Ok(session) => {
                let url = session.url.ok_or((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Missing checkout URL".to_string(),
                ))?;
                Ok(Redirect::to(&url))
            }
            Err(err) => Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )),
        }
    } else {
        Ok(Redirect::to("/"))
    }
}

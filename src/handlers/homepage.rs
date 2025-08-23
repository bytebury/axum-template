use std::{env, sync::Arc};

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{ConnectInfo, State},
    response::Redirect,
    routing::get,
};

use crate::{
    AppState,
    extractors::maybe_current_user::MaybeCurrentUser,
    handlers::SharedTemplateContext,
    infrastructure::audit::ip_address::{self, CountryDetails},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(homepage))
        .route("/buy-subscription", get(buy_subscription))
        .route("/manage-subscription", get(manage_subscriptions))
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct HomepageTemplate {
    shared: SharedTemplateContext,
    country: CountryDetails,
}

async fn homepage(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedTemplateContext::new(&state, current_user),
        country: ip_address::get_country_details(addr.ip())
            .unwrap()
            .unwrap_or_default(),
    }
}

async fn buy_subscription(
    State(state): State<Arc<AppState>>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let user = match current_user {
        Some(u) => u,
        None => return Ok(Redirect::to("/")),
    };

    let price_id = env::var("STRIPE_PRICE_ID").expect("STRIPE_PRICE_ID must be set");
    let session = state
        .stripe
        .checkout(&user, &price_id)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let url = session.url.ok_or((
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Missing checkout URL".to_string(),
    ))?;

    Ok(Redirect::to(&url))
}

async fn manage_subscriptions(
    State(state): State<Arc<AppState>>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let user = match current_user {
        Some(u) => u,
        None => return Ok(Redirect::to("/")),
    };

    let session = state
        .stripe
        .manage_subscription(&user)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to(&session.url))
}

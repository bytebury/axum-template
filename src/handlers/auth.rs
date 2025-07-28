use axum::{
    Json, Router,
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    AppState,
    infrastructure::auth::{OAuth, google::GoogleOAuth},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/google", get(signin_with_google))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/signout", delete(signout))
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
}

async fn signin_with_google() -> impl IntoResponse {
    Redirect::to(GoogleOAuth::new().auth_url().as_str())
}

async fn google_callback(Query(params): Query<AuthRequest>) -> impl IntoResponse {
    let oauth = GoogleOAuth::new();
    let user = oauth.exchange_code_for_user(&params.code).await;

    // TODO: generate a JWT with this information
    //       create the user if they do not exist (via e-mail)
    //       set the auth_token with the JWT to sign them in.
    //       redirect them to the appropriate place

    match user {
        Some(user) => Json(user).into_response(),
        None => Html("Something went wrong".to_string()).into_response(),
    }
}

async fn signout() -> impl IntoResponse {
    Html("TODO -- SIGN A USER OUT")
}

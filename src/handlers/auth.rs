use axum::{
    Json, Router,
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get},
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    AppState,
    infrastructure::auth::{OAuth, google::GoogleOAuth},
    models::user::User,
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

async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let oauth = GoogleOAuth::new();
    let user = oauth.exchange_code_for_user(&params.code).await?;

    // TODO: When we get the user, we need to generate the JWT
    //       and then redirect them where they need to go.

    if let Ok(user) = User::find_by_email(&user.email, &state.db).await {
        return Ok(Json(user));
    }

    let inserted_user = user
        .create(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inserted_user))
}

async fn signout() -> impl IntoResponse {
    Html("TODO -- SIGN A USER OUT")
}

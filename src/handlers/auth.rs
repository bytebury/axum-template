use axum::{
    Json, Router,
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get},
};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    AppState,
    infrastructure::auth::{
        OAuth,
        google::{GoogleOAuth, GoogleUserInfo},
    },
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
    let access_token = GoogleOAuth::new()
        .exchange_code_for_access_token(&params.code)
        .await;

    // TODO: generate a JWT with this information
    //       create the user if they do not exist
    //       set the auth_token with the JWT to sign them in.

    match fetch_google_user_info(&access_token).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => Html("Something went wrong".to_string()).into_response(),
    }
}

async fn fetch_google_user_info(token: &str) -> Result<GoogleUserInfo, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<GoogleUserInfo>()
        .await?;

    Ok(res)
}

async fn signout() -> impl IntoResponse {
    Html("TODO -- SIGN A USER OUT")
}

use axum::{
    Router,
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{self, Cookie},
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    AppState,
    infrastructure::{
        auth::{OAuth, google::GoogleOAuth},
        jwt::{JwtService, user_claims::UserClaims},
    },
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
    Redirect::to(GoogleOAuth::default().auth_url().as_str())
}

async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let user = GoogleOAuth::default()
        .exchange_code_for_user(&params.code)
        .await?;

    let user = match User::find_by_email(&user.email, &state.db).await {
        Ok(Some(user)) => user,
        Ok(None) => user
            .create(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let token = JwtService::generate(&UserClaims::from(user))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .secure(!state.is_dev_mode);

    let cookies = cookies.add(auth_cookie);

    Ok((cookies, Html("<script>window.location = '/'</script>")))
}

async fn signout(State(state): State<Arc<AppState>>, cookies: CookieJar) -> impl IntoResponse {
    let cookies = cookies.remove(
        Cookie::build(("auth_token", ""))
            .path("/")
            .secure(!state.is_dev_mode)
            .http_only(true)
            .same_site(cookie::SameSite::Strict),
    );
    (cookies, Html("<script>window.location = '/'</script>"))
}

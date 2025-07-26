use std::sync::Arc;

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::{delete, get},
};

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/google", get(signin_with_google))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/signout", delete(signout))
}

async fn signin_with_google() -> impl IntoResponse {
    Html("Hello World")
}

async fn google_callback() -> impl IntoResponse {
    Html("Hello World")
}

async fn signout() -> impl IntoResponse {
    Html("Hello World")
}

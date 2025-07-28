use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{AppState, infrastructure::jwt::JwtService, models::user::User};

pub struct MaybeCurrentUser(pub Option<User>);

impl FromRequestParts<Arc<AppState>> for MaybeCurrentUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, StatusCode> {
        let state = Arc::from_ref(state);
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("auth_token") {
            Some(cookie) => cookie.value(),
            None => return Ok(MaybeCurrentUser(None)),
        };

        let token_data = match JwtService::verify(token) {
            Ok(claims) => claims,
            Err(_) => return Ok(MaybeCurrentUser(None)),
        };

        let user = match User::find_by_email(&token_data.claims.email, &state.db).await {
            Ok(Some(user)) => Some(user),
            _ => None,
        };

        Ok(MaybeCurrentUser(user))
    }
}

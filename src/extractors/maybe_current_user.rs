use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{
    AppState,
    infrastructure::jwt::{JwtService, user_claims::UserClaims},
    models::user::User,
    repositories::user_repository::UserRepository,
};

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

        let token_data = match JwtService::verify::<UserClaims>(token) {
            Ok(claims) => claims,
            Err(_) => return Ok(MaybeCurrentUser(None)),
        };

        let user_repository = UserRepository::new(&state.db);
        let user = match user_repository
            .find_by_email(&token_data.claims.email)
            .await
        {
            Ok(Some(user)) => Some(user),
            _ => None,
        };

        Ok(MaybeCurrentUser(user))
    }
}

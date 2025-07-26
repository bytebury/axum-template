use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get},
};
use oauth2::CsrfToken;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl,
    RevocationUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

use crate::AppState;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
}

async fn google_oauth_client()
-> BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet> {
    let client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variables."),
    );
    let client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid auth endpoint URL.");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL.");
    let redirect_url = RedirectUrl::new("http://localhost:8080/auth/google/callback".to_string())
        .expect("Invalid redirect URL");
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("Invalid revocation endpoint URL");

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
        .set_revocation_url(revocation_url)
}

async fn signin_with_google() -> impl IntoResponse {
    let (authorize_url, _csrf_state) = google_oauth_client()
        .await
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    Redirect::to(authorize_url.as_str())
}

async fn google_callback(Query(params): Query<AuthRequest>) -> impl IntoResponse {
    let http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    let token_result = google_oauth_client()
        .await
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&http_client)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    let access_token: String = token_result
        .expect("Access token should be available")
        .access_token()
        .secret()
        .clone();

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

use std::env;

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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

pub async fn google_authorize_url() -> String {
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
    return authorize_url.as_str().to_string();
}

pub async fn google_exchange_code_for_user(code: &str) -> String {
    let http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    let token_result = google_oauth_client()
        .await
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    token_result
        .expect("Access token should be available")
        .access_token()
        .secret()
        .clone()
}

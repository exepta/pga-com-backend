#![allow(unused)]

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuth2Provider {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
}

impl Default for OAuth2Provider {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            auth_url: String::new(),
            token_url: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct OAuth2ProviderFactory { }

impl OAuth2ProviderFactory {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {  })
    }
}

impl OAuth2ProviderFactory {

    pub async fn oauth2_login(&self, oauth2_user: OAuth2Provider) -> crate::Result<()>{
        let client = BasicClient::new(
            ClientId::new(oauth2_user.client_id.clone()),
            Some(ClientSecret::new(oauth2_user.client_secret.clone())),
            AuthUrl::new(oauth2_user.auth_url.clone()).unwrap(),
            Some(TokenUrl::new(oauth2_user.token_url.clone()).unwrap()),
        ).set_redirect_uri(RedirectUrl::new("http://localhost:8090/v0/auth/callback".to_string()).unwrap());

        let (authorize_url, _pkce_code_verifier) = client
            .authorize_url(|| CsrfToken::new_random())
            .add_scope(Scope::new("profile".to_string()))
            .url();

        println!("Browse to: {}", authorize_url);

        let token = client
            .exchange_code(AuthorizationCode::new("AUTH_CODE".to_string()))
            .request_async(async_http_client)
            .await.unwrap();

        println!("Token: {}", token.access_token().secret());
        Ok(())
    }
}
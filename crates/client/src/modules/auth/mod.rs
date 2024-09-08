pub mod token_create;
pub mod user_register;

use anyhow::{Result, anyhow};
use http_auth_basic::Credentials;
use reqwest::{Client, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION};

pub struct AuthClient {
    client: Client,
    domain: Url,
}

impl AuthClient {
    pub fn new(domain: Url) -> Self {
        Self {
            domain,
            client: Client::new(),
        }
    }

    pub async fn login(&self, email: String, password: String) -> Result<()> {
        let credentials = Credentials::new(email.as_str(), password.as_str());
        let authorization = credentials.as_http_header();
        let authorization = HeaderValue::from_str(authorization.as_str()).unwrap();
        let url = self.domain.join("/api/v1/auth/login")?;

        self.client
            .get(url)
            .header(AUTHORIZATION, authorization)
            .send()
            .await
            .map_err(|err| anyhow!("Failed to authenticate. {err}"))?;

        Ok(())
    }

    pub async fn token_create(&self, email: String, password: String) -> Result<token_create::TokenCreate> {
        token_create::token_create(self, email, password).await
    }

    pub async fn user_register(
        &self,
        input: user_register::UserRegisterInput,
    ) -> Result<user_register::UserRegister> {
        user_register::user_register(self, input).await
    }
}

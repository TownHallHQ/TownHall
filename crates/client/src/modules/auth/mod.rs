pub mod token_create;
pub mod user_register;

use http_auth_basic::Credentials;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};

pub struct AuthClient {
    client: Client,
}

impl Default for AuthClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn login(&self, email: String, password: String) {
        let credentials = Credentials::new(email.as_str(), password.as_str());
        let authorization = credentials.as_http_header();
        let authorization = HeaderValue::from_str(authorization.as_str()).unwrap();

        self.client
            .get("http://127.0.0.1:7878/api/v1/auth/login")
            .header(AUTHORIZATION, authorization)
            .send()
            .await
            .unwrap();
    }

    pub async fn token_create(&self, email: String, password: String) -> token_create::TokenCreate {
        token_create::token_create(&self.client, email, password).await
    }

    pub async fn user_register(
        &self,
        input: user_register::UserRegisterInput,
    ) -> user_register::UserRegister {
        user_register::user_register(&self.client, input).await
    }
}

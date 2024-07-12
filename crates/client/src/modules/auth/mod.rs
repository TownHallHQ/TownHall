pub mod token_create;
pub mod user_register;

use reqwest::Client;

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

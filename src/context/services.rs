use std::sync::Arc;

use crate::modules::auth::service::AuthService;

use super::Config;

pub struct Services {
    pub auth: Arc<AuthService>,
}

impl Services {
    pub fn new(config: &Config) -> Self {
        let auth = AuthService::new(config.jwt_secret.clone());

        Self {
            auth: Arc::new(auth),
        }
    }
}

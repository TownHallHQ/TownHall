use std::sync::Arc;

use crate::modules::auth::service::AuthService;

use super::{Config, Store};

pub struct Services {
    pub auth: Arc<AuthService>,
}

impl Services {
    pub fn new(_store: Arc<Store>, config: &Config) -> Self {
        let auth = AuthService::new(config.jwt_secret.clone());

        Self {
            auth: Arc::new(auth),
        }
    }
}

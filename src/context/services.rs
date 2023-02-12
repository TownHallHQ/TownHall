use std::sync::Arc;

use crate::modules::auth::service::AuthService;
use crate::modules::link::service::LinkService;

use super::{Config, Store};

pub struct Services {
    pub auth: Arc<AuthService>,
    pub link: Arc<LinkService>,
}

impl Services {
    pub fn new(store: Arc<Store>, config: &Config) -> Self {
        let auth = AuthService::new(config.jwt_secret.clone());
        let link = LinkService::new(Arc::clone(&store));

        Self {
            auth: Arc::new(auth),
            link: Arc::new(link),
        }
    }
}

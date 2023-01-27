use std::sync::Arc;

use crate::modules::auth::service::AuthService;
use crate::modules::{link::service::LinkService, user::service::UserService};

use super::{Config, Store};

pub struct Services {
    pub auth: Arc<AuthService>,
    pub user: Arc<UserService>,
    pub link: Arc<LinkService>,
}

impl Services {
    pub fn new(store: Arc<Store>, config: &Config) -> Self {
        let auth = AuthService::new(config.jwt_secret.clone());
        let user = UserService::new(Arc::clone(&store));
        let link = LinkService::new(Arc::clone(&store));

        Self {
            auth: Arc::new(auth),
            user: Arc::new(user),
            link: Arc::new(link),
        }
    }
}

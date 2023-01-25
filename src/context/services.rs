use std::sync::Arc;

// use crate::modules::auth::service::AuthService;
use crate::modules::{link::service::LinkService, user::service::UserService};

use super::{Config, Store};

pub struct Services {
    // auth: Arc<AuthService>,
    pub user: Arc<UserService>,
    pub link: Arc<LinkService>,
}

impl Services {
    pub fn new(store: Arc<Store>) -> Self {
        let user = UserService::new(Arc::clone(&store));
        let link = LinkService::new(Arc::clone(&store));

        Self {
            user: Arc::new(user),
            link: Arc::new(link),
        }
    }
}

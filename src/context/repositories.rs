use std::sync::Arc;

use crate::modules::link::repository::LinkRepository;
use crate::modules::user::repository::UserRepository;

use super::Store;

pub struct Repositories {
    pub link: LinkRepository,
    pub user: UserRepository,
}

impl Repositories {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            link: LinkRepository::new(Arc::clone(&store)),
            user: UserRepository::new(Arc::clone(&store)),
        }
    }
}

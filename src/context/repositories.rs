use std::sync::Arc;

use crate::modules::user::repository::UserRepository;

use super::Store;

pub struct Repositories {
    pub user: UserRepository,
}

impl Repositories {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            user: UserRepository::new(Arc::clone(&store)),
        }
    }
}

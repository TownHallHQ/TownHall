pub mod user;
pub mod user_email;

use std::sync::Arc;

use crate::context::Store;

use self::user::UserRepository;
use self::user_email::UserEmailRepository;

pub struct UserRepositories {
    /// Stores emails and user IDs belonging to those emails
    pub user_email: UserEmailRepository,
    pub user: UserRepository,
}

impl UserRepositories {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            user_email: UserEmailRepository::new(Arc::clone(&store)),
            user: UserRepository::new(Arc::clone(&store)),
        }
    }
}

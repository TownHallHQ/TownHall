use std::sync::Arc;

use tracing::info;

use crate::context::Store;
use crate::modules::user::model::User;
use crate::shared::repository::Repository;

use super::repositories::user::CreateUserDto;
use super::repositories::UserRepositories;

pub struct UserService {
    repositories: Arc<UserRepositories>,
}

impl UserService {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            repositories: Arc::new(UserRepositories::new(store)),
        }
    }

    pub fn list(&self) -> Vec<User> {
        self.repositories.user.list().unwrap()
    }

    pub fn get(&self, id: String) -> Option<User> {
        self.repositories.user.find_by_id(id).unwrap()
    }

    pub fn create(&self, dto: CreateUserDto) -> User {
        // Verify the email in the `dto` is actually available
        // TODO: Improve `find_by_id` to use borrowing
        self.repositories.user_email.find_by_id(dto.email.clone());
        let user = self.repositories.user.create(dto).unwrap();

        info!(email=%user.email, "User created with success");
        user
    }

    pub fn find_by_email(&self, email: String) -> Option<User> {
        let users = self.list();

        users.into_iter().find(|item| email == item.email)
    }
}

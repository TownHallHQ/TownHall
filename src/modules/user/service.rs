use std::sync::Arc;

use crate::context::Store;
use crate::modules::user::model::User;
use crate::shared::repository::Repository;

use super::repository::{CreateUserDto, UserRepository};

pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            repository: Arc::new(UserRepository::new(store)),
        }
    }

    pub fn list(&self) -> Vec<User> {
        self.repository.list().unwrap()
    }

    pub fn get(&self, id: String) -> Option<User> {
        self.repository.find_by_id(id).unwrap()
    }

    pub fn create(&self, dto: CreateUserDto) -> User {
        self.repository.create(dto).unwrap()
    }

    pub fn find_by_email(&self, email: String) -> Option<User> {
        let users = self.list();

        users.into_iter().find(|item| email == item.email)
    }
}

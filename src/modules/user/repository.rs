use std::sync::Arc;

use serde::Serialize;

use crate::context::Store;
use crate::shared::repository::Repository;

use crate::modules::user::model::User;

const USER_REPOSITORY_TREE: char = 'u';

pub struct UserRepository {
    store: Arc<Store>,
}

impl UserRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

impl Repository<USER_REPOSITORY_TREE, User> for UserRepository {
    type Error = ();

    fn get_tree(&self) -> sled::Result<sled::Tree> {
        self.store.db.open_tree(USER_REPOSITORY_TREE.to_string())
    }
}

#[derive(Clone, Serialize)]
pub struct CreateUserDto {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password_hash: String,
}

impl From<CreateUserDto> for User {
    fn from(value: CreateUserDto) -> Self {
        User {
            name: value.name,
            surname: value.surname,
            email: value.email,
            password_hash: value.password_hash,
            ..Default::default()
        }
    }
}

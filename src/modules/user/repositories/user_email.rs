use std::sync::Arc;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::context::Store;
use crate::shared::repository::{Record, Repository};

const USER_REPOSITORY_TREE: char = 'e';

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserEmail {
    pub user_id: String,
    pub user_email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Record for UserEmail {
    fn get_id(&self) -> String {
        self.user_email.to_string()
    }

    fn set_id(&mut self, id: &str) {
        self.user_email = id.to_string()
    }
}

pub struct UserEmailRepository {
    store: Arc<Store>,
}

impl UserEmailRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

impl Repository<USER_REPOSITORY_TREE, UserEmail> for UserEmailRepository {
    type Error = ();

    fn get_tree(&self) -> sled::Result<sled::Tree> {
        self.store.db.open_tree(USER_REPOSITORY_TREE.to_string())
    }
}

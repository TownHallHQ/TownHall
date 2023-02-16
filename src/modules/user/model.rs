use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::shared::repository::Record;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password_hash: String,
    pub links_ids: Vec<Vec<u8>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Record for User {
    fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    fn set_id(&mut self, id: &[u8]) {
        self.id = id.to_vec();
    }

    fn set_updated_at(&mut self) {
        self.updated_at = Local::now();
    }
}

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::shared::repository::Record;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Record for User {
    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }
}

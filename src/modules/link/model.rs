use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::shared::repository::Record;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub hash: String,
    pub original_url: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub owner_id: Option<String>,
}

impl Record for Link {
    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }
}

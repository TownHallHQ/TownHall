use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::shared::repository::Record;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Link {
    pub id: Vec<u8>,
    pub original_url: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub owner_id: Option<Vec<u8>>,
}

impl Record for Link {
    fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    fn set_id(&mut self, id: &[u8]) {
        self.id = id.to_owned();
    }
}

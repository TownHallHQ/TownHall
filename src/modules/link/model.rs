use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub hash: String,
    pub original_url: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub owner_id: Option<String>,
}

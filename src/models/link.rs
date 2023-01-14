use chrono::{DateTime, Local};

pub struct Model {
    pub id: i32,
    pub hash: String,
    pub original_url: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub owner_id: Option<i32>,
}

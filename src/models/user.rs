use chrono::{DateTime, Local};

pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl User {}

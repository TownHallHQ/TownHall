use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::Result;
use crate::user::model::Email;

#[async_trait]
pub trait UserRepository: Clone {
    async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord>;
    async fn find_by_email(&self, email: Email) -> Result<Option<UserRecord>>;
    async fn find_by_id(&self, id: Pxid) -> Result<Option<UserRecord>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    /// User's Phone Number, probably can use:
    ///
    /// https://github.com/rin-nas/postgresql-patterns-library/blob/master/domains/phone.sql
    /// https://github.com/rin-nas/postgresql-patterns-library/blob/master/functions/phone_parse.sql
    pub phone: Option<String>,
    pub password_hash: String,
    pub avatar_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertUserDto {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
}

pub mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use chrono::Utc;
    use pxid::Pxid;

    use crate::user::error::Result;
    use crate::user::model::{Email, User};
    use crate::user::repository::{InsertUserDto, UserRepository};

    use super::UserRecord;

    #[derive(Clone)]
    pub struct TestUserRepository(Arc<Mutex<HashMap<Pxid, UserRecord>>>);

    impl TestUserRepository {
        pub fn new() -> Self {
            TestUserRepository(Arc::new(Mutex::new(HashMap::new())))
        }

        pub fn clear(&self) {
            let mut lock = self.0.lock().unwrap();

            lock.clear();
        }
    }

    #[async_trait]
    impl UserRepository for TestUserRepository {
        async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord> {
            let mut db = self.0.lock().unwrap();
            let id = User::new_id()?;

            db.insert(
                id.clone(),
                UserRecord {
                    id: id.to_string(),
                    name: dto.name,
                    surname: dto.surname,
                    email: dto.email.clone(),
                    phone: dto.phone.clone(),
                    password_hash: dto.password_hash,
                    avatar_id: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                },
            );

            Ok(db.get(&id).unwrap().to_owned())
        }

        async fn find_by_email(&self, email: Email) -> Result<Option<UserRecord>> {
            let db = self.0.lock().unwrap();
            let maybe_user_record = db.values().find(|user_record| {
                if user_record.email == email.to_string() {
                    return true;
                }

                false
            });

            Ok(maybe_user_record.and_then(|user_record| Some(user_record.to_owned())))
        }

        async fn find_by_id(&self, id: Pxid) -> Result<Option<UserRecord>> {
            let db = self.0.lock().unwrap();

            if let Some(user_record) = db.get(&id) {
                return Ok(Some(user_record.to_owned()));
            }

            Ok(None)
        }
    }
}

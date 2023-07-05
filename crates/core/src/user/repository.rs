use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::common::Database;
use crate::user::error::{Result, UserError};
use crate::user::model::{Email, User};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFilter {
    pub id: Option<Pxid>,
    pub email: Option<Email>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertUserDto {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone)]
pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    pub fn into_record(model: entity::user::Model) -> UserRecord {
        UserRecord {
            id: model.id,
            name: model.name,
            surname: model.surname,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
            created_at: DateTime::from_utc(model.created_at, Utc),
            updated_at: DateTime::from_utc(model.updated_at, Utc),
            deleted_at: model.deleted_at.map(|naive| DateTime::from_utc(naive, Utc)),
        }
    }

    pub async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord> {
        let active_model = entity::user::ActiveModel {
            id: Set(User::generate_id()?.to_string()),
            name: Set(dto.name),
            surname: Set(dto.surname),
            username: Set(dto.username.to_string()),
            email: Set(dto.email.to_string()),
            password_hash: Set(dto.password_hash),
            ..Default::default()
        };

        let model = active_model.insert(&*self.db).await.map_err(|err| {
            tracing::error!(%err, "Failed to insert into database");
            match err {
                migration::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    if let Some(db_err) = err.into_database_error() {
                        if let Some(constraint) = db_err.constraint() {
                            if constraint == "user_username_key" {
                                return UserError::UsernameTakenError(dto.username);
                            }

                            if constraint == "user_email_key" {
                                return UserError::EmailTakenError(dto.email);
                            }
                        }
                    }
                    UserError::DatabaseError
                }
                _ => UserError::DatabaseError,
            }
        })?;

        Ok(UserRepository::into_record(model))
    }

    pub async fn find(&self, filter: Option<UserFilter>) -> Result<Vec<UserRecord>> {
        let mut query = entity::user::Entity::find();

        if let Some(filter) = filter {
            if let Some(id) = filter.id {
                query = query.filter(entity::user::Column::Id.eq(id.to_string()));
            }

            if let Some(email) = filter.email {
                query = query.filter(entity::user::Column::Email.eq(email.to_string()));
            }
        }

        let models = query.all(&*self.db).await.map_err(|err| {
            tracing::error!(%err, "Failed to find from database");
            UserError::DatabaseError
        })?;

        Ok(models
            .into_iter()
            .map(UserRepository::into_record)
            .collect())
    }
}

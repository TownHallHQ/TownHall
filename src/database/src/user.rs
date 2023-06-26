use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use gabble::user::error::{Result, UserError};
use gabble::user::model::User;
use gabble::user::repository::{InsertUserDto, UserFilter, UserRecord};

use crate::Database;

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
}

#[async_trait]
impl gabble::user::repository::UserRepository for UserRepository {
    async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord> {
        let active_model = entity::user::ActiveModel {
            id: Set(User::generate_id()?.to_string()),
            name: Set(dto.name),
            surname: Set(dto.surname),
            username: Set(dto.username.to_string()),
            email: Set(dto.email.to_string()),
            password_hash: Set(dto.password_hash),
            ..Default::default()
        };

        let model = active_model.insert(&*self.db.0).await.map_err(|err| {
            // TODO: Handle duplicated ULID error
            match err {
                sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    if let Some(db_err) = err.into_database_error() {
                        match db_err.constraint() {
                            Some("user_email_key") => {
                                tracing::error!(%db_err);
                                return UserError::EmailTakenError(dto.email);
                            }
                            Some("user_username_key") => {
                                tracing::error!(%db_err);
                                return UserError::UsernameTakenError(dto.username);
                            }
                            _ => {
                                tracing::error!(%db_err, "Failed to insert into database");
                                return UserError::DatabaseError;
                            }
                        }
                    } else {
                        tracing::error!("Failed to insert into database");
                        return UserError::DatabaseError;
                    }
                }
                _ => {
                    tracing::error!(%err, "Failed to insert into database");
                    return UserError::DatabaseError;
                }
            }
        })?;

        Ok(UserRepository::into_record(model))
    }

    async fn find(&self, filter: Option<UserFilter>) -> Result<Vec<UserRecord>> {
        let mut query = entity::user::Entity::find();

        if let Some(filter) = filter {
            if let Some(id) = filter.id {
                query = query.filter(entity::user::Column::Id.eq(id.to_string()));
            }

            if let Some(email) = filter.email {
                query = query.filter(entity::user::Column::Email.eq(email.to_string()));
            }
        }

        let models = query.all(&*self.db.0).await.map_err(|err| {
            tracing::error!(%err, "Failed to find from database");
            UserError::DatabaseError
        })?;

        Ok(models
            .into_iter()
            .map(UserRepository::into_record)
            .collect())
    }
}

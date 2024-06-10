use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};

use crate::shared::database::Database;
use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;
use crate::user::error::{Result, UserError};
use crate::user::model::{Email, User, Username};

const UNIQUE_USERNAME_CONSTRAINT: &str = "user_username_key";
const UNIQUE_EMAIL_CONSTRAINT: &str = "user_email_key";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFilter {
    pub id: Option<Pxid>,
    pub email: Option<Email>,
    pub username: Option<Username>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
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
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub struct UpdateUserDto {
    pub name: Option<String>,
    pub surname: Option<String>,
}

impl TryFrom<entity::user::Model> for User {
    type Error = UserError;

    fn try_from(model: entity::user::Model) -> Result<Self> {
        let username = Username::from_str(&model.username)?;
        let email = Email::from_str(&model.email)?;

        Ok(User {
            id: Pxid::from_str(&model.id)?,
            name: model.name,
            surname: model.surname,
            username,
            email,
            avatar_id: model.avatar_id.map(|id| Pxid::from_str(&id)).transpose()?,
            created_at: DateTime::from_naive_utc_and_offset(model.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(model.updated_at, Utc),
            deleted_at: model
                .deleted_at
                .map(|naive| DateTime::from_naive_utc_and_offset(naive, Utc)),
        })
    }
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
            avatar_id: model.avatar_id,
            created_at: DateTime::from_naive_utc_and_offset(model.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(model.updated_at, Utc),
            deleted_at: model
                .deleted_at
                .map(|naive| DateTime::from_naive_utc_and_offset(naive, Utc)),
        }
    }

    pub async fn insert(&self, dto: InsertUserDto) -> Result<User> {
        let active_model = entity::user::ActiveModel {
            id: Set(User::pxid()?.to_string()),
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
                sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    if let Some(db_err) = err.into_database_error() {
                        if let Some(constraint) = db_err.constraint() {
                            if constraint == UNIQUE_USERNAME_CONSTRAINT {
                                return UserError::UsernameTakenError(dto.username);
                            }

                            if constraint == UNIQUE_EMAIL_CONSTRAINT {
                                return UserError::EmailTakenError(dto.email);
                            }
                        }
                    }
                    UserError::DatabaseError
                }
                _ => UserError::DatabaseError,
            }
        })?;

        User::try_from(model)
    }

    pub async fn update(&self, id: Pxid, dto: UpdateUserDto) -> Result<UserRecord> {
        let user = entity::prelude::User::find_by_id(&id.to_string())
            .one(&*self.db)
            .await
            .map_err(|_| UserError::DatabaseError)?;

        if let Some(u) = user {
            let mut active_model: entity::user::ActiveModel = u.into();

            if let Some(name) = dto.name {
                active_model.name = Set(name);
            }
            if let Some(surname) = dto.surname {
                active_model.surname = Set(surname);
            }

            let record = active_model
                .update(&*self.db)
                .await
                .map_err(|_| UserError::DatabaseError)?;

            return Ok(Self::into_record(record));
        }

        Err(UserError::UserNotFound)
    }

    pub async fn list(
        &self,
        pagination: Option<Pagination>,
        filter: Option<UserFilter>,
    ) -> Result<QuerySet<UserRecord>> {
        self.db
            .transaction::<_, QuerySet<UserRecord>, UserError>(|txn| {
                Box::pin(async move {
                    let mut query = entity::user::Entity::find();

                    if let Some(filter) = filter {
                        if let Some(id) = filter.id {
                            query = query.filter(entity::user::Column::Id.eq(id.to_string()));
                        }

                        if let Some(email) = filter.email {
                            query = query.filter(entity::user::Column::Email.eq(email.to_string()));
                        }

                        if let Some(username) = filter.username {
                            query = query
                                .filter(entity::user::Column::Username.eq(username.to_string()));
                        }
                    }

                    let count = query
                        .clone()
                        .select_only()
                        .count(txn)
                        .await
                        .map_err(|err| {
                            tracing::error!(%err, "Failed to count total users");
                            UserError::DatabaseError
                        })?;

                    let pagination = pagination.unwrap_or_default();

                    let mut query = query.cursor_by(entity::user::Column::Id);

                    pagination.apply(&mut query);

                    let active_records = query.all(txn).await.map_err(|err| {
                        tracing::error!(%err, "Failed to retrieve users");
                        UserError::DatabaseError
                    })?;

                    if active_records.is_empty() {
                        return Ok(QuerySet::empty());
                    }
                    let records = active_records.into_iter().map(Self::into_record).collect();

                    Ok(QuerySet::new(records, count))
                })
            })
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to retrieve users");
                UserError::DatabaseError
            })
    }

    pub async fn find_by_email(&self, email: &Email) -> Result<Option<UserRecord>> {
        let maybe_user = entity::prelude::User::find()
            .filter(entity::user::Column::Email.eq(email.to_string()))
            .one(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, %email, "Failed to find User by email");
                UserError::DatabaseError
            })?;

        if let Some(user_model) = maybe_user {
            return Ok(Some(Self::into_record(user_model)));
        }

        Ok(None)
    }

    pub async fn find_password_hash_by_email(&self, email: &Email) -> Result<Option<String>> {
        let maybe_user = entity::prelude::User::find()
            .filter(entity::user::Column::Email.eq(email.to_string()))
            .one(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, %email, "Failed to find User by email");
                UserError::DatabaseError
            })?;

        if let Some(user_model) = maybe_user {
            return Ok(Some(user_model.password_hash));
        }

        Ok(None)
    }

    pub async fn find_by_id(&self, id: &Pxid) -> Result<Option<UserRecord>> {
        let maybe_user = entity::prelude::User::find()
            .filter(entity::user::Column::Id.eq(id.to_string()))
            .one(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, %id, "Failed to find User by ID");
                UserError::DatabaseError
            })?;

        if let Some(user_model) = maybe_user {
            return Ok(Some(Self::into_record(user_model)));
        }

        Ok(None)
    }

    pub async fn update_avatar(&self, id: Pxid, avatar_id: Pxid) -> Result<UserRecord> {
        let maybe_user = entity::prelude::User::find()
            .filter(entity::user::Column::Id.eq(id.to_string()))
            .one(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, %id, %avatar_id, "Failed to find User by ID");
                UserError::DatabaseError
            })?;

        if let Some(user_model) = maybe_user {
            let mut active_user: entity::user::ActiveModel = user_model.into();

            active_user.avatar_id = Set(Some(avatar_id.to_string()));
            let user_model = active_user.update(&*self.db).await.map_err(|err| {
                tracing::error!(%err, "Failed to update user avatar");
                UserError::UserNotFound
            })?;

            return Ok(Self::into_record(user_model));
        }

        Err(UserError::UserNotFound)
    }
}

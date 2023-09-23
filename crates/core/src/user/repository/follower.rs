use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, CursorTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::shared::database::Database;
use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;
use crate::user::error::{Result, UserError};
use crate::user::service::FollowPeers;

const FK_USER_FOLLOWERS_FOLLOWER_USER: &str = "FK_user_followers_follower_user";
const FK_USER_FOLLOWERS_FOLLOWEE_USER: &str = "FK_user_followers_followee_user";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFollowersRecord {
    pub follower_id: Pxid,
    pub followee_id: Pxid,
    pub created_at: DateTime<Utc>,
}

impl From<entity::user_followers::Model> for UserFollowersRecord {
    fn from(value: entity::user_followers::Model) -> Self {
        // Calls unwrap() because the database should always return a valid Pxid

        Self {
            follower_id: Pxid::from_str(&value.follower_id).unwrap(),
            followee_id: Pxid::from_str(&value.followee_id).unwrap(),
            created_at: value.created_at.and_utc(),
        }
    }
}

pub struct InsertUserFollowersDto {
    pub follower_id: Pxid,
    pub followee_id: Pxid,
}

#[derive(Debug, Default)]
pub struct UserFollowersFilter {
    pub follower_id: Option<Pxid>,
    pub followee_id: Option<Pxid>,
}

pub struct DeleteUserFollowersDto {
    pub follower_id: Pxid,
    pub followee_id: Pxid,
}

#[derive(Clone)]
pub struct UserFollowersRepository {
    db: Database,
}

impl UserFollowersRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    pub async fn insert(&self, dto: InsertUserFollowersDto) -> Result<UserFollowersRecord> {
        let active_model = entity::user_followers::ActiveModel {
            followee_id: Set(dto.followee_id.to_string()),
            follower_id: Set(dto.follower_id.to_string()),
            ..Default::default()
        };

        let model = active_model.insert(&*self.db).await.map_err(|err| {
            tracing::error!(%err, "Failed to insert into database");
            match err {
                sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    if let Some(db_err) = err.into_database_error() {
                        if let Some(constraint) = db_err.constraint() {
                            if constraint == FK_USER_FOLLOWERS_FOLLOWEE_USER {
                                tracing::error!(followee_id=%dto.followee_id, "Failed to find followee user");
                                return UserError::UserNotFound;
                            }

                            if constraint == FK_USER_FOLLOWERS_FOLLOWER_USER {
                                tracing::error!(follower_id=%dto.follower_id, "Failed to find follower user");
                                return UserError::UserNotFound;
                            }
                        }
                    }
                    UserError::DatabaseError
                }
                _ => UserError::DatabaseError,
            }
        })?;

        Ok(UserFollowersRecord::from(model))
    }

    pub async fn list(
        &self,
        pagination: Option<Pagination>,
        filter: Option<UserFollowersFilter>,
    ) -> Result<QuerySet<UserFollowersRecord>> {
        self.db
            .transaction::<_, QuerySet<UserFollowersRecord>, UserError>(|txn| {
                Box::pin(async move {
                    let mut query = entity::user_followers::Entity::find();

                    if let Some(filter) = filter {
                        if let Some(id) = filter.followee_id {
                            query = query.filter(
                                entity::user_followers::Column::FolloweeId.eq(id.to_string()),
                            );
                        }

                        if let Some(id) = filter.follower_id {
                            query = query.filter(
                                entity::user_followers::Column::FollowerId.eq(id.to_string()),
                            );
                        }
                    }

                    let count = query
                        .clone()
                        .select_only()
                        .count(txn)
                        .await
                        .map_err(|err| {
                            tracing::error!(%err, "Failed to count total followers");
                            UserError::DatabaseError
                        })?;

                    let pagination = pagination.unwrap_or_default();

                    let mut query = query.cursor_by(entity::user_followers::Column::FollowerId);

                    pagination.apply(&mut query);

                    let active_records = query.all(txn).await.map_err(|err| {
                        tracing::error!(%err, "Failed to retrieve users");
                        UserError::DatabaseError
                    })?;

                    if active_records.is_empty() {
                        return Ok(QuerySet::empty());
                    }

                    let records = active_records
                        .into_iter()
                        .map(UserFollowersRecord::from)
                        .collect();

                    Ok(QuerySet::new(records, count))
                })
            })
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to retrieve users");
                UserError::DatabaseError
            })
    }

    pub async fn delete(&self, follower_id: Pxid, followee_id: Pxid) -> Result<()> {
        self.db
            .transaction::<_, (), UserError>(|txn| {
                Box::pin(async move {
                    let maybe_model = entity::user_followers::Entity::find()
                        .filter(
                            entity::user_followers::Column::FollowerId.eq(follower_id.to_string()),
                        )
                        .filter(
                            entity::user_followers::Column::FolloweeId.eq(followee_id.to_string()),
                        )
                        .one(txn)
                        .await
                        .unwrap();

                    if let Some(model) = maybe_model {
                        model.delete(txn).await.map_err(|err| {
                            tracing::error!(%err, "Failed to delete user follow");
                            UserError::DatabaseError
                        })?;

                        return Ok(());
                    }

                    Err(UserError::UserFollowNotFound(FollowPeers {
                        follower_id,
                        followee_id,
                    }))
                })
            })
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to retrieve users");
                UserError::DatabaseError
            })
    }
}

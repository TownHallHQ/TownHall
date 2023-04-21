use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionError,
    TransactionTrait,
};

use quicklink::link::error::{LinkError, Result};
use quicklink::link::model::link::Link;
use quicklink::link::repository::{InsertLinkDto, LinkFilter, LinkRecord};

use crate::Database;

#[derive(Clone)]
pub struct LinkRepository {
    db: Database,
}

impl LinkRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    pub fn into_record(model: entity::link::Model) -> LinkRecord {
        LinkRecord {
            id: model.id,
            ulid: model.ulid,
            original_url: model.original_url,
            created_at: DateTime::from_utc(model.created_at, Utc),
            updated_at: DateTime::from_utc(model.updated_at, Utc),
            deleted_at: model.deleted_at.map(|naive| DateTime::from_utc(naive, Utc)),
        }
    }
}

#[async_trait]
impl quicklink::link::repository::LinkRepository for LinkRepository {
    async fn insert(&self, dto: InsertLinkDto) -> Result<LinkRecord> {
        self.db
            .transaction::<_, LinkRecord, LinkError>(|txn| {
                Box::pin(async move {
                    let active_model = entity::link::ActiveModel {
                        id: Set(Link::generate_id().to_string()),
                        ulid: Set(dto.ulid),
                        original_url: Set(dto.original_url),
                        ..Default::default()
                    };

                    let model = active_model.insert(txn).await.map_err(|err| {
                        // TODO: Handle duplicated ULID error
                        tracing::error!(%err, "Failed to insert into database");
                        LinkError::DatabaseError
                    })?;

                    let user_link_active_model = entity::user_links::ActiveModel {
                        link_id: Set(model.id.clone()),
                        user_id: Set(dto.owner_id),
                    };

                    user_link_active_model.insert(txn).await.map_err(|err| {
                        tracing::error!(%err, "Failed to insert into database");
                        LinkError::DatabaseError
                    })?;

                    Ok(LinkRepository::into_record(model))
                })
            })
            .await
            .map_err(|err| {
                tracing::error!(%err, "Insert Product Transaction Failed");
                match err {
                    TransactionError::Connection(_) => LinkError::DatabaseError,
                    TransactionError::Transaction(err) => err,
                }
            })
    }

    async fn find(&self, filter: Option<LinkFilter>) -> Result<Vec<LinkRecord>> {
        let mut query = entity::link::Entity::find();

        if let Some(filter) = filter {
            if let Some(id) = filter.id {
                query = query.filter(entity::link::Column::Id.eq(id.to_string()));
            }

            if let Some(ulid) = filter.ulid {
                query = query.filter(entity::link::Column::Ulid.eq(ulid.to_string()));
            }
        }

        let rows = query.all(&*self.db.0).await.map_err(|err| {
            tracing::error!(%err, "Failed to fetch from database");
            LinkError::DatabaseError
        })?;

        if rows.is_empty() {
            return Ok(Vec::default());
        }

        Ok(rows.into_iter().map(LinkRepository::into_record).collect())
    }

    async fn find_by_owner_id(&self, owner_id: Pxid) -> Result<Vec<LinkRecord>> {
        let rows = entity::user::Entity::find()
            .find_with_related(entity::link::Entity)
            .filter(entity::user_links::Column::UserId.eq(owner_id.to_string()))
            .all(&*self.db.0)
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to fetch from database");
                LinkError::DatabaseError
            })?;

        if rows.len() != 1 {
            return Ok(Vec::default());
        }

        let models = rows.first().unwrap().to_owned().1;
        let records = models
            .into_iter()
            .map(|link| LinkRepository::into_record(link))
            .collect();

        Ok(records)
    }
}

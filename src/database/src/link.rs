use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

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
        let active_model = entity::link::ActiveModel {
            id: Set(Link::generate_id().to_string()),
            ulid: Set(dto.ulid),
            original_url: Set(dto.original_url),
            ..Default::default()
        };

        let model = active_model.insert(&*self.db.0).await.map_err(|err| {
            // TODO: Handle duplicated ULID error
            tracing::error!(%err, "Failed to insert into database");
            LinkError::DatabaseError
        })?;

        Ok(LinkRepository::into_record(model))
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
}

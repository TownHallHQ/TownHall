use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sea_orm::{ActiveModelTrait, Set};

use crate::common::Database;
use crate::post::error::{PostError, Result};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PostRecord {
    pub id: String,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub head: bool,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertPostDto {
    pub id: String,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub head: bool,
    pub title: String,
    pub content: String,
}

#[derive(Clone)]
pub struct PostRepository {
    db: Database,
}

impl PostRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    pub fn into_record(model: entity::post::Model) -> PostRecord {
        PostRecord {
            id: model.id,
            author_id: model.author_id,
            parent_id: model.parent_id,
            head: model.head,
            title: model.title,
            content: model.content,
            created_at: DateTime::from_utc(model.created_at, Utc),
            updated_at: DateTime::from_utc(model.updated_at, Utc),
        }
    }

    pub async fn insert(&self, dto: InsertPostDto) -> Result<PostRecord> {
        let active_model = entity::post::ActiveModel {
            id: Set(dto.id),
            author_id: Set(dto.author_id),
            parent_id: Set(dto.parent_id),
            head: Set(dto.head),
            title: Set(dto.title),
            content: Set(dto.content),
            ..Default::default()
        };

        let model = active_model.insert(&*self.db).await.map_err(|err| {
            tracing::error!(%err, "Failed to insert into database");
            PostError::DatabaseError
        })?;

        Ok(PostRepository::into_record(model))
    }
}

use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, CursorTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::post::error::{PostError, Result};
use crate::shared::database::Database;
use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PostRecord {
    pub id: String,
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertPostDto {
    pub id: String,
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
}

#[derive(Clone)]
pub struct PostRepository {
    db: Database,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PostFilter {
    pub user_id: Option<Pxid>,
}

impl PostRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    pub fn into_record(model: entity::post::Model) -> PostRecord {
        PostRecord {
            id: model.id,
            author_id: Pxid::from_str(&model.author_id).unwrap(),
            parent_id: model.parent_id.map(|id| Pxid::from_str(&id).unwrap()),
            head: model.head,
            title: model.title,
            content: model.content,
            created_at: DateTime::from_utc(model.created_at, Utc),
            updated_at: DateTime::from_utc(model.updated_at, Utc),
        }
    }

    pub async fn list(
        &self,
        pagination: Option<Pagination>,
        filter: Option<PostFilter>,
    ) -> Result<QuerySet<PostRecord>> {
        self.db
            .transaction::<_, QuerySet<PostRecord>, PostError>(|txn| {
                Box::pin(async move {
                    let mut query = entity::post::Entity::find();

                    if let Some(filter) = filter {
                        if let Some(id) = filter.user_id {
                            query = query.filter(entity::post::Column::AuthorId.eq(id.to_string()));
                        }
                    }

                    let count = query
                        .clone()
                        .select_only()
                        .count(txn)
                        .await
                        .map_err(|err| {
                            tracing::error!(%err, "Failed to count total posts");
                            PostError::DatabaseError
                        })?;

                    let pagination = pagination.unwrap_or_default();
                    let mut query = query.cursor_by(entity::post::Column::Id);

                    pagination.apply(&mut query);

                    let active_records = query.all(txn).await.map_err(|err| {
                        tracing::error!(%err, "Failed to retrieve posts");
                        PostError::DatabaseError
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
                tracing::error!(%err, "Failed to retrieve posts");
                PostError::DatabaseError
            })
    }

    pub async fn insert(&self, dto: InsertPostDto) -> Result<PostRecord> {
        let active_model = entity::post::ActiveModel {
            id: Set(dto.id),
            author_id: Set(dto.author_id.to_string()),
            parent_id: Set(dto.parent_id.map(|id| id.to_string())),
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

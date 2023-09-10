use pxid::Pxid;

use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;

use super::error::Result;
use super::model::Post;
use super::repository::{InsertPostDto, PostFilter, PostRepository};

pub struct CreatePostDto {
    pub title: String,
    pub content: Option<String>,
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
}

pub struct CreateCommentDto {
    pub content: String,
    pub author_id: Pxid,
    pub parent_id: Pxid,
}

#[derive(Clone)]
pub struct PostService {
    repository: Box<PostRepository>,
}

impl PostService {
    pub fn new(repository: PostRepository) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    /// Retrieves posts in the platform
    pub async fn list(
        &self,
        pagination: Option<Pagination>,
        filter: Option<PostFilter>,
    ) -> Result<QuerySet<Post>> {
        let records = self.repository.list(pagination, filter).await?;
        let qs = records.inner_map(|record| Post::try_from(record).unwrap());

        Ok(qs)
    }

    pub async fn create(&self, dto: CreatePostDto) -> Result<Post> {
        let record = self
            .repository
            .insert(InsertPostDto {
                id: Post::generate_id()?.to_string(),
                author_id: dto.author_id,
                parent_id: dto.parent_id,
                head: dto.parent_id.is_none(),
                title: dto.title,
                content: dto.content,
            })
            .await?;

        let post = Post::try_from(record)?;

        Ok(post)
    }

    pub async fn create_comment(&self, dto: CreateCommentDto) -> Result<Post> {
        let record = self
            .repository
            .insert(InsertPostDto {
                id: Post::generate_id()?.to_string(),
                author_id: dto.author_id,
                parent_id: Some(dto.parent_id),
                head: false,
                title: "".to_string(),
                content: Some(dto.content),
            })
            .await?;

        let post = Post::try_from(record)?;

        Ok(post)
    }
}

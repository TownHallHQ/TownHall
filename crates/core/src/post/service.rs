use pxid::Pxid;

use super::error::Result;
use super::model::Post;
use super::repository::{InsertPostDto, PostRepository};

pub struct CreatePostDto {
    pub title: String,
    pub content: String,
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
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
}

use pxid::Pxid;

use super::error::Result;
use super::model::Post;
use super::repository::{InsertPostDto, PostRepository};

pub struct CreatePostDto {
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
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
                author_id: dto.author_id.to_string(),
                parent_id: dto.parent_id.map(|pxid| pxid.to_string()),
                head: dto.head,
                title: dto.title,
                content: dto.content,
            })
            .await?;

        let post = Post::try_from(record)?;

        Ok(post)
    }
}

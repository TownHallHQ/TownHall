use pxid::Pxid;

use super::error::Result;
use super::model::link::Link;
use super::repository::{LinkFilter, LinkRepository};

pub struct CreateLinkDto {
    pub original_url: String,
    pub fingerprint: Option<String>,
}

#[derive(Clone)]
pub struct LinkService<R: LinkRepository> {
    repository: Box<R>,
}

impl<R> LinkService<R>
where
    R: LinkRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    async fn create(&self, user_id: Pxid, dto: CreateLinkDto) -> Result<Link> {
        todo!()
    }

    async fn find(&self, filter: Option<LinkFilter>) -> Result<Vec<Link>> {
        todo!()
    }
}

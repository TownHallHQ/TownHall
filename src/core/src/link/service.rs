use std::str::FromStr;

use super::error::{LinkError, Result};
use super::model::link::Link;
use super::model::ulid::Ulid;
use super::repository::{LinkFilter, LinkRepository};

pub struct CreateLinkDto {
    pub original_url: String,
    pub ulid: Option<String>,
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

    pub async fn create(&self, dto: CreateLinkDto) -> Result<Link> {
        use super::repository::InsertLinkDto;

        let ulid = Self::handle_ulid_input(dto.ulid)?;
        let record = self
            .repository
            .insert(InsertLinkDto {
                user_id: String::default(),
                original_url: dto.original_url,
                ulid: ulid.to_string(),
            })
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to create Link instance");
                LinkError::DatabaseError
            })?;

        Link::try_from(record)
    }

    pub async fn find(&self, _filter: Option<LinkFilter>) -> Result<Vec<Link>> {
        todo!()
    }

    fn handle_ulid_input(s: Option<String>) -> Result<Ulid> {
        if let Some(ulid_str) = s {
            if ulid_str.is_empty() {
                return Ok(Ulid::default());
            }

            return Ulid::from_str(&ulid_str);
        }

        Ok(Ulid::default())
    }
}

use pxid::Pxid;

use super::error::Result;
use super::model::Chat;
use super::repository::{ChatRepository, InsertChatDto};

#[derive(Clone)]
pub struct ChatService<R: ChatRepository> {
    repository: Box<R>,
}

impl<R> ChatService<R>
where
    R: ChatRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    pub async fn create(&self, dto: InsertChatDto) -> Result<Chat> {
        let record = self.repository.insert(dto).await?;
        let chat = Chat::try_from(record)?;

        Ok(chat)
    }

    pub async fn list(&self, whoami: Pxid) -> Result<Vec<Chat>> {
        let records = self.repository.find_all(whoami).await?;
        let users = records
            .into_iter()
            .map_while(|record| Chat::try_from(record).ok())
            .collect();

        Ok(users)
    }
}

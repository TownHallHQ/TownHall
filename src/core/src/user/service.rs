use pxid::Pxid;

use super::error::Result;
use super::model::{Email, User};
use super::repository::UserRepository;

pub struct CreateUserDto {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: Box<R>,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    pub async fn find_by_email(&self, email: Email) -> Result<Option<User>> {
        if let Some(user_record) = self.repository.find_by_email(email).await? {
            let user = User::try_from(user_record)?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    pub async fn find_by_id(&self, id: Pxid) -> Result<Option<User>> {
        if let Some(user_record) = self.repository.find_by_id(id).await? {
            let user = User::try_from(user_record)?;

            return Ok(Some(user));
        }

        Ok(None)
    }
}

use pxid::Pxid;

use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;

use super::error::Result;
use super::model::{Email, Password, User, Username};
use super::repository::{InsertUserDto, UpdateUserDto, UserFilter, UserRepository};

pub struct CreateUserDto {
    pub name: String,
    pub surname: String,
    pub username: Username,
    pub email: Email,
    pub password: Password,
}

#[derive(Clone)]
pub struct UserService {
    repository: Box<UserRepository>,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    pub async fn create(&self, dto: CreateUserDto) -> Result<User> {
        let record = self
            .repository
            .insert(InsertUserDto {
                id: User::generate_id()?.to_string(),
                name: dto.name,
                surname: dto.surname,
                username: dto.username.to_string(),
                email: dto.email.to_string(),
                password_hash: dto.password.to_string(),
            })
            .await?;
        let user = User::try_from(record)?;

        Ok(user)
    }

    pub async fn list(
        &self,
        pagination: Option<Pagination>,
        filter: Option<UserFilter>,
    ) -> Result<QuerySet<User>> {
        let records = self.repository.list(pagination, filter).await?;

        let qs = records.inner_map(|record| User::try_from(record).unwrap());

        Ok(qs)
    }

    pub async fn update(&self, id: Pxid, dto: UpdateUserDto) -> Result<User> {
        let record = self.repository.update(id, dto).await?;

        let user = User::try_from(record)?;

        Ok(user)
    }
}

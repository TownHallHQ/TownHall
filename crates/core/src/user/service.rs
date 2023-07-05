use super::error::Result;
use super::model::{Email, Password, User, Username};
use super::repository::{InsertUserDto, UserFilter, UserRepository};

pub struct CreateUserDto {
    pub name: String,
    pub surname: String,
    pub username: Username,
    pub email: Email,
    pub password: Password,
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

    pub async fn find(&self, filter: Option<UserFilter>) -> Result<Vec<User>> {
        let records = self.repository.find(filter).await?;
        let users = records
            .into_iter()
            .map_while(|record| User::try_from(record).ok())
            .collect();

        Ok(users)
    }
}

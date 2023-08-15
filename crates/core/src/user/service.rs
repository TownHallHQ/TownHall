use pxid::Pxid;

use crate::image::model::UseCase;
use crate::image::service::{ImageProvider, ImageService, UploadImageDto};
use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;

use super::error::{Result, UserError};
use super::model::{Email, Password, User, Username};
use super::repository::{InsertUserDto, UpdateUserDto, UserFilter, UserRepository};

pub struct CreateUserDto {
    pub name: String,
    pub surname: String,
    pub username: Username,
    pub email: Email,
    pub password: Password,
}

pub struct UploadAvatarDto {
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct UserService<P: ImageProvider> {
    repository: Box<UserRepository>,
    image_service: ImageService<P>,
}

impl<P: ImageProvider> UserService<P> {
    pub fn new(repository: UserRepository, image_service: ImageService<P>) -> Self {
        Self {
            repository: Box::new(repository),
            image_service,
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

    pub async fn find_by_id(&self, id: Pxid) -> Result<Option<User>> {
        let qs = self
            .repository
            .list(
                Some(Pagination::first()),
                Some(UserFilter {
                    id: Some(id),
                    ..Default::default()
                }),
            )
            .await?;

        if let Some(record) = qs.first() {
            return Ok(Some(User::try_from(record.to_owned())?));
        }

        Ok(None)
    }

    /// Uploads a new avatar for the user. If the user already holds an avatar,
    /// this first deletes the current avatar and uploads a new one.
    pub async fn update_avatar(&self, id: Pxid, dto: UploadAvatarDto) -> Result<User> {
        self.image_service
            .validate(&dto.bytes, UseCase::Avatar)
            .map_err(|err| {
                tracing::warn!("Provided image is not valid for uploading as user's avatar");
                err
            })?;

        let Some(user) = self.find_by_id(id).await? else {
                return Err(UserError::UserNotFound);
            };

        if let Some(avatar_id) = user.avatar_id {
            // Delete current avatar before proceeding
            self.image_service.delete(avatar_id).await.map_err(|err| {
                tracing::error!(%err, "Failed to delete current avatar while uploading new");
                err
            })?;
        }

        let image = self
            .image_service
            .upload(UploadImageDto { bytes: dto.bytes }, UseCase::Avatar)
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to upload new user avatar");
                err
            })?;

        match self.repository.update_avatar(id, image.id).await {
            Ok(user_record) => User::try_from(user_record),
            Err(err) => Err(err),
        }
    }
}

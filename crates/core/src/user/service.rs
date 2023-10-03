use pxid::Pxid;

use crate::image::model::UseCase;
use crate::image::service::{ImageProvider, ImageService, UploadImageDto};
use crate::shared::pagination::Pagination;
use crate::shared::query_set::QuerySet;

use super::error::{Result, UserError};
use super::model::{Email, Password, User, Username};
use super::repository::follower::{
    InsertUserFollowersDto, UserFollowersFilter, UserFollowersRepository,
};
use super::repository::user::{InsertUserDto, UpdateUserDto, UserFilter, UserRepository};

/// Represents the association between two users where one follows the other
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FollowPeers {
    pub follower_id: Pxid,
    pub followee_id: Pxid,
}

impl From<super::repository::follower::UserFollowersRecord> for FollowPeers {
    fn from(value: super::repository::follower::UserFollowersRecord) -> Self {
        Self {
            follower_id: value.follower_id,
            followee_id: value.followee_id,
        }
    }
}

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
    user_followers: Box<UserFollowersRepository>,
    image_service: ImageService<P>,
}

impl<P: ImageProvider> UserService<P> {
    pub fn new(
        repository: UserRepository,
        user_followers: UserFollowersRepository,
        image_service: ImageService<P>,
    ) -> Self {
        Self {
            repository: Box::new(repository),
            user_followers: Box::new(user_followers),
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
        let maybe_user = self.repository.find_by_id(&id).await?;

        if let Some(record) = maybe_user {
            return Ok(Some(User::try_from(record)?));
        }

        Ok(None)
    }

    pub async fn find_by_email(&self, email: &Email) -> Result<Option<User>> {
        let maybe_user_record = self.repository.find_by_email(email).await?;

        if let Some(user_record) = maybe_user_record {
            return Ok(Some(User::try_from(user_record)?));
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

    /// Creates a new Follow relationship between 2 users
    ///
    /// This receives an instance of [`FollowPeers`] and retrieves another
    /// instance of [`FollowPeers`] which comes from the database.
    pub async fn follow(&self, dto: FollowPeers) -> Result<FollowPeers> {
        let followers = self
            .user_followers
            .insert(InsertUserFollowersDto {
                follower_id: dto.follower_id,
                followee_id: dto.followee_id,
            })
            .await?;

        Ok(FollowPeers {
            follower_id: followers.follower_id,
            followee_id: followers.followee_id,
        })
    }

    /// Deletes a Follow relationship between 2 users
    pub async fn unfollow(&self, dto: FollowPeers) -> Result<()> {
        self.user_followers
            .delete(dto.follower_id, dto.followee_id)
            .await
    }

    /// Retrieves a list (`Vec`) of relationships [`FollowPeers`] where the
    /// `followee` is always the same
    pub async fn followers(
        &self,
        followee_id: Pxid,
        pagination: Option<Pagination>,
    ) -> Result<QuerySet<FollowPeers>> {
        let records = self
            .user_followers
            .list(
                pagination,
                Some(UserFollowersFilter {
                    followee_id: Some(followee_id),
                    ..Default::default()
                }),
            )
            .await?;

        let qs = records.inner_map(FollowPeers::from);

        Ok(qs)
    }
}

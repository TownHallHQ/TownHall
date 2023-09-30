pub mod auth;
pub mod image;

use std::sync::Arc;

use anyhow::Result;

use playa::image::repository::ImageRepository;
use playa::image::service::ImageService;
use playa::post::repository::PostRepository;
use playa::post::service::PostService;
use playa::shared::database::Database;
use playa::user::repository::follower::UserFollowersRepository;
use playa::user::repository::user::UserRepository;
use playa::user::service::UserService;

use crate::config::Config;

use self::auth::AuthService;
use self::image::providers::ImageServiceProvider;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<AuthService>,
    pub user: Arc<UserService<ImageServiceProvider>>,
    pub post: Arc<PostService>,
    pub image: Arc<ImageService<ImageServiceProvider>>,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        let db_pool = Database::new(&config.database_url)
            .await
            .expect("Failed to create a new database pool");
        let auth_service = AuthService::new(&config.jwt_secret);
        let image_repository = ImageRepository::new(&db_pool);
        let image_provider = ImageServiceProvider::new(config).await?;
        let image_service = ImageService::new(image_repository, image_provider);
        let user_repository = UserRepository::new(&db_pool);
        let user_followers_repository = UserFollowersRepository::new(&db_pool);
        let user_service = UserService::new(
            user_repository,
            user_followers_repository,
            image_service.clone(),
        );
        let post_repository = PostRepository::new(&db_pool);
        let post_service = PostService::new(post_repository);

        Ok(Self {
            auth: Arc::new(auth_service),
            user: Arc::new(user_service),
            post: Arc::new(post_service),
            image: Arc::new(image_service),
        })
    }

    pub async fn shared(config: &Config) -> Result<SharedServices> {
        let service = Self::new(config).await?;

        Ok(Arc::new(service))
    }
}

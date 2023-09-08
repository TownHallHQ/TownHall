pub mod auth;
pub mod image;

use std::sync::Arc;

use playa::image::repository::ImageRepository;
use playa::image::service::{ImageProvider, ImageService};
use playa::post::repository::PostRepository;
use playa::post::service::PostService;
use playa::shared::database::Database;
use playa::user::repository::UserRepository;
use playa::user::service::UserService;

use crate::config::Config;

use self::auth::AuthService;

pub type SharedServices<P> = Arc<Services<P>>;

#[derive(Clone)]
pub struct Services<P: ImageProvider> {
    pub auth: Arc<AuthService>,
    pub user: Arc<UserService<P>>,
    pub post: Arc<PostService>,
    pub image: Arc<ImageService<P>>,
}

impl<P: ImageProvider + Clone> Services<P> {
    pub async fn new(config: &Config, image_provider: P) -> Self {
        let db_pool = Database::new(&config.database_url)
            .await
            .expect("Failed to create a new database pool");
        let auth_service = AuthService::new(&config.jwt_secret);
        let image_repository = ImageRepository::new(&db_pool);
        let image_service = ImageService::new(image_repository, image_provider);
        let user_repository = UserRepository::new(&db_pool);
        let user_service = UserService::new(user_repository, image_service.clone());
        let post_repository = PostRepository::new(&db_pool);
        let post_service = PostService::new(post_repository);

        Self {
            auth: Arc::new(auth_service),
            user: Arc::new(user_service),
            post: Arc::new(post_service),
            image: Arc::new(image_service),
        }
    }

    pub async fn shared(config: &Config, image_provider: P) -> SharedServices<P> {
        let service = Self::new(config, image_provider).await;

        Arc::new(service)
    }
}

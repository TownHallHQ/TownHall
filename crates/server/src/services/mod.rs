pub mod auth;

use std::sync::Arc;

use database::Database;
use gabble::user::service::UserService;

use crate::config::Config;

use self::auth::AuthService;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<AuthService>,
    pub user: Arc<UserService<database::user::UserRepository>>,
}

impl Services {
    pub async fn new(config: &Config) -> Self {
        let db_pool = Database::new(&config.database_url)
            .await
            .expect("Failed to create a new database pool");
        let auth_service = AuthService::new(&config.jwt_secret);
        let user_repository = database::user::UserRepository::new(&db_pool);
        let user_service = UserService::new(user_repository);

        Self {
            auth: Arc::new(auth_service),
            user: Arc::new(user_service),
        }
    }

    pub async fn shared(config: &Config) -> SharedServices {
        let service = Self::new(config).await;

        Arc::new(service)
    }
}

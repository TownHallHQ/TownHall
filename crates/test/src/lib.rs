use std::sync::Arc;

use anyhow::Result;
use pxid::Pxid;

use libserver::config::Config;
use libserver::context::Context;
use libserver::graphql::schema::{build_schema_with_context, GraphQLSchema};
use libserver::services::auth::Token;
use migration::{Migrator, MigratorTrait};
use playa::shared::database::Database;

pub const TEST_ADMIN_EMAIL: &str = "admin@whizzes.io";
pub const TEST_ADMIN_PASSWORD: &str = "R00tP@ssw0rd";
pub const TEST_JWT_SECRET: &str = "test-jwt-value";

#[cfg(test)]
mod modules;

pub struct TestUtil {
    pub context: Arc<Context>,
    pub db: Database,
    pub schema: GraphQLSchema,
}

impl TestUtil {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        let config = Config::new();
        let context = Context::new(&config).await?;
        let context = Arc::new(context);
        let schema = build_schema_with_context(&context);
        let db = Database::new(config.database_url.as_str())
            .await
            .expect("Failed to connect to DB");

        Ok(Self {
            context,
            db,
            schema,
        })
    }

    /// Creates a new instance of [`TestUtil`] and clears the database.
    /// This is equivalent to calling
    ///
    /// ```ignore
    /// use crate::TestUtil;
    ///
    /// let test_util = TestUtil::new().await;
    ///
    /// test_util.clear_db().await;
    /// ```
    ///
    pub async fn new_cleared() -> Self {
        let test_util = Self::new().await.expect("Failed to create TestUtil");

        test_util.clear_db().await;

        test_util
    }

    pub async fn clear_db(&self) {
        Migrator::fresh(&*self.db).await.unwrap();
    }

    pub fn parts(&self) -> (Arc<Context>, GraphQLSchema) {
        (Arc::clone(&self.context), self.schema.clone())
    }

    pub async fn token_create(&self, uid: Pxid) -> Token {
        self.context.services.auth.sign_token(uid).unwrap()
    }
}

use std::ops::Deref;
use std::sync::Arc;

use sea_orm::{ConnectOptions, Database as SeaORMDatabase, DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct Database(Arc<DatabaseConnection>);

impl Deref for Database {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DatabaseConnection> for Database {
    fn from(value: DatabaseConnection) -> Self {
        let db_conn = Arc::new(value);

        Database(db_conn)
    }
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let db_opts = ConnectOptions::new(database_url.to_owned());
        let db_conn = SeaORMDatabase::connect(db_opts).await?;
        let db_conn = Arc::new(db_conn);

        Ok(Self(db_conn))
    }
}

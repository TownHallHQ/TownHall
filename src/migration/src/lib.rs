pub use sea_orm_migration::prelude::*;

mod m20230408_000001_create_table_user;
mod m20230408_000002_create_table_link;
mod m20230408_000003_create_table_user_links;

pub const PXID_LENGTH: u32 = 30;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230408_000001_create_table_user::Migration),
            Box::new(m20230408_000002_create_table_link::Migration),
            Box::new(m20230408_000003_create_table_user_links::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20230408_000001_create_table_user;
mod m20230704_000001_create_table_post;
mod m20230807_000001_create_table_image;
mod m20230813_000001_add_column_avatar_id_user;

pub const PXID_LENGTH: u32 = 30;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230408_000001_create_table_user::Migration),
            Box::new(m20230704_000001_create_table_post::Migration),
            Box::new(m20230807_000001_create_table_image::Migration),
            Box::new(m20230813_000001_add_column_avatar_id_user::Migration),
        ]
    }
}

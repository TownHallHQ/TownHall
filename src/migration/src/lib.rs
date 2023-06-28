pub use sea_orm_migration::prelude::*;

mod m20230408_000001_create_table_user;
mod m20230621_000001_create_table_chat;
mod m20230621_000002_create_table_message;
mod m20230621_000003_create_table_user_chats;
mod m20230627_000001_create_table_threads;
mod m20230627_000002_create_table_post;

pub const PXID_LENGTH: u32 = 30;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230408_000001_create_table_user::Migration),
            Box::new(m20230621_000001_create_table_chat::Migration),
            Box::new(m20230621_000002_create_table_message::Migration),
            Box::new(m20230621_000003_create_table_user_chats::Migration),
            Box::new(m20230627_000001_create_table_threads::Migration),
            Box::new(m20230627_000002_create_table_post::Migration),
        ]
    }
}

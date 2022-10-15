mod m20220915_153744_create_links_table;
mod m20221015_100421_create_users_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221015_100421_create_users_table::Migration),
            Box::new(m20220915_153744_create_links_table::Migration),
        ]
    }
}

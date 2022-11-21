use sea_orm_migration::prelude::*;

use super::m20220915_153744_create_links_table::Link;
use super::m20221015_100421_create_users_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // https://docs.rs/sea-orm-migration/0.9.2/sea_orm_migration/prelude/struct.ColumnDef.html
        manager
            .alter_table(
                Table::alter()
                    .table(Link::Table)
                    .add_column(ColumnDef::new(User::OwnerId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("FK_link_owner")
                    .from_tbl(User::Table)
                    .from_col(User::Id)
                    .to_tbl(Link::Table)
                    .to_col(Link::OwnerId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Link::Table).to_owned())
            .await
    }
}

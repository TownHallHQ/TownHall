use sea_orm_migration::prelude::*;

use super::m20220915_153744_create_links_table::Link;
use super::m20221015_100421_create_users_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub const LINK_OWNER_FK: &str = "FK_link_owner";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // https://docs.rs/sea-orm-migration/0.9.2/sea_orm_migration/prelude/struct.ColumnDef.html
        manager
            .alter_table(
                Table::alter()
                    .table(Link::Table)
                    .add_column(ColumnDef::new(Link::OwnerId).integer())
                    .to_owned(),
            )
            .await?;

        let fk_link_owner = TableForeignKey::new()
            .name(LINK_OWNER_FK)
            .from_col(Link::OwnerId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Link::Table)
                    .add_foreign_key(&fk_link_owner)
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

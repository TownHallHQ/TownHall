use sea_orm_migration::prelude::*;

use crate::m20230807_000001_create_table_image::Image;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::AvatarId).string_len(30).unique_key())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("FK_user_avatar_id")
                            .from_col(User::AvatarId)
                            .to_tbl(Image::Table)
                            .to_col(Image::Id)
                            .on_delete(ForeignKeyAction::SetDefault)
                            .on_update(ForeignKeyAction::SetDefault),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .drop_column(User::AvatarId)
                    .table(User::Table)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    AvatarId,
}

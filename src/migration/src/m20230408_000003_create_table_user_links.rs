use sea_orm_migration::prelude::*;

use crate::m20230408_000001_create_table_user::User;
use crate::m20230408_000002_create_table_link::Link;
use crate::PXID_LENGTH;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLinks::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("PK_user_links")
                            .col(UserLinks::UserId)
                            .col(UserLinks::LinkId),
                    )
                    .col(
                        ColumnDef::new(UserLinks::UserId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserLinks::LinkId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_links_user_id")
                            .from(UserLinks::Table, UserLinks::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_links_link_id")
                            .from(UserLinks::Table, UserLinks::LinkId)
                            .to(Link::Table, Link::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserLinks {
    Table,
    UserId,
    LinkId,
}

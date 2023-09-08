use sea_orm_migration::prelude::*;

use crate::{m20230408_000001_create_table_user::User, PXID_LENGTH};

const MAX_TITLE_CONTENT_LENGTH: u32 = 150;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .string_len(PXID_LENGTH)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Post::AuthorId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Post::ParentId).string_len(PXID_LENGTH))
                    .col(ColumnDef::new(Post::Head).boolean().not_null())
                    .col(
                        ColumnDef::new(Post::Title)
                            .string_len(MAX_TITLE_CONTENT_LENGTH)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Post::Content).string().null())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_post_user_id")
                            .from(Post::Table, Post::AuthorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    AuthorId,
    ParentId,
    Head,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
}

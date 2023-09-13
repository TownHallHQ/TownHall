use sea_orm_migration::prelude::*;

use crate::{m20230408_000001_create_table_user::User, PXID_LENGTH};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Followers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Followers::Follower)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Followers::Followee)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Followers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_follower_user")
                            .from(Followers::Table, Followers::Follower)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_followee_user")
                            .from(Followers::Table, Followers::Followee)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

          let follower_followee_index = Index::create()
            .name("IDX_follower_followee")
            .table(Followers::Table)
            .col(Followers::Follower)
            .col(Followers::Followee)
            .unique()
            .to_owned();

          manager.create_index(follower_followee_index).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Followers::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Followers {
    Table,
    Follower,
    Followee,
    CreatedAt,
}

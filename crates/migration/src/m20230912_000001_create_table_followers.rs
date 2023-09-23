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
                    .table(UserFollowers::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("PK_user_followers_follower_followee")
                            .col(UserFollowers::FollowerId)
                            .col(UserFollowers::FolloweeId),
                    )
                    .col(
                        ColumnDef::new(UserFollowers::FollowerId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowers::FolloweeId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_followers_follower_user")
                            .from(UserFollowers::Table, UserFollowers::FollowerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_followers_followee_user")
                            .from(UserFollowers::Table, UserFollowers::FolloweeId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFollowers::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum UserFollowers {
    Table,
    FollowerId,
    FolloweeId,
    CreatedAt,
}

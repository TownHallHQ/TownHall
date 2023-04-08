use sea_orm_migration::prelude::*;

use crate::PXID_LENGTH;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .string_len(PXID_LENGTH)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string_len(256).not_null())
                    .col(ColumnDef::new(User::Surname).string_len(256).not_null())
                    .col(ColumnDef::new(User::Email).string_len(256).unique_key())
                    .col(
                        ColumnDef::new(User::PasswordHash)
                            .string_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(ColumnDef::new(User::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    Surname,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

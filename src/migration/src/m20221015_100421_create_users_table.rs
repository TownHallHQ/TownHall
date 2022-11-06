use sea_orm_migration::prelude::*;

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    LastName,
    Email,
    Hash,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // https://docs.rs/sea-orm-migration/0.9.2/sea_orm_migration/prelude/struct.ColumnDef.html
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string()
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::LastName)
                            .string()
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .string_len(512)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Hash)
                            .string()
                            .string_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
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

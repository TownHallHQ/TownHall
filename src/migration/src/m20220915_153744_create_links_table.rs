use sea_orm_migration::prelude::*;

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Link {
    Table,
    Id,
    Hash,
    OriginalUrl,
    OwnerId,
    ExpiresAt,
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
                    .table(Link::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Link::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Link::Hash)
                            .string()
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Link::OriginalUrl)
                            .string()
                            .string_len(512)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Link::ExpiresAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Link::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Link::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
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

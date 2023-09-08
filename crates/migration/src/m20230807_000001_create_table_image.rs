use sea_orm_migration::prelude::*;

use playa::image::model::{IMAGE_JPEG, IMAGE_PNG};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut create_image_mime_type = sea_query::extension::postgres::Type::create();

        create_image_mime_type
            .as_enum(MimeType::Type)
            .values(vec![MimeType::Jpeg, MimeType::Png]);

        manager.create_type(create_image_mime_type).await?;

        let mut create_image_use_case = sea_query::extension::postgres::Type::create();

        create_image_use_case
            .as_enum(UseCase::Type)
            .values(vec![UseCase::Avatar, UseCase::Post]);

        manager.create_type(create_image_use_case).await?;

        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Image::Id)
                            .string_len(30)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Image::Height).integer().not_null())
                    .col(ColumnDef::new(Image::Width).integer().not_null())
                    .col(ColumnDef::new(Image::Url).string_len(512).not_null())
                    .col(ColumnDef::new(Image::ThumbnailUrl).string_len(512))
                    .col(ColumnDef::new(Image::Size).integer().not_null())
                    .col(
                        ColumnDef::new(Image::MimeType)
                            .enumeration(MimeType::Type, vec![MimeType::Jpeg, MimeType::Png])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Image::UseCase)
                            .enumeration(UseCase::Type, vec![UseCase::Avatar, UseCase::Post])
                            .not_null(),
                    )
                    .col(ColumnDef::new(Image::ProviderId).string_len(256).not_null())
                    .col(
                        ColumnDef::new(Image::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(
                        ColumnDef::new(Image::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await?;

        let mut drop_image_mime_type_type_stmt = sea_query::extension::postgres::Type::drop();

        drop_image_mime_type_type_stmt.name(MimeType::Type);

        manager.drop_type(drop_image_mime_type_type_stmt).await?;

        let mut drop_image_use_case_type_stmt = sea_query::extension::postgres::Type::drop();

        drop_image_use_case_type_stmt.name(UseCase::Type);

        manager.drop_type(drop_image_use_case_type_stmt).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Image {
    Table,
    Id,
    Height,
    Width,
    Url,
    ThumbnailUrl,
    Size,
    MimeType,
    UseCase,
    ProviderId,
    CreatedAt,
    UpdatedAt,
}

pub enum MimeType {
    Type,
    Jpeg,
    Png,
}

impl Iden for MimeType {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "image_mime_type",
                Self::Jpeg => IMAGE_JPEG,
                Self::Png => IMAGE_PNG,
            }
        )
        .unwrap();
    }
}

pub enum UseCase {
    Type,
    Avatar,
    Post,
}

impl Iden for UseCase {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "image_use_case",
                Self::Avatar => "avatar",
                Self::Post => "post",
            }
        )
        .unwrap();
    }
}

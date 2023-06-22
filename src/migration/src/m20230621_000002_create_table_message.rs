use sea_orm_migration::prelude::*;

use crate::PXID_LENGTH;

const MAX_MESSAGE_CONTENT_LENGTH: u32 = 2048;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut create_type_stmt = sea_query::extension::postgres::Type::create();

        create_type_stmt
            .as_enum(MessageKind::Type)
            .values(vec![MessageKind::Text, MessageKind::Image]);

        manager.create_type(create_type_stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .string_len(PXID_LENGTH)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Message::Kind)
                            .enumeration(
                                MessageKind::Type,
                                vec![MessageKind::Text, MessageKind::Image],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::Content)
                            .string_len(MAX_MESSAGE_CONTENT_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(
                        ColumnDef::new(Message::UpdatedAt)
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
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;

        let mut drop_type_stmt = sea_query::extension::postgres::Type::drop();

        drop_type_stmt.name(MessageKind::Type);

        manager.drop_type(drop_type_stmt).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Message {
    Table,
    Id,
    Kind,
    Content,
    CreatedAt,
    UpdatedAt,
}

pub enum MessageKind {
    Type,
    Text,
    Image,
}

impl Iden for MessageKind {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "message_kind",
                Self::Text => "text",
                Self::Image => "image",
            }
        )
        .unwrap();
    }
}

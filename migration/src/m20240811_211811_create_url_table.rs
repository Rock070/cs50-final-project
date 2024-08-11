use crate::tables::{Urls, Users};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Urls::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Urls::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(Urls::Url)
                        .string()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(Urls::ShortUrl)
                        .string()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(Urls::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(Urls::UpdatedAt)
                        .timestamp()
                    )
                    .col(
                        ColumnDef::new(Urls::UserId)
                        .uuid()
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_user_id")
                        .from(Urls::Table, Urls::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Urls::Table).to_owned())
            .await
    }
}
use crate::tables::{Urls, RequestRecords};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RequestRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RequestRecords::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(RequestRecords::Ip)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(RequestRecords::UserAgent)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(RequestRecords::Referer)
                            .string()
                    )
                    .col(
                        ColumnDef::new(RequestRecords::UrlId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(RequestRecords::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_url_id")
                            .from(RequestRecords::Table, RequestRecords::UrlId)
                            .to(Urls::Table, Urls::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RequestRecords::Table).to_owned())
            .await
    }
}

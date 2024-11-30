use sea_orm_migration::{prelude::*, schema::*};
use crate::tables::RequestRecords;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RequestRecords::Table)
                    .rename_column(RequestRecords::Referer, RequestRecords::Origin)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RequestRecords::Table)
                    .rename_column(RequestRecords::Origin, RequestRecords::Referer)
                    .to_owned(),
            )
            .await
    }
}

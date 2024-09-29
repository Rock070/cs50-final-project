use sea_orm_migration::{prelude::*, schema::*};
use crate::tables::Urls;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Urls::Table)
                    .add_column(
                        ColumnDef::new(Urls::IsDelete)
                        .boolean()
                        .not_null()
                        .default(false)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Urls::Table)
                    .drop_column(Urls::IsDelete)
                    .to_owned(),
            )
            .await
    }
}
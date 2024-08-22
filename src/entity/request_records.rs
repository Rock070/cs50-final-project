//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "request_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub ip: String,
    pub user_agent: String,
    pub origin: Option<String>,
    pub url_id: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::urls::Entity",
        from = "Column::UrlId",
        to = "super::urls::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Urls,
}

impl Related<super::urls::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Urls.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Password,
    Email,
    CreatedAt,
    UpdatedAt,
}


#[derive(DeriveIden)]
pub enum Urls {
    Table,
    Id,
    Url,
    ShortUrl,
    CreatedAt,
    UpdatedAt,
    UserId,
}

#[derive(DeriveIden)]
pub enum RequestRecords {
    Table,
    Id,
    Ip,
    UserAgent,
    Referer,
    UrlId,
    CreatedAt,
}

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Database};
use crate::entity::urls;

use axum::{
    http::StatusCode,
    extract::Path,
    response::{Redirect, IntoResponse},
};

pub async fn redirect_hash_url(Path(path): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    // validate path only include base62 characters [0-9a-zA-Z]
    if !is_valid_path(&path) {
        return Err((StatusCode::BAD_REQUEST, format!("Invalid path: {}", path)));
    }

    let database = Database::connect("postgres://rock:rock0702@localhost:5432/shor")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // redirect to the original url
    let column = urls::Entity::find()
        .filter(urls::Column::ShortUrl.eq(path))
        .one(&database)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(model) = column {
        return Ok(Redirect::temporary(&model.url));
    }

    Err((StatusCode::NOT_FOUND, "Not found".to_string()))
}



fn is_valid_path(path: &str) -> bool {
    path.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}
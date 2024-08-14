use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use crate::{
    entity::urls,
    application::AppState
};

use axum::{
    http::StatusCode,
    extract::{Path, State},
    response::{Redirect, IntoResponse},
};

pub async fn redirect_hash_url(state: State<AppState>, Path(path): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    // validate path only include base62 characters [0-9a-zA-Z]
    if !is_valid_path(&path) {
        return Err((StatusCode::BAD_REQUEST, format!("Invalid path: {}", path)));
    }

    let column = urls::Entity::find()
        .filter(urls::Column::ShortUrl.eq(path))
        .one(&state.database)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(model) = column {
        return Ok(Redirect::temporary(&model.url));
    }

    Err((StatusCode::NOT_FOUND, "Not found".to_string()))
}

// TODO: testing
fn is_valid_path(path: &str) -> bool {
    path.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_path() {
        assert_eq!(is_valid_path("abc"), true);
        assert_eq!(is_valid_path("a1b2"), true);
        assert_eq!(is_valid_path("a1b2!"), false);
    }
}
use crate::{application::AppState, entity::urls};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

pub async fn redirect_hash_url(
    state: State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
    path.chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
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

use crate::{application::AppState, entity::{urls, request_records}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, ActiveValue};
use axum_extra::{
    TypedHeader,
    headers::{UserAgent, Origin},
};
use uuid::Uuid;

use std::net::SocketAddr;

use axum::{
    extract::{Path, State, ConnectInfo},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

pub async fn redirect_hash_url(
    state: State<AppState>,
    header_user_agent: Option<TypedHeader<UserAgent>>,
    header_origin: Option<TypedHeader<Origin>>,
    ConnectInfo(header_connect_info): ConnectInfo<SocketAddr>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
     // Log the additional informationq
    let connect_info = header_connect_info.clone();
    let user_agent = if let Some(user_agent) = header_user_agent { user_agent.clone().to_string() } else {"".to_string()};
    let origin = if let Some(origin) = header_origin { origin.clone().to_string() } else { "".to_string() };

    if !is_valid_path(&path) {
        return Err((StatusCode::BAD_REQUEST, format!("Invalid path: {}", path)));
    }

    let column = urls::Entity::find()
        .filter(urls::Column::ShortUrl.eq(path))
        .one(&state.database)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(model) = column {
        let request_record = request_records::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            user_agent: ActiveValue::set(user_agent.clone()),
            origin: ActiveValue::set(Some(origin.clone())),
            ip: ActiveValue::set(connect_info.to_string()),
            url_id: ActiveValue::set(model.id),
            ..Default::default()
        };

        request_records::Entity::insert(request_record)
            .exec(&state.database)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Ok(Redirect::temporary(&model.url));
    }

    Err((StatusCode::NOT_FOUND, "Not found".to_string()))
}

/**
 * validate path only include base62 characters [0-9a-zA-Z]
 */
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

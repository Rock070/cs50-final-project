use crate::{
    application::AppState,
    domain::HashUrlBody,
    entity::{urls, users},
};

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use base62::encode;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use jsonwebtoken::errors::ErrorKind;

pub async fn hash_url(
    state: State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
    Json(data): Json<HashUrlBody>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let url_body = HashUrlBody::new(data.url).map_err(|e| {
        let error_message = format!("[無效的 URL] {}", e);
        (StatusCode::BAD_REQUEST, error_message)
    })?;

    let mut user_id = String::new();

    if let Some(authorization_header) = authorization {
        let token = authorization_header.token().to_string();

        let claim = match state
            .jwt_handler
            .clone()
            .decode_token(&state.application, token)
        {
            Ok(claim) => claim,
            Err(err) => {
                if err.into_kind() == ErrorKind::ExpiredSignature {
                    return Ok((StatusCode::UNAUTHORIZED, "token is expired".to_string()));
                }

                return Ok((StatusCode::UNAUTHORIZED, "token is invalid".to_string()));
            }
        };

        let user_name = claim.aud.clone();

        if !user_name.is_empty() {
            let users_column = users::Entity::find()
                .filter(users::Column::Username.eq(&user_name))
                .one(&state.database)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "token is invalid".to_string()))?;

            let user = match users_column {
                Some(user) => user,
                None => return Ok((StatusCode::UNAUTHORIZED, "token is invalid".to_string())),
            };

            user_id = user.id.to_string();

            let urls_column = urls::Entity::find()
                .filter(
                    urls::Column::Url
                        .eq(url_body.url.clone())
                        .and(urls::Column::UserId.eq(Uuid::parse_str(&user_id).unwrap())),
                )
                .one(&state.database)
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            if let Some(url) = urls_column {
                return Ok((StatusCode::OK, format!("縮短的 URL 是: {}", url.short_url)));
            }
        }
    } else {
        let urls_column = urls::Entity::find()
            .filter(
                urls::Column::Url
                    .eq(&url_body.url)
                    .and(urls::Column::UserId.is_null()),
            )
            .one(&state.database)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        if let Some(url) = urls_column {
            return Ok((StatusCode::OK, format!("縮短的 URL 是: {}", url.short_url)));
        }
    }

    // 使用 SHA-256 對 URL 進行哈希
    let mut hasher = Sha256::new();
    let raw_url = format!("{}{}", user_id.clone(), url_body.url);
    hasher.update(raw_url.as_bytes());
    let result = hasher.finalize();

    // 將哈希結果轉換為 u64
    // 取 sha-256 結果的前 8 個 bytes，使用 try_into 宣告為長度為 8 的 u8 陣列，
    // 再使用 from_be_bytes 轉換為 u64
    let num = u64::from_be_bytes(result[..8].try_into().unwrap());

    // 使用 base62 編碼
    let short_url = encode(num);

    let user_id = if user_id.is_empty() {
        None
    } else {
        Some(Uuid::parse_str(&user_id).unwrap())
    };

    let url = urls::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        url: ActiveValue::Set(url_body.url),
        user_id: if user_id.is_some() {
            ActiveValue::Set(user_id)
        } else {
            ActiveValue::NotSet
        },
        short_url: ActiveValue::Set(short_url.clone()),
        updated_at: ActiveValue::NotSet,
        ..Default::default()
    };

    urls::Entity::insert(url)
        .exec(&state.database)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::OK, format!("縮短的 URL 是: {}", short_url)))
}

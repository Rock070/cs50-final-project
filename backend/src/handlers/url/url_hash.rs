use crate::{
    application::AppState,
    domain::Url,
    entity::{urls, users},
    AppError, AppHttpResponse, BadRequestError, HttpResponseCode,
};

use axum::{extract::State, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use base62::encode;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct HashUrlRequest {
    url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HashUrlResponse {
    url: String,
}

/// 2.1.1.1 產生短網址
#[utoipa::path(
    post,
    path = "/api/url/hash",
    tag = "url",
    operation_id = "url-hash",
    request_body(
        content = HashUrlRequest,
        description = "Hash URL request",
        content_type = "application/json",
        example = json!({"url": "https://www.example.com"})
    ),
    responses(
        (status = 200, description = HttpResponseCode::Success.to_message(), body = AppHttpResponseHashUrlResponse, example = json!({"message": HttpResponseCode::Success.to_message(), "code": HttpResponseCode::Success.to_str(), "data": {"url": "https://shor.cc/Qhq1TQ2nVI"}})),
        (status = 400, description = HttpResponseCode::BadRequest.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
        (status = 401, description = HttpResponseCode::Unauthorized.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::Unauthorized.to_message(), "code": HttpResponseCode::Unauthorized.to_str(), "data": null})),
    )
)]
pub async fn url_hash(
    state: State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
    Json(data): Json<HashUrlRequest>,
) -> Result<Json<AppHttpResponse<HashUrlResponse>>, AppError> {
    let new_url = Url::try_from(data)?.0;

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
                    return Err(AppError::UnauthorizedError("token is expired".to_string()));
                }

                return Err(AppError::UnauthorizedError("token is invalid".to_string()));
            }
        };

        let user_name = claim.aud.clone();

        if !user_name.is_empty() {
            let users_column = users::Entity::find()
                .filter(users::Column::Username.eq(&user_name))
                .one(&state.database)
                .await?;

            let user = match users_column {
                Some(user) => user,
                None => return Err(AppError::UnauthorizedError("token is invalid".to_string())),
            };

            user_id = user.id.to_string();

            let urls_column = urls::Entity::find()
                .filter(
                    urls::Column::Url
                        .eq(&new_url)
                        .and(urls::Column::UserId.eq(Uuid::parse_str(&user_id).unwrap()).and(urls::Column::IsDelete.eq(false))),
                )
                .one(&state.database)
                .await?;

            if let Some(url) = urls_column {
                return Ok(Json(AppHttpResponse::new(
                    HttpResponseCode::Success.to_message().to_string(),
                    HttpResponseCode::Success.to_str().to_string(),
                    Some(HashUrlResponse {
                        url: format!("{}{}", &state.application.base_url, &url.short_url),
                    }),
                )));
            }
        }
    } else {
        let urls_column = urls::Entity::find()
            .filter(
                urls::Column::Url
                    .eq(&new_url)
                    .and(urls::Column::UserId.is_null()),
            )
            .one(&state.database)
            .await?;

        if let Some(url) = urls_column {
            return Ok(Json(AppHttpResponse::new(
                HttpResponseCode::Success.to_message().to_string(),
                HttpResponseCode::Success.to_str().to_string(),
                Some(HashUrlResponse {
                    url: format!("{}{}", &state.application.base_url, &url.short_url),
                }),
            )));
        }
    }

    // 使用 SHA-256 對 URL 進行哈希
    let mut hasher = Sha256::new();
    let raw_url = format!("{}{}", user_id.clone(), &new_url);
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
        url: ActiveValue::Set(new_url),
        user_id: if user_id.is_some() {
            ActiveValue::Set(user_id)
        } else {
            ActiveValue::NotSet
        },
        short_url: ActiveValue::Set(short_url.clone()),
        updated_at: ActiveValue::NotSet,
        ..Default::default()
    };

    urls::Entity::insert(url).exec(&state.database).await?;

    Ok(Json(AppHttpResponse::new(
        HttpResponseCode::Success.to_message().to_string(),
        HttpResponseCode::Success.to_str().to_string(),
        Some(HashUrlResponse {
            url: format!("{}{}", &state.application.base_url, &short_url),
        }),
    )))
}

impl TryFrom<HashUrlRequest> for Url {
    type Error = BadRequestError;

    fn try_from(value: HashUrlRequest) -> Result<Url, BadRequestError> {
        let url = Url::parse_url(value.url.unwrap_or_default())?;

        Ok(url)
    }
}

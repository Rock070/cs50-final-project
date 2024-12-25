use crate::{
    application::AppState,
    entity::{request_records, urls, users},
    AppError, AppHttpResponse, HttpResponseCode,
};

use axum::{extract::State, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use jsonwebtoken::errors::ErrorKind;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize)]
pub struct UserUrl {
    pub id: String,
    pub short_url: String,
    pub url: String,
    pub created_at: String,
    pub request_count: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserUrlsResponse {
    pub urls: Vec<UserUrl>,
}

/// 1.1.1.4 取得使用者短網址
#[utoipa::path(
  get,
  path = "/api/user/urls",           
  tag = "user",
  operation_id = "user_urls",
  security(
    ("bearer_auth" = [])
  ),
  responses(
      (status = 200, description = HttpResponseCode::Success.to_message(), body = AppHttpResponseUserUrlsResponse, example = json!({"message": HttpResponseCode::Success.to_message(), "code": HttpResponseCode::Success.to_str(), "data": {"urls": [{"id": "206eded9-628e-4d11-b5da-ab2573f8000e", "short_url": "http://localhost:3000/A3mrImkt6DR", "url": "https://google.com", "created_at": "2024-12-25 16:50:44.698372", "request_count": 3}]}})),
      (status = 400, description = HttpResponseCode::BadRequest.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
      (status = 401, description = HttpResponseCode::Unauthorized.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::Unauthorized.to_message(), "code": HttpResponseCode::Unauthorized.to_str(), "data": null})),
  )
)]
pub async fn user_urls(
    state: State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<Json<AppHttpResponse<UserUrlsResponse>>, AppError> {
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

            let urls = urls::Entity::find()
                .filter(
                    urls::Column::UserId
                        .eq(Some(user.id))
                        .and(urls::Column::IsDelete.eq(false)),
                )
                .all(&state.database)
                .await?;

            let mut user_urls = Vec::new();

            for url in urls {
                let url_request_count = request_records::Entity::find()
                    .filter(request_records::Column::UrlId.eq(url.id))
                    .all(&state.database)
                    .await?;

                user_urls.push(UserUrl {
                    id: url.id.to_string(),
                    short_url: format!("{}{}", &state.application.base_url, &url.short_url),
                    url: url.url.to_string(),
                    created_at: url.created_at.to_string(),
                    request_count: url_request_count.len() as u64,
                });
            }
            return Ok(Json(AppHttpResponse::new(
                HttpResponseCode::Success.to_message().to_string(),
                HttpResponseCode::Success.to_str().to_string(),
                Some(UserUrlsResponse { urls: user_urls }),
            )));
        }
    }

    return Err(AppError::UnauthorizedError(
        HttpResponseCode::Unauthorized.to_message().to_string(),
    ));
}

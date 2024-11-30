use crate::{
  application::AppState,
  entity::{users, urls},
  AppError, AppHttpResponse, HttpResponseCode,
  BadRequestError,
};
use uuid::Uuid;
use axum::{extract::State, Json};
use axum_extra::{
  headers::{authorization::Bearer, Authorization},
  TypedHeader,
};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use jsonwebtoken::errors::ErrorKind;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteUrlRequest {
    id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteUrlResponse {
    pub id: String,
}

/// 2.1.1.2 刪除短網址
#[utoipa::path(
  delete,
  path = "/api/url",           
  tag = "url",
  operation_id = "delete-url",
  security(
    ("bearer_auth" = [])
  ),
  request_body(
    content = DeleteUrlRequest,
    description = "Delete URL request",
    content_type = "application/json",
    example = json!({"id": "1234567890"})
  ),
  responses(
      (status = 200, description = HttpResponseCode::Success.to_message(), body = AppHttpResponseDeleteUrlResponse, example = json!({"message": HttpResponseCode::Success.to_message(), "code": HttpResponseCode::Success.to_str(), "data": {"id": "1234567890"}})),
      (status = 400, description = HttpResponseCode::BadRequest.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
      (status = 401, description = HttpResponseCode::Unauthorized.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::Unauthorized.to_message(), "code": HttpResponseCode::Unauthorized.to_str(), "data": null})),
  ),
)]
pub async fn url_delete(
  state: State<AppState>,
  authorization: Option<TypedHeader<Authorization<Bearer>>>,
  Json(data): Json<DeleteUrlRequest>,
) -> Result<Json<AppHttpResponse<DeleteUrlResponse>>, AppError> {
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

          let url = urls::Entity::find()
              .filter(urls::Column::Id.eq(Uuid::parse_str(&data.id).unwrap()).and(urls::Column::UserId.eq(user.id)))
              .one(&state.database)
              .await?;

          if let Some(url) = url {
            // is_delete set to true
            let url = urls::ActiveModel {
              id: ActiveValue::set(url.id),
              is_delete: ActiveValue::set(true),
              ..Default::default()
            };

            urls::Entity::update(url).exec(&state.database).await?;

            return Ok(Json(AppHttpResponse::new(
              HttpResponseCode::Success.to_message().to_string(),
              HttpResponseCode::Success.to_str().to_string(),
              Some(DeleteUrlResponse {
                  id: data.id.clone(),    
              }),
            )));
          }

          return Err(AppError::BadRequestError(BadRequestError("url not found".to_string())));
        }

        return Err(AppError::BadRequestError(BadRequestError("user not found".to_string())));
  } 

  return Err(AppError::UnauthorizedError(HttpResponseCode::Unauthorized.to_message().to_string()));
}

use crate::{
  application::AppState,
  entity::users,
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

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfoResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// 1.1.1.1 取得使用者資訊
#[utoipa::path(
  get,
  path = "/api/user",           
  tag = "user",
  operation_id = "user_info",
  security(
    ("bearer_auth" = [])
  ),
  responses(
      (status = 200, description = HttpResponseCode::Success.to_message(), body = AppHttpResponseUserInfoResponse, example = json!({"message": HttpResponseCode::Success.to_message(), "code": HttpResponseCode::Success.to_str(), "data": {"id": "1234567890", "username": "test", "email": "test@example.com"}})),
      (status = 400, description = HttpResponseCode::BadRequest.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
      (status = 401, description = HttpResponseCode::Unauthorized.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::Unauthorized.to_message(), "code": HttpResponseCode::Unauthorized.to_str(), "data": null})),
  )
)]
pub async fn user_info(
  state: State<AppState>,
  authorization: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<Json<AppHttpResponse<UserInfoResponse>>, AppError> {
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

          // response user info
          return Ok(Json(AppHttpResponse::new(
            HttpResponseCode::Success.to_message().to_string(),
            HttpResponseCode::Success.to_str().to_string(),
            Some(UserInfoResponse {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
            }))))
      }
  } 

  return Err(AppError::UnauthorizedError(HttpResponseCode::Unauthorized.to_message().to_string()));
}

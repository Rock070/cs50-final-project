use crate::{
  AppState,
  LoginPayload,
  users
};

use sea_orm::EntityTrait;
use validator::Validate;
use sea_orm::{
  ColumnTrait,
  QueryFilter
};

use serde_json::json;

use axum::{
  http::StatusCode,
  extract::{
    State, 
    Json
  }
};

use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub async fn user_login(state: State<AppState>, Json(payload): Json<LoginPayload>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)> {
  let payload_data = LoginPayload {
    username: payload.username,
    password: payload.password,
  };

  if let Err(error) = payload_data.validate() {
    return Err((StatusCode::BAD_REQUEST, Json(error.to_string())));
  }

  let column = users::Entity::find()
    .filter(users::Column::Username.eq(payload_data.username))
    .one(&state.database)
    .await
    .unwrap();

    if column.is_none() {
      return Err((StatusCode::BAD_REQUEST, Json("Username or password is incorrect".to_string())));
    }

    let column = column.unwrap();

    let store_password = PasswordHash::new(&column.password);

    if store_password.is_err() {
      return Err((StatusCode::BAD_REQUEST, Json("Username or password is incorrect".to_string())));
    }

    let argon2 = Argon2::default();
    let result = argon2.verify_password(payload_data.password.as_bytes(), &store_password.unwrap());

    if result.is_err() {
      return Err((StatusCode::BAD_REQUEST, Json("Username or password is incorrect".to_string())));
    }

  let token = state.jwt_handler.clone().create_token(&state.application, column.username.as_str()).map_err(|e| {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
  })?;

  return Ok((StatusCode::OK, Json(json!({
    "message": "Login successful",
    "status": "success",
    "code": StatusCode::OK.as_u16(),
    "data": {
      "id": column.id,
      "username": column.username,
      "email": column.email,
      "token": token
    }
  }).to_string())));
}

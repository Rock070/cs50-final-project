use crate::{
    users, AppError, AppHttpResponse, AppState, BadRequestError, HttpResponseCode, LoginPayload,
};

use validator::ValidationErrors;

use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::Validate;

use axum::extract::{Json, State};

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct LoginResponseData {
    pub token: String,
    pub id: String,
    pub username: String,
    pub email: String,
}

/// 1.1.1.2 使用者登入
#[utoipa::path(
    post,
    path = "/user/login",
    tag= "user",
    request_body(
        content = LoginRequest,
        description = "Login request",
        content_type = "application/json",
        example = json!({"username": "test", "password": "test"})
    ),
    responses(
        (status = 200, description = HttpResponseCode::Success.to_message(), body = AppHttpResponseLoginResponse, example = json!({"message": HttpResponseCode::Success.to_message(), "code": HttpResponseCode::Success.to_str(), "data": {"token": "eydJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOjEsImlhdCI6MTcyMjYyMjYyMiwiZXhwIjoxNzIyNjIyNjgyLCJ0eXBlIjoiQkVBUkRfVE9LRU4ifQ.1234567890", "id": "e83592cf-fe9d-4473-acc5-c7202be2230d", "username": "test", "email": "test@example.com"}})),
        (status = 400, description = HttpResponseCode::BadRequest.to_message(), body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
    )
)]
pub async fn user_login(
    state: State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AppHttpResponse<LoginResponseData>>, AppError> {
    let payload_data = LoginPayload::try_from(payload)?;

    let column = users::Entity::find()
        .filter(users::Column::Username.eq(payload_data.username))
        .one(&state.database)
        .await?;

    let column = column.unwrap();

    let store_password = PasswordHash::new(&column.password)
        .map_err(|e| AppError::BadRequestError(BadRequestError(e.to_string())))?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(payload_data.password.as_bytes(), &store_password)
        .map_err(|_| {
            AppError::BadRequestError(BadRequestError(
                "Username or password is incorrect".to_string(),
            ))
        })?;

    let token = state
        .jwt_handler
        .clone()
        .create_token(&state.application, column.username.as_str())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    return Ok(Json(AppHttpResponse::new(
        HttpResponseCode::Success.to_message().to_string(),
        HttpResponseCode::Success.to_str().to_string(),
        Some(LoginResponseData {
            token,
            id: column.id.to_string(),
            username: column.username,
            email: column.email,
        }),
    )));
}

impl TryFrom<LoginRequest> for LoginPayload {
    type Error = ValidationErrors;

    fn try_from(value: LoginRequest) -> Result<Self, ValidationErrors> {
        let payload_data = LoginPayload {
            username: value.username.clone(),
            password: value.password.clone(),
        };

        if let Err(error) = payload_data.validate() {
            return Err(error);
        };

        Ok(Self {
            username: payload_data.username,
            password: payload_data.password,
        })
    }
}

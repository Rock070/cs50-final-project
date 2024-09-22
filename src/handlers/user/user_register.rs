use uuid::Uuid;
use validator::Validate;

use crate::{
    application::AppState, domain::RegisterPayload, entity::users, http::HttpResponseCode,
    AppError, AppHttpResponse, CustomError,
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use validator::ValidationErrors;

use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

#[derive(Deserialize, Debug, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
    pub code: String,
    pub data: RegisterResponseData,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct RegisterResponseData {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

/// 1.1.1.1 使用者註冊
#[utoipa::path(
    post,
    path = "/api/user/register",
    tag = "user",
    request_body(
        content = RegisterRequest,
        description = "Register request",
        content_type = "application/json",
        example = json!({"username": "test", "password": "test", "email": "test@test.com"})
    ),
    responses(
        (status = 200, body = RegisterResponseData, example = json!({"message": "register successfully", "code": "1", "data": {"email": "test@test.com", "username": "test"}})),
        (status = 400, body = AppHttpResponseNone, example = json!({"message": HttpResponseCode::BadRequest.to_message(), "code": HttpResponseCode::BadRequest.to_str(), "data": null})),
    )
)]
pub async fn user_register(
    state: State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AppHttpResponse<RegisterResponseData>>, AppError> {
    let register_data = RegisterPayload::try_from(payload)?;

    let password = register_data.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    let username = register_data.username.clone();

    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(&state.database)
        .await?;

    if user.is_none() {
        let id = Uuid::new_v4();
        let register_data_model = users::ActiveModel {
            id: ActiveValue::Set(id),
            username: ActiveValue::Set(register_data.username.clone()),
            password: ActiveValue::Set(password_hash),
            email: ActiveValue::Set(register_data.email.clone()),
            salt: ActiveValue::Set(salt.to_string()),
            ..Default::default()
        };

        users::Entity::insert(register_data_model)
            .exec(&state.database)
            .await?;

        return Ok(Json(AppHttpResponse::new(
            HttpResponseCode::Success.to_message().to_string(),
            HttpResponseCode::Success.to_str().to_string(),
            Some(RegisterResponseData {
                id: id,
                username: register_data.username,
                email: register_data.email,
            }),
        )));
    } else {
        return Err(AppError::CustomError(CustomError {
            status: StatusCode::BAD_REQUEST,
            code: HttpResponseCode::UsernameAlreadyExists,
            message: HttpResponseCode::UsernameAlreadyExists
                .to_message()
                .to_string(),
        }));
    }
}

impl TryFrom<RegisterRequest> for RegisterPayload {
    type Error = ValidationErrors;

    fn try_from(value: RegisterRequest) -> Result<Self, ValidationErrors> {
        let payload_data = RegisterPayload {
            username: value.username.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
        };

        if let Err(error) = payload_data.validate() {
            return Err(error);
        }

        Ok(Self {
            username: payload_data.username,
            password: payload_data.password,
            email: payload_data.email,
        })
    }
}

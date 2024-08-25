use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use serde_json::json;
use sea_orm::error::DbErr;

pub enum AppError {
    UnauthorizedError(String),
    BadRequestError(BadRequestError),
    DatabaseError(DbErr),
    InternalServerError(String),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let internal_server_error_message = String::from("An unexpected error occurred. Please try again later.");

        let (status, code, message) = match self {
            AppError::UnauthorizedError(error) => (StatusCode::UNAUTHORIZED, "000001".to_string(), error),
            AppError::BadRequestError(error) => (StatusCode::BAD_REQUEST, "000002".to_string(), error.0),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "000003".to_string(), internal_server_error_message.clone()),
            AppError::InternalServerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, "000004".to_string(), format!("{}\n{}", &internal_server_error_message, error)),
        };

        let body = Json(json!(AppHttpResponse::<()>::new(message, code, None)));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub struct BadRequestError(pub String);

impl From<BadRequestError> for AppError {
    fn from(error: BadRequestError) -> Self {
        AppError::BadRequestError(error)
    }
}

impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        AppError::DatabaseError(error)
    }
}

#[derive(Serialize)]
pub struct AppHttpResponse<T: Serialize> {
    message: String,
    code: String,
    data: Option<T>,
}

impl<T: Serialize> AppHttpResponse<T> {
    pub fn new(message: String, code: String, data: Option<T>) -> Self {
        Self { message, code, data }
    }
}
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sea_orm::error::DbErr;
use crate::{HttpResponseCode, AppHttpResponse};


#[derive(Debug)]
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
            AppError::UnauthorizedError(error) => (StatusCode::UNAUTHORIZED, HttpResponseCode::Unauthorized.to_str(), error),
            AppError::BadRequestError(error) => (StatusCode::BAD_REQUEST, HttpResponseCode::BadRequest.to_str(), error.0),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, HttpResponseCode::DatabaseError.to_str(), internal_server_error_message.clone()),
            AppError::InternalServerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, HttpResponseCode::InternalServerError.to_str(), format!("{}\n{}", &internal_server_error_message, error)),
        };

        let body = Json(json!(AppHttpResponse::<()>::new(message, code.to_string(), None)));

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

impl From<String> for BadRequestError {
    fn from(error: String) -> Self {
        BadRequestError(error)
    }
}
use serde::Serialize;
use utoipa::{ToSchema, TupleUnit};

use crate::{HashUrlResponse, LoginResponseData, RegisterResponseData};

// https://github.com/juhaku/utoipa/issues/684#issuecomment-1715245683
#[derive(Serialize, ToSchema)]
#[aliases(
  AppHttpResponseNone = AppHttpResponse<TupleUnit>,
  AppHttpResponseHashUrlResponse = AppHttpResponse<HashUrlResponse>,
  AppHttpResponseLoginResponse = AppHttpResponse<LoginResponseData>,
  AppHttpResponseRegisterResponse = AppHttpResponse<RegisterResponseData>,
)]
pub struct AppHttpResponse<T: Serialize> {
    message: String,
    code: String,
    data: Option<T>,
}

impl<T: Serialize> AppHttpResponse<T> {
    pub fn new(message: String, code: String, data: Option<T>) -> Self {
        Self {
            message,
            code,
            data,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub enum HttpResponseCode {
    Success,
    BadRequest,
    ValidationError,
    UsernameAlreadyExists,
    Unauthorized,
    DatabaseError,
    InternalServerError,
}

impl HttpResponseCode {
    pub fn to_str(&self) -> &'static str {
        match self {
            HttpResponseCode::Success => "1",
            HttpResponseCode::Unauthorized => "000001",
            HttpResponseCode::BadRequest => "000002",
            HttpResponseCode::UsernameAlreadyExists => "000003",
            HttpResponseCode::ValidationError => "000004",
            HttpResponseCode::DatabaseError => "000005",
            HttpResponseCode::InternalServerError => "000006",
        }
    }

    pub fn to_message(&self) -> &'static str {
        match self {
            HttpResponseCode::Success => "success",
            HttpResponseCode::Unauthorized => "Unauthorized",
            HttpResponseCode::BadRequest => "Bad Request",
            HttpResponseCode::UsernameAlreadyExists => "Username already exists",
            HttpResponseCode::ValidationError => "Validation Error",
            HttpResponseCode::DatabaseError => "Database Error",
            HttpResponseCode::InternalServerError => "Internal Server Error",
        }
    }
}

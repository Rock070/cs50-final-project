use serde::Serialize;
use utoipa::{ToSchema, TupleUnit};

use crate::HashUrlResponse;

// https://github.com/juhaku/utoipa/issues/684#issuecomment-1715245683
#[derive(Serialize, ToSchema)]
#[aliases(
  AppHttpResponseNone = AppHttpResponse<TupleUnit>,
  AppHttpResponseHashUrlResponse = AppHttpResponse<HashUrlResponse>,
)]
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

pub enum HttpResponseCode {
    Success,
    BadRequest,
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
      HttpResponseCode::DatabaseError => "000003",
      HttpResponseCode::InternalServerError => "000004",
    }
  }

  pub fn to_message(&self) -> &'static str {
    match self {
      HttpResponseCode::Success => "success",
      HttpResponseCode::Unauthorized => "Unauthorized",
      HttpResponseCode::BadRequest => "Bad Request",
      HttpResponseCode::DatabaseError => "Database Error",
      HttpResponseCode::InternalServerError => "Internal Server Error",
    }
  }
}
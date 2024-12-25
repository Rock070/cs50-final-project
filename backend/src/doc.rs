use crate::{
    handlers::*, AppHttpResponse, DeleteUrlResponse, HashUrlResponse, LoginRequest,
    LoginResponseData, RegisterRequest, RegisterResponseData,
};

use utoipa::{OpenApi, TupleUnit};

#[derive(OpenApi)]
#[openapi(
  paths(
    user_info,
    user_register,
    user_login,
    user_urls,
    url_redirect,
    url_hash,
    url_delete,
  ),
  tags(
    (name = "user"),
    (name = "url")
  ),
  components(schemas(
      HashUrlRequest,
      UserInfoResponse,
      LoginRequest,
      RegisterRequest,
      UserUrlsResponse,
      RegisterResponseData,
      LoginResponseData,
      HashUrlResponse,
      TupleUnit,
      AppHttpResponse<HashUrlResponse>,
      AppHttpResponse<UserUrlsResponse>,
      AppHttpResponse<UserInfoResponse>,
      AppHttpResponse<LoginResponseData>,
      AppHttpResponse<TupleUnit>,
      AppHttpResponse<DeleteUrlResponse>
    )
  ),
)]
pub struct ApiDoc;

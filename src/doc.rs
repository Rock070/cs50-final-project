use crate::{
    handlers::*, AppHttpResponse, HashUrlResponse, LoginRequest, LoginResponseData,
    RegisterRequest, RegisterResponseData,
};

use utoipa::{OpenApi, TupleUnit};

#[derive(OpenApi)]
#[openapi(
  paths(
    user_info,
    user_register,
    user_login,
    hash_url,
    redirect_hash_url,
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
      RegisterResponseData,
      LoginResponseData,
      HashUrlResponse,
      TupleUnit,
      AppHttpResponse<HashUrlResponse>,
      AppHttpResponse<LoginResponseData>,
      AppHttpResponse<TupleUnit>
    )
  ),
)]
pub struct ApiDoc;

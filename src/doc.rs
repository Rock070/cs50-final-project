use crate::handlers::*;

use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
  // paths(hash_url, redirect_url, user_login, user_register),
  // components(schemas(HashUrlRequest, String, LoginPayload, RegisterPayload)),
  paths(hash_url),
  components(schemas(HashUrlRequest)),
)]
pub struct ApiDoc;
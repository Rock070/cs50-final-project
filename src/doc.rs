use crate::{
  handlers::*,
  HashUrlResponse,
  AppHttpResponse,
};

use utoipa::{OpenApi, TupleUnit};

#[derive(OpenApi)]
#[openapi(
  paths(hash_url),
  components(schemas(
      HashUrlRequest, 
      HashUrlResponse,
      TupleUnit,
      AppHttpResponse<HashUrlResponse>, 
      AppHttpResponse<TupleUnit>
    )
  ),
)]
pub struct ApiDoc;
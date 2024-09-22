use crate::{
    application::AppState,
    entity::{request_records, urls},
    AppError, BadRequestError, HashPath,
};
use axum_extra::{
    headers::{Origin, UserAgent},
    TypedHeader,
};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Path, State},
    response::Redirect,
};

type HashUrlRequest = String;

/// 2.1.1.2 轉導短網址至原網址
#[utoipa::path(
    get,
    path = "/api/{path}",
    tag = "url",
    responses(
        (status = 302, description = "Redirect to the original URL"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn redirect_hash_url(
    state: State<AppState>,
    header_user_agent: Option<TypedHeader<UserAgent>>,
    header_origin: Option<TypedHeader<Origin>>,
    connect_info: Option<ConnectInfo<SocketAddr>>,
    Path(path): Path<HashUrlRequest>,
) -> Result<Redirect, AppError> {
    let info = connect_info
        .map(|ConnectInfo(addr)| addr.to_string())
        .unwrap_or_else(|| "unkown".to_string());

    let user_agent = header_user_agent
        .map(|ua| ua.to_string())
        .unwrap_or_default();
    let origin = header_origin.map(|o| o.to_string()).unwrap_or_default();

    let path = HashPath::try_from(path)?.0;

    let column = urls::Entity::find()
        .filter(urls::Column::ShortUrl.eq(path))
        .one(&state.database)
        .await?;

    if let Some(model) = column {
        let request_record = request_records::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            user_agent: ActiveValue::set(user_agent.clone()),
            origin: ActiveValue::set(Some(origin.clone())),
            ip: ActiveValue::set(info.to_string()),
            url_id: ActiveValue::set(model.id),
            ..Default::default()
        };

        request_records::Entity::insert(request_record)
            .exec(&state.database)
            .await?;

        return Ok(Redirect::temporary(&model.url));
    }

    Err(AppError::BadRequestError(BadRequestError::from(
        "Not found".to_string(),
    )))
}

impl TryFrom<HashUrlRequest> for HashPath {
    type Error = BadRequestError;

    fn try_from(value: String) -> Result<Self, BadRequestError> {
        let path = HashPath::parse_path(value)?;

        Ok(path)
    }
}

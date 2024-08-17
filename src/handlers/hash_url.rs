use crate::{application::AppState, domain::HashUrlBody, entity::urls};

use axum::{extract::State, http::StatusCode, Json};
use base62::encode;
use sea_orm::{ActiveValue, EntityTrait};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub async fn hash_url(
    state: State<AppState>,
    Json(data): Json<HashUrlBody>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let body = HashUrlBody::new(data.url);
    match body {
        Ok(url_body) => {
            // 使用 SHA-256 對 URL 進行哈希
            let mut hasher = Sha256::new();
            hasher.update(url_body.url.as_bytes());
            let result = hasher.finalize();

            // 將哈希結果轉換為 u64
            // 取 sha-256 結果的前 8 個 bytes，使用 try_into 宣告為長度為 8 的 u8 陣列，
            // 再使用 from_be_bytes 轉換為 u64
            let num = u64::from_be_bytes(result[..8].try_into().unwrap());

            // 使用 base62 編碼
            let short_url = encode(num);

            let url = urls::ActiveModel {
                id: ActiveValue::Set(Uuid::new_v4()),
                url: ActiveValue::Set(url_body.url),
                user_id: ActiveValue::NotSet,
                short_url: ActiveValue::Set(short_url.clone()),
                updated_at: ActiveValue::NotSet,
                ..Default::default()
            };

            urls::Entity::insert(url)
                .exec(&state.database)
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            Ok((StatusCode::OK, format!("縮短的 URL 是: {}", short_url)))
        }
        Err(e) => {
            let error_message = format!("[無效的 URL] {}", e);
            Err((StatusCode::BAD_REQUEST, error_message))
        }
    }
}

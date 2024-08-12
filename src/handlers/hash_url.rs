use axum::{http::StatusCode, Json};
use crate::{{}};
use sha2::{Sha256, Digest};
use base62::{encode, decode};
use serde::Deserialize;
use validator::Validate;
use sea_orm::Database;

use crate::domain::HashUrlBodyUrl;

#[derive(Debug, Deserialize, Validate)]
pub struct HashUrlBody {
    /**
     * raw url before hashing
     */
    #[validate(url)]
    pub url: String,
}

pub async fn hash_url(Json(data): Json<HashUrlBody>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let body = HashUrlBodyUrl::new(data.url);
    match body {
        Ok(_url) => {
          let database = Database::connect("postgres://rock:rock0702@localhost:5432/short_url")
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

          let hash = Sha256::digest(b"my message");
          let num: u64 = 999999;
          let hash2 = encode(num);
          let de = decode("4C91");
          println!("hash Result: {:?}", hash);
          println!("hash2 Result: {}", hash2);
          println!("de Result: {:?}", de);

          Ok((StatusCode::OK, "URL is valid".to_string()))
        }
        Err(e) => {
            let error_message = format!("[Invalid URL] {}", e);
            println!("{}", 2);
            Err((StatusCode::BAD_REQUEST, error_message))
        }
    }
}

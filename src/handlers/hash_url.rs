use axum::{http::StatusCode, response::IntoResponse, Form};
use sha2::{Sha256, Digest};
use base62::{encode, decode};
use serde::Deserialize;
use validator::Validate;

use crate::domain::HashUrlBodyUrl;

#[derive(Debug, Deserialize, Validate)]
pub struct HashUrlBody {
    /**
     * raw url before hashing
     */
    #[validate(url)]
    pub url: String,
}

pub async fn hash_url(Form(data): Form<HashUrlBody>) -> impl IntoResponse {
    let body = HashUrlBodyUrl::new(data.url);
    match body {
        Ok(_url) => {
          let hash = Sha256::digest(b"my message");
          let num: u64 = 999999;
          let hash2 = encode(num);
          let de = decode("4C91");
          println!("hash Result: {:?}", hash);
          println!("hash2 Result: {}", hash2);
          println!("de Result: {:?}", de);

          (StatusCode::OK, "URL is valid".to_string())
        }
        Err(e) => {
            let error_message = format!("[Invalid URL] {}", e);
            println!("{}", 2);
            (StatusCode::BAD_REQUEST, error_message)
        }
    }
}

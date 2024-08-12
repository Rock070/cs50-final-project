use axum::{
  body::Body,
  http::{Request, StatusCode},
};

use shor::Application;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
  let app = Application::build().router;

  let response = app
      .oneshot(
          Request::builder()
              .uri("/health_check")
              .body(Body::empty())
              .unwrap(),
      )
      .await
      .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_hash_url() {
  let app = Application::build().router;

  let response = app
      .oneshot(
          Request::builder()
              .method("POST")
              .uri("/hash_url")
              .header("Content-Type", "application/json")
              .body(Body::from(r#"{"url": "https://example.com"}"#))
              .unwrap(),
      )
      .await
      .unwrap();

  assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_hash_invalid_url() {
  let app = Application::build().router;

  let response = app
      .oneshot(
          Request::builder()
              .method("POST")
              .uri("/hash_url")
              .header("Content-Type", "application/json")
              .body(Body::from(r#"{"url": "invalid_url"}"#))
              .unwrap(),
      )
      .await
      .unwrap();

  assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
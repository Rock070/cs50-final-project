use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use shor::Application;
use tower::ServiceExt;

#[tokio::test]
async fn test_handle_hash_url() {
    let app = Application::build()
        .await
        .router;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/hash_url")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"url": "https://google.com"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_handle_hash_invalid_url() {
    let app = Application::build()
        .await
        .router;

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


#[tokio::test]
async fn test_handle_redirect_hash_url() {
    let app = Application::build()
      .await
      .router;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/Qhq1TQ2nVI")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
}

#[tokio::test]
async fn test_handle_redirect_invalid_hash_url() {
    let app = Application::build()
      .await
      .router;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/Qhq1TQ2nVI/")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// testing redirect url with valid url, but not exist
#[tokio::test]
async fn test_handle_redirect_nonexistent_hash_url() {
    let app = Application::build()
        .await
        .router;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

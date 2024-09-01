use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

mod common;

mod test_hash_url {

    use super::*;

    #[tokio::test]
    async fn valid() {
        let app = common::setup().await;

        let response = app
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hash-url")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"url": "https://google.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            body,
            json!({
                "message": "success",
                "code": "1",
                "data": {
                    "url": format!("{}{}", app.application.base_url, "Qhq1TQ2nVI")
                }
            })
        );
    }
}

mod test_redirect {
    use super::*;

    #[tokio::test]
    async fn valid() {
        let app_hash = common::setup().await;

        let response = app_hash
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hash-url")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"url": "https://www.apple.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            body,
            json!({
                "message": "success",
                "code": "1",
                "data": {
                    "url": format!("{}{}", app_hash.application.base_url, "9uPcJsmzjF8")
                }
            })
        );

        let app_redirect = common::setup().await;
        let response = app_redirect
            .router
            .oneshot(
                Request::builder()
                    .uri("/9uPcJsmzjF8")
                    .method("GET")
                    .body(Body::empty())
                    .expect("Failed to build request."),
            )
            .await
            .expect("Failed to get response.");
        println!("Response status: {:?}", response.status());
        println!("Response headers: {:?}", response.headers());
        let status = response.status();

        assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    }

    #[tokio::test]
    async fn invalid() {
        let app = common::setup().await;
        let response = app
            .router
            .oneshot(
                Request::builder()
                    .uri("/Qhq1TQ2nVI/")
                    .method("GET")
                    .body(Body::empty())
                    .expect("Failed to build request."),
            )
            .await
            .unwrap();

        println!("response: {:?}", response);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn nonexistent() {
        let app = common::setup().await;
        let response = app
            .router
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

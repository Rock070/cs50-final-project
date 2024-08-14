use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shor::Application;
use tower::ServiceExt;

// TODO: 只建立一次app，然後在每個 test 中重複使用
async fn setup() -> axum::Router {
    Application::build().await.router
}

mod test_hash_url {
    use super::*;

    #[tokio::test]
    async fn test_valid_url() {
        let app = setup().await;
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
    async fn test_invalid_url() {
        let app = setup().await;
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
}

mod test_redirect {
    use super::*;

    #[tokio::test]
    async fn test_valid_hash() {
        let app = setup().await;
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
    async fn test_invalid_hash() {
        let app = setup().await;
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

    #[tokio::test]
    async fn test_nonexistent_hash() {
        let app = setup().await;
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
}
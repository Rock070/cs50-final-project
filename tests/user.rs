use uuid::Uuid;
mod common;
use axum::{
    body::{to_bytes, Body},
    http::Request,
    http::StatusCode,
};

use serde::Serialize;
use tower::ServiceExt;
#[derive(Serialize)]
struct RegisterUser {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize)]
struct LoginUser {
    username: String,
    password: String,
}

mod user_register_test {
    use super::*;

    #[tokio::test]
    async fn valid_register() {
        let app = common::setup().await;

        let random_username = format!("test{}", Uuid::new_v4());
        let user = RegisterUser {
            username: random_username.clone(),
            password: "test".to_string(),
            email: "test@test.com".to_string(),
        };

        let body = serde_json::to_string(&user).unwrap();

        let response = app
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        let data = &body["data"];

        assert_eq!(data["username"], random_username);
        assert_eq!(data["email"], "test@test.com");

        // 確認 id 存在且是 UUID 格式
        assert!(body["data"].get("id").is_some());
        assert!(Uuid::parse_str(body["data"]["id"].as_str().unwrap()).is_ok());
    }

    #[tokio::test]
    async fn invalid_register() {
        let app = common::setup().await;
        let random_username = format!("test{}", Uuid::new_v4());

        let user = RegisterUser {
            username: random_username.clone(),
            password: "test".to_string(),
            email: "valid_email".to_string(),
        };

        let body = serde_json::to_string(&user).unwrap();

        let response = app
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

mod user_login_test {
    use super::*;

    #[tokio::test]
    async fn valid_login() {
        let app_register = common::setup().await;

        let random_username = format!("test{}", Uuid::new_v4());
        let user = RegisterUser {
            username: random_username.clone(),
            password: "test".to_string(),
            email: "test@test.com".to_string(),
        };

        let body = serde_json::to_string(&user).unwrap();

        let response = app_register
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let login_user = LoginUser {
            username: random_username.clone(),
            password: "test".to_string(),
        };

        let body = serde_json::to_string(&login_user).unwrap();

        let app_login = common::setup().await;

        let response = app_login
            .router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        let data = &body["data"];

        assert_eq!(data["username"], random_username);
        assert_eq!(data["email"], "test@test.com");
        assert_eq!(data["token"].as_str().is_some(), true);

        // 確認 id 存在且是 UUID 格式
        assert!(body["data"].get("id").is_some());
        assert!(Uuid::parse_str(body["data"]["id"].as_str().unwrap()).is_ok());
    }
}

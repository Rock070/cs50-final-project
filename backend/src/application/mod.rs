use crate::{
    handlers::*, ApiDoc, ApplicationSetting, Configuration, DatabaseSetting, JwtHandler,
    JwtHandlerSetting,
};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{delete, get, post},
    Router,
};

use jsonwebtoken::{Algorithm, Header};
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    pub application: ApplicationSetting,
    pub router: Router,
}

impl Application {
    pub async fn build(config: &Configuration) -> Self {
        let database = get_database(&config.database)
            .await
            .expect("Failed to connect to database");
        let jwt_handler = get_jwt_handler(&config.jwt_handler);
        let cors = get_cors_layer(&config.application);

        let router = Router::new()
            .route("/*path", get(url_redirect))
            .route("/api/url/hash", post(url_hash))
            .route("/api/url", delete(url_delete))
            .route("/api/user/urls", get(user_urls))
            .route("/api/user", get(user_info))
            .route("/api/user/login", post(user_login))
            .route("/api/user/register", post(user_register))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .with_state(AppState {
                database,
                jwt_handler,
                application: config.application.clone(),
            });

        Self {
            application: config.application.clone(),
            router,
        }
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.application.port));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        axum::serve(
            listener,
            self.router
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }
}

pub async fn get_database(setting: &DatabaseSetting) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(setting.connection_string().expose_secret()).await
}

pub fn get_jwt_handler(setting: &JwtHandlerSetting) -> JwtHandler {
    JwtHandler {
        private_key: setting.private_key.clone(),
        header: Header::new(Algorithm::RS256),
        public_key: setting.public_key.clone(),
        expiration_minutes: setting.expiration_minutes,
    }
}

pub fn get_cors_layer(setting: &ApplicationSetting) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            setting
                .cors_base_url
                .iter()
                .map(|url| url.parse().unwrap())
                .collect::<Vec<HeaderValue>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true)
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub jwt_handler: JwtHandler,
    pub application: ApplicationSetting,
}

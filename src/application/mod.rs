use crate::{
    handlers::*, ApiDoc, ApplicationSetting, Configuration, DatabaseSetting, JwtHandler,
    JwtHandlerSetting,
};

use axum::{
    routing::{get, post},
    Router,
};
use jsonwebtoken::{Algorithm, Header};
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    port: u16,
    pub router: Router,
}

impl Application {
    pub async fn build(config: &Configuration) -> Self {
        let database = get_database(&config.database).await.unwrap();
        let jwt_handler = get_jwt_handler(&config.jwt_handler);

        let router = Router::new()
            .route("/*path", get(redirect_hash_url))
            .route("/hash_url", post(hash_url))
            .route("/user/login", post(user_login))
            .route("/user/register", post(user_register))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(TraceLayer::new_for_http())
            .with_state(AppState {
                database,
                jwt_handler,
                application: config.application.clone(),
            });

        Self {
            port: config.application.port,
            router,
        }
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));

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
        expiration_time: setting.expiration_time,
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub jwt_handler: JwtHandler,
    pub application: ApplicationSetting,
}

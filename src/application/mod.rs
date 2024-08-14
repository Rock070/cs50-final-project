use crate::handlers::*;
use axum:: {
    routing::{get, post},
    Router
};
use sea_orm::{DatabaseConnection, Database};

pub struct Application {
    port: u16,
    pub router: Router,
}

impl Application {
    pub async fn build() -> Self {
        let database = Database::connect("postgres://rock:rock0702@localhost:5432/shor")
        .await
        .unwrap_or_else(|error| {
            panic!("Error connecting to database: {}", error);
        });

        let router = Router::new()
            .route("/*path", get(redirect_hash_url))
            .route("/hash_url", post(hash_url))
            .with_state(AppState {
                database
            });

        Self { port: 3000, router }
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, self.router).await.unwrap();
    }
}


#[derive(Debug, Clone)]
pub struct AppState {
    pub database: DatabaseConnection
}
use crate::handlers::*;
use axum::routing::{get, post};
use axum::Router;

pub struct Application {
    port: u16,
    router: Router,
}

impl Application {
    pub fn build() -> Self {
        let router = Router::new()
            .route("/health_check", get(health_check))
            .route("/hash_url", post(hash_url));

        Self { port: 3000, router }
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, self.router).await.unwrap();
    }
}

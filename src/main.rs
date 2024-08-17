use shor::Application;

#[tokio::main]
async fn main() {
    Application::build().await.run().await;
}

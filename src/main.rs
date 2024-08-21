use shor::{Application, get_configuration};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration.");

    Application::build(&config).await.run().await;
}
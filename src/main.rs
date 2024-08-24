use shor::{get_configuration, Application};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration.");

    Application::build(&config).await.run().await;
}

use shor::{get_configuration, get_subscriber, Application};
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration.");

    get_subscriber(&config).init();

    Application::build(&config).await.run().await;
}

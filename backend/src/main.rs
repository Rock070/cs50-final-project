use shor::{get_configuration, get_subscriber, Application};
use std::env;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let mut config_path = "./configuration/development.yml".to_string();
    let args: Vec<String> = env::args().collect();
    if let Some(pos) = args.iter().position(|arg| arg == "--config") {
        if let Some(path) = args.get(pos + 1) {
            config_path = path.clone();
        }
    }
    let config = get_configuration(&config_path).expect("Failed to read configuration.");

    get_subscriber(&config).init();
    Application::build(&config).await.run().await;
}

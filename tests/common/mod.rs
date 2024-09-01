use shor::{get_configuration, Application};

pub async fn setup() -> Application {
    let config = get_configuration().expect("Failed to read configuration.");
    Application::build(&config).await
}

use crate::Configuration;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn get_subscriber(config: &Configuration) -> impl SubscriberInitExt {
    let filter = get_filter(&config.application.logging_levels);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json())
}

pub fn get_filter(modules: &[String]) -> EnvFilter {
    let filter_settings = modules.join(",");

    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse_lossy(filter_settings)
}

use config::Config;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Deserializer};
use serde_aux::prelude::deserialize_number_from_string;
use std::env;

#[derive(Debug)]
pub enum ENVIRONMENT {
    DEVELOPMENT,
    PRODUCTION,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub application: ApplicationSetting,
    pub database: DatabaseSetting,
    pub jwt_handler: JwtHandlerSetting,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSetting {
    pub base_url: String,
    pub port: u16,
    pub owner: String,
    pub name: String,
    pub cors_base_url: Vec<String>,
    pub logging_levels: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSetting {
    pub db_type: String,
    pub name: String,
    pub user: String,
    #[serde(deserialize_with = "deserialize_secret")]
    pub password: Secret<String>,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtHandlerSetting {
    #[serde(deserialize_with = "deserialize_secret")]
    pub private_key: Secret<String>,
    pub public_key: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub expiration_minutes: i64,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_type,
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }
}

// 自訂的反序列化函數
fn deserialize_secret<'de, D>(deserializer: D) -> Result<Secret<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Secret::new(s))
}

pub fn get_configuration(config_path: &str) -> Result<Configuration, config::ConfigError> {
    let basic_path = env::current_dir().expect("Failed to get current directory");
    let config_path = basic_path.join(config_path);

    let path = config_path.clone();

    let config = Config::builder()
        .add_source(config::File::from(config_path.join(path)).required(true))
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()?;

    config.try_deserialize::<Configuration>()
}

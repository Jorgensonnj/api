use config::{Config, ConfigError, File, FileFormat};
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub telemetry: TelemetrySettings,
    pub database: Option<DatabaseSettings>,
    pub modules: Option<HashMap<String, ModuleSettings>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ServerSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub key_path: Option<String>,
    pub cert_path: Option<String>,
}

impl ServerSettings {
    // Server string
    pub fn address_string(&self) -> String {
        format!(
            "{}://{}:{}",
            self.driver,
            self.host,
            self.port,
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TelemetrySettings {
    pub env_filter: String
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub driver: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

impl DatabaseSettings {
    // Database credential string
    pub fn connection_db_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.driver,
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }

    // Database credential string without database name
    pub fn connection_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/",
            self.driver,
            self.username,
            self.password,
            self.host,
            self.port
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ModuleSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
}

impl ModuleSettings {
    // Module connection string
    pub fn address_string(&self) -> String {
        format!(
            "{}://{}:{}",
            self.driver,
            self.host,
            self.port
        )
    }
}


pub fn get_config() -> Result<Settings, ConfigError> {

    // Start at the root directory
    let mut file_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    file_path.push_str("/settings.toml");

    // Build configuration
    let config = Config::builder()
        .set_default("server.driver",        "http")?
        .set_default("server.host",          "localhost")?
        .set_default("server.port",          "8080")?
        .set_default("telemetry.env_filter", "warn")?
        .add_source(File::new(&file_path, FileFormat::Toml))
        .build()
        .expect("Configuration failed to build");

    // Convert configuration into settings
    let convert_config: Result<Settings, ConfigError> = config.try_deserialize();

    convert_config
}

use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub telemetry: TelemetrySettings,
}

#[derive(serde::Deserialize)]
pub struct ServerSettings {
    pub application_port: u16,
    pub key_path: String,
    pub cert_path: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub driver: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

#[derive(serde::Deserialize)]
pub struct TelemetrySettings {
    pub env_filter: String
}

impl DatabaseSettings {
    // Database credential string
    pub fn connection_db_string(&self) -> String {
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
    pub fn connection_string(&self) -> String {
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

pub fn get_config() -> Result<Settings, ConfigError> {

    // Start at the root directory
    let mut file_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    file_path.push_str("/src/settings.toml");

    // Build configuration
    let config = Config::builder()
        .set_default("default", "1")?
        .add_source(File::new(&file_path, FileFormat::Toml))
        .build()
        .expect("Configuration failed to build");

    // Convert configuration into settings
    let convert_config: Result<Settings, ConfigError> = config.try_deserialize();

    convert_config
}

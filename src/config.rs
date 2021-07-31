
use config::{Config, ConfigError, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
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
    // Initialize
    let mut settings = Config::default();

    // Add configuration values from a file named "configuration".
    settings.merge(File::with_name("src/settings.toml"))?;

    settings.try_into()
}

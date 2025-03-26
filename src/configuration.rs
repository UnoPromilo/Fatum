use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let configuration = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    configuration.try_deserialize::<Configuration>()
}

#[derive(Deserialize)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct ApplicationConfiguration {
    pub port: u16,
    pub host: String,
    pub hmac_secret: SecretString,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> SecretString {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
        .into()
    }
}

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
    pub application_port: u16,
    pub database: DatabaseConfiguration,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
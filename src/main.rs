use fatum_api_rs::configuration::*;
use fatum_api_rs::startup::run;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port,
    );
    let listener = TcpListener::bind(address).await?;
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to the database");
    run(
        listener,
        connection_pool,
        configuration.application.hmac_secret,
    )?
    .await
}

use fatum_api_rs::configuration::*;
use fatum_api_rs::startup::run;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await?;
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    run(listener, connection_pool)?.await
}

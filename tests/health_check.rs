use fatum_api_rs::configuration::get_configuration;
use fatum_api_rs::startup::run;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_returns_a_200() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    
    // Act
    let response = client
    .get(&format!("{}/health_check", &test_app.address))
    .send()
    .await;
    
    // Assert
    assert!(response.is_ok());
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(async move { server.await });
    TestApp {
        address,
        db_pool: db_pool.clone(),
    }
}

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

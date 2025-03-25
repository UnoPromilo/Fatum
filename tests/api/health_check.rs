use crate::helpers::spawn_app;

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

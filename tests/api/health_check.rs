use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_returns_a_200() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = test_app.get_health_check().await;

    // Assert
    assert!(response.status().is_success());
}

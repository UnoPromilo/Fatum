use crate::helpers::spawn_app;
use sqlx::PgPool;

#[sqlx::test]
async fn health_check_returns_a_200(db_pool: PgPool) {
    // Arrange
    let test_app = spawn_app(db_pool).await;

    // Act
    let response = test_app.get_health_check().await;

    // Assert
    assert!(response.status().is_success());
}

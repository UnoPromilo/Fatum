use crate::helpers::spawn_app;
use sqlx::PgPool;
#[sqlx::test]
async fn using_correct_username_and_password_returns_a_200(db_pool: PgPool) {
    // Arrange
    let app = spawn_app(db_pool).await;
    let test_user = &app.test_user;
    let login_body = serde_json::json!({
        "email": test_user.email,
        "password": test_user.password,
    });

    // Act
    let response = app.post_login(&login_body).await;
    
    // Assert
    assert!(response.status().is_success());
    let response_body: Response = response.json().await.unwrap();
    assert_eq!(response_body.jwt.is_empty(), false);
}

#[sqlx::test]
async fn using_correct_username_and_incorrect_password_returns_a_404(db_pool: PgPool) {
    // Arrange
    let app = spawn_app(db_pool).await;
    let test_user = &app.test_user;
    let login_body = serde_json::json!({
        "email": test_user.email,
        "password": "wrong password",
    });

    // Act
    let response = app.post_login(&login_body).await;
    
    // Assert
    assert_eq!(response.status().as_str(), "404");
}

#[sqlx::test]
async fn using_incorrect_username_and_correct_password_returns_a_404(db_pool: PgPool) {
    // Arrange
    let app = spawn_app(db_pool).await;
    let test_user = &app.test_user;
    let login_body = serde_json::json!({
        "email": "invalid@email.com",
        "password": test_user.password,
    });

    // Act
    let response = app.post_login(&login_body).await;

    // Assert
    assert_eq!(response.status().as_str(), "404");
}

#[derive(serde::Deserialize)]
struct Response {
    jwt: String,
}

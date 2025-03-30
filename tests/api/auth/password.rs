use crate::helpers::spawn_app;
use sqlx::PgPool;

#[sqlx::test]
async fn change_password_without_token_will_return_401(db_pool: PgPool) {
    // Arrange
    let app = spawn_app(db_pool).await;
    let request_body = serde_json::json!({});

    // Act
    let response = app.change_password(&request_body).await;

    // Assert
    assert_eq!(response.status(), 401)
}

#[sqlx::test]
async fn change_password_with_incorrect_old_password_should_return_403(db_pool: PgPool) {
    // Arrange
    let mut app = spawn_app(db_pool).await;
    let test_user = &app.test_user;
    let token = test_user.get_jwt_token(&app).await;
    app.attach_auth_header(&token);
    let request_body = serde_json::json!({
        "oldPassword": "invalid_password",
        "newPassword": "validNewPassword1@"
    });

    // Act
    let response = app.change_password(&request_body).await;

    // Assert
    assert_eq!(response.status(), 403)
}

#[sqlx::test]
async fn change_password_with_weak_new_password_should_return_422(db_pool: PgPool) {
    // Arrange
    let mut app = spawn_app(db_pool).await;
    let token = &app.test_user.get_jwt_token(&app).await;
    app.attach_auth_header(&token);
    let request_body = serde_json::json!({
        "oldPassword": app.test_user.password,
        "newPassword": "easy"
    });

    // Act
    let response = app.change_password(&request_body).await;

    // Assert
    assert_eq!(response.status(), 422)
}

#[sqlx::test]
async fn change_password_with_strong_new_password_should_return_204(db_pool: PgPool) {
    // Arrange
    let mut app = spawn_app(db_pool).await;
    let token = &app.test_user.get_jwt_token(&app).await;
    app.attach_auth_header(&token);
    let request_body = serde_json::json!({
        "oldPassword": app.test_user.password,
        "newPassword": "validNewPassword1@"
    });

    // Act
    let response = app.change_password(&request_body).await;

    // Assert
    assert_eq!(response.status(), 204)
}

#[sqlx::test]
async fn login_request_with_new_password_should_return_success(db_pool: PgPool) {
    // Arrange
    let mut app = spawn_app(db_pool).await;
    let new_password = "validNewPassword1@";
    let token = &app.test_user.get_jwt_token(&app).await;
    app.attach_auth_header(&token);
    let change_password_body = serde_json::json!({
        "oldPassword": app.test_user.password,
        "newPassword": new_password,
    });

    app.change_password(&change_password_body).await;

    let login_body = serde_json::json!({
        "email": app.test_user.email,
        "password": new_password,
    });

    // Act
    let login_response = app.post_login(&login_body).await;

    // Assert
    assert!(login_response.status().is_success());
}

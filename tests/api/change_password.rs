use crate::helpers::TestApp;
use uuid::Uuid;
use zero2prod::routes::{PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = TestApp::spawn().await;

    // Act
    let response = app.get_change_password().await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/password"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
         the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/password"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

// Helper function used for trying various new passwords
async fn try_new_password(app: &TestApp, new_password: &str) -> String {
    // Act - Part 1 - Login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/dashboard"
    );

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": new_password,
            "new_password_check": new_password,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/password"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    response.text().await.unwrap()
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();

    let html_page = try_new_password(&app, &new_password).await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));

    // Act - Part 4 - Logout
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    // Act - Part 5 - Follow the redirect
    let html_page = app.get_login().await.text().await.unwrap();
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));

    // Act - Part 6 - Login using the new password
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/dashboard"
    );
}

#[tokio::test]
async fn new_password_must_not_be_too_short() {
    // Arrange
    let app = TestApp::spawn().await;
    let mut new_password = Uuid::new_v4().to_string();

    new_password.truncate(PASSWORD_MIN_LEN - 1);
    let html_page = try_new_password(&app, &new_password).await;
    assert!(html_page.contains(
        "<p><i>Your new password is too short - \
        please enter at least 12 characters.</i></p>"
    ));

    new_password.push('0');
    let html_page = try_new_password(&app, &new_password).await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));
}

#[tokio::test]
async fn new_password_must_not_be_too_long() {
    // Arrange
    let app = TestApp::spawn().await;

    let new_password = str::repeat("0", PASSWORD_MAX_LEN + 1);
    let html_page = try_new_password(&app, &new_password).await;
    assert!(html_page.contains(
        "<p><i>Your new password is too long - \
        please enter at most 128 characters.</i></p>"
    ));

    let new_password = str::repeat("0", PASSWORD_MAX_LEN);
    let html_page = try_new_password(&app, &new_password).await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));
}

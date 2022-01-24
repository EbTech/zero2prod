use crate::helpers::TestApp;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = TestApp::spawn().await;
    const FAIL_STRING: &str = "<p><i>Authentication failed</i></p>";

    // Act - Part 1 - Try to login
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    // Act - Part 2 - Follow the redirect
    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains(FAIL_STRING));

    // Act - Part 3 - Reload the login page
    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(!html_page.contains(FAIL_STRING));
}

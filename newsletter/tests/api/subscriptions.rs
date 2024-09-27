use crate::helpers::spawn_app;
use newsletter::get_config;
use sqlx::query;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // arrange
    let app = spawn_app().await;
    let config = get_config().expect("Failed to read configuration.");
    // Connection trait must be in scope!
    let client = reqwest::Client::new();
    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // act
    let response = app.post_subscriptions(valid_body.into()).await;

    // assert
    assert_eq!(200, response.status().as_u16());

    let q = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription.");

    assert_eq!(q.email, "ursula_le_guin@gmail.com");
    assert_eq!(q.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_body_pairs = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("something=else", "missing both name and email"),
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        (
            "name=hello<>there{}&email=ursula_le_guin%40gmail.com",
            "invalid name",
        ),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (invalid_body, error_message) in invalid_body_pairs {
        // act
        let response = app.post_subscriptions(invalid_body.into()).await;

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional context on test failure
            "The API did not return a 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

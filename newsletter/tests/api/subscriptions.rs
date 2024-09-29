use crate::helpers::spawn_app;
use newsletter::get_config;
use sqlx::query;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // arrange
    let app = spawn_app().await;
    let config = get_config().expect("Failed to read configuration.");
    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    let response = app.post_subscriptions(valid_body.into()).await;

    // assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_new_subscriber() {
    // arrange
    let app = spawn_app().await;
    let config = get_config().expect("Failed to read configuration.");
    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(valid_body.into()).await;

    // assert

    let q = query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription.");

    assert_eq!(q.email, "ursula_le_guin@gmail.com");
    assert_eq!(q.name, "le guin");
    assert_eq!(q.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    // arrange
    let app = spawn_app().await;
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

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;

    // assert
    // mock asserts on drop
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;

    // assert
    // get the first request received by the mock server
    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    // parse the request body as JSON
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();

        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(&body["TextBody"].as_str().unwrap());

    // The two links should be identical
    assert_eq!(html_link, text_link);
}

//! tests/health_check.rs

use newsletter::{get_config, get_subscriber, init_subscriber, DatabaseSettings, EmailClient};
use once_cell::sync::Lazy;
use secrecy::Secret;
use sqlx::{query, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test".into();
    let default_filter_level = "debug".into();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

// no need to implement any clean up logic!
//
// when a tokio runtime is shut down all tasks spawned on it are dropped. tokio::test spins up a
// new runtime at the beginning of each test case, and they shut down at the end of each test case.
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an
    // available port which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut config = get_config().expect("Failed to read configuration.");
    config.database.name = Uuid::new_v4().to_string();
    let connection_pool = configure_db(&config.database).await;

    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address!");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.authorization_token,
    );

    let server = newsletter::run(listener, connection_pool.clone(), email_client)
        .expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{}", port),
        db_pool: connection_pool,
    }
}

// You might have noticed that we do not perform any clean-up step at the end of our tests - the
// logical databases we create are not being deleted. This is intentional: we could add a clean-up
// step, but our Postgres instance is used only for test purposes and it’s easy enough to restart it
// if, after hundreds of test runs, performance starts to suffer due to the number of lingering
// (almost empty) databases.
async fn configure_db(config: &DatabaseSettings) -> PgPool {
    // Create database
    let maintenance_settings = DatabaseSettings {
        name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: Secret::new("password".to_string()),
        host: config.host.clone(),
        port: config.port,
        require_ssl: config.require_ssl,
    };
    let mut connection = PgConnection::connect_with(&maintenance_settings.connection_options())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.connection_options())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // arrange
    let url = spawn_app().await.address;
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &url))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // arrange
    let app = spawn_app().await;
    let config = get_config().expect("Failed to read configuration.");
    // Connection trait must be in scope!
    let client = reqwest::Client::new();
    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(valid_body)
        .send()
        .await
        .expect("Failed to execute request.");

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
    let app_address = spawn_app().await.address;
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
        // ("name=Ursula&email=", "empty email"),
        // ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (invalid_body, error_message) in invalid_body_pairs {
        // act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

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

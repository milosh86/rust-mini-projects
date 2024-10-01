use newsletter::startup::{get_connection_pool, Application};
use newsletter::{get_config, get_subscriber, init_subscriber, DatabaseSettings};
use secrecy::Secret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::LazyLock;
use uuid::Uuid;
use wiremock::MockServer;

static TRACING: LazyLock<()> = LazyLock::new(|| {
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

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
        // Extract the link from one of the request fields.
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };
        let html = get_link(&body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(&body["TextBody"].as_str().unwrap());

        ConfirmationLinks { html, plain_text }
    }

    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

// no need to implement any clean up logic!
//
// when a tokio runtime is shut down all tasks spawned on it are dropped. tokio::test spins up a
// new runtime at the beginning of each test case, and they shut down at the end of each test case.
pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let mock_server = MockServer::start().await;

    let config = {
        let mut config = get_config().expect("Failed to read configuration.");
        // use a different db for each test case
        config.database.name = Uuid::new_v4().to_string();
        // use a random OS port
        config.application.port = 0;
        // use the mock server for email
        config.email_client.base_url = mock_server.uri();
        config
    };

    configure_db(&config.database).await;

    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application server!");

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application_port);
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&config.database),
        email_server: mock_server,
    }
}

// You might have noticed that we do not perform any clean-up step at the end of our tests - the
// logical databases we create are not being deleted. This is intentional: we could add a clean-up
// step, but our Postgres instance is used only for test purposes and it’s easy enough to restart it
// if, after hundreds of test runs, performance starts to suffer due to the number of lingering
// (almost empty) databases.
pub async fn configure_db(config: &DatabaseSettings) -> PgPool {
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

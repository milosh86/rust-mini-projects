use newsletter::{get_config, get_subscriber, init_subscriber, run};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration.");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port.");

    let connection_pool =
        PgPool::connect_lazy(&config.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

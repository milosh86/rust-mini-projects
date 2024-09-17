use newsletter::{get_config, get_subscriber, init_subscriber, run};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration.");

    let connection_pool =
        PgPoolOptions::new().connect_lazy_with(config.database.connection_options());

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port.");
    run(listener, connection_pool)?.await?;
    Ok(())
}

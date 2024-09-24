use newsletter::startup::Application;
use newsletter::{get_config, get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read configuration.");

    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}

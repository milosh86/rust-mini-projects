use std::net::TcpListener;

use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let connection_pool_wrapped = web::Data::new(pg_pool);
    let email_client_wrapped = web::Data::new(email_client);

    tracing::info!("Starting server at http://{}", listener.local_addr()?);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(connection_pool_wrapped.clone())
            .app_data(email_client_wrapped.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

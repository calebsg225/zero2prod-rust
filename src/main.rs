//! main.rs

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        telemetry::get_subscriber("z2p".to_string(), "info".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // panic if we can't read a config file
    let config = get_config().expect("failed to read configuration file");

    // generate the address to run the backend from based on configuration options
    let address = format!("{}:{}", config.application.host, config.application.port);

    // bind a server listener to the address the backend will be running on
    let listener = TcpListener::bind(address)?;

    // create a pool of reusable postgres database connections
    let connection_pool = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to postgres");

    // start the backend
    run(listener, connection_pool)?.await
}

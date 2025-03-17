//! main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // panic if we can't read a config file
    let config = get_config().expect("failed to read configuration file");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    run(listener, connection_pool)?.await
}

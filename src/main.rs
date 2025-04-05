//! main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // redirect all `log` events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // initialize global logger
    // use the default env if exists or print at info-level or above by default
    let env_filer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // add the bunyan formatter layer: formats logs to bunyan json
    let formatting_layer = BunyanFormattingLayer::new("z2p".into(), std::io::stdout);

    // create subscriber with previously created layers
    let subscriber = Registry::default()
        .with(env_filer)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // set the subscriber as the global default
    set_global_default(subscriber).expect("Failed to set logging subscriber");

    // panic if we can't read a config file
    let config = get_config().expect("failed to read configuration file");

    // generate the address to run the backend from based on configuration options
    let address = format!("127.0.0.1:{}", config.application_port);

    // bind a server listener to the address the backend will be running on
    let listener = TcpListener::bind(address)?;

    // create a pool of reusable postgres database connections
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    // start the backend
    run(listener, connection_pool)?.await
}

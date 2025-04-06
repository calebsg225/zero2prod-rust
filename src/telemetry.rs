//! src/telemetry.rs

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

/// Create a subscriber with multiple layers
pub fn get_subscriber(name: String, env_filter_type: String) -> impl Subscriber + Sync + Send {
    // initialize global logger
    // use the default env if exists or print at info-level or above by default
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter_type));

    // add the bunyan formatter layer: formats logs to bunyan json
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    // return subscriber with previously created layers
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data
///
/// This should only be called once
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // redirect all `log` events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // set the subscriber as the global default
    set_global_default(subscriber).expect("Failed to set subscriber");
}

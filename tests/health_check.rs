//! tests/health_checks.rs

use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::sync::LazyLock;
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_config};
use zero2prod::startup::run;
use zero2prod::telemetry;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Ensure that the `tracing` stack is only initialized once using `LazyLock`
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let sub_name = "test".to_string();
    let default_filter_level = "debug".to_string();

    match std::env::var("TEST_LOG") {
        Ok(_) => {
            let subscriber =
                telemetry::get_subscriber(sub_name, default_filter_level, std::io::stdout);
            telemetry::init_subscriber(subscriber);
        }
        Err(_) => {
            let subscriber =
                telemetry::get_subscriber(sub_name, default_filter_level, std::io::sink);
            telemetry::init_subscriber(subscriber);
        }
    }
});

/// runs a test tcp server
async fn spawn_app() -> TestApp {
    // code in `TRACING` is run the first time, other times are ignored
    LazyLock::force(&TRACING);

    // create a new listener on an open port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // once a port is selected, figure out which port
    let port = listener.local_addr().unwrap().port();
    // construct the address the listener is running on
    let address = format!("http://127.0.0.1:{}", port);

    // import config options
    let mut config = get_config().expect("Failed to read config");
    // generate a unique id
    config.database.database_name = Uuid::new_v4().to_string();
    // create a pool of reusable postgres database connections
    let connection_pool = configure_database(&config.database).await;

    // create a new tcp server
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    // spawn a new async task running the newly created tcp server
    let _ = tokio::spawn(server);

    // returns the address the server is running on for clients to send requests to
    // returns the db connection pool for clients to concurrently send queries to
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create a single db connection
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to postgres");

    // create a database on the connection
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=foo%20bar&email=foobar%40baz.mail";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved suscription");
    assert_eq!(saved.name, "foo bar");
    assert_eq!(saved.email, "foobar@baz.mail");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_missing_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let invalid_bodies = vec![
        ("name=foo%20bar", "missing email"),
        ("email=foobar%40baz.mail", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in invalid_bodies {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Health check failed");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

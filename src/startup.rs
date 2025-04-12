//! src/startup.rs
use actix_web::{App, HttpServer, dev, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<dev::Server, std::io::Error> {
    // wrap the connection pool in web::Data extractor so that the pool can be accessed by
    // httpserver routes
    let db_pool = web::Data::new(connection);
    // create a new tcp server
    let server = HttpServer::new(move || {
        App::new()
            // use wrap() to add middlewares
            // TracingLogger replaces Logger. TracingLogger is tracing-aware.
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_pool.clone())
    })
    // bind server to listen on existing tcp listener
    .listen(listener)?
    // set to start listening to incoming connections
    .run();

    // do not run the server here: instead return in a Result to be run later
    Ok(server)
}

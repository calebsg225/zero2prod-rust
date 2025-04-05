//! src/subscriptions.rs

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

// let rust know that FormData is what this post body deserializes into
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// pool is the db connection pool
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    // generate unique request id
    // used to keep track of different log spans that may be mixed up across threads
    let req_id = Uuid::new_v4();
    // create an info-level span
    // prefix '%' implements Display for logging purposes
    let req_span = tracing::info_span!(
        "Adding a new subscriber.",
        %req_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    // creating the guard: when this is dropped, the log span is closed
    // this is not good to use in async functions...
    let _req_sp_guard = req_span.enter();

    // create info-level span. Instead of calling 'enter', instrument will be used on the query
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // attempt to post data to sqlx db
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    // use a connection from the connection pool to execute this query
    // Data.get_ref returns PgPool
    .execute(pool.get_ref())
    // instrument this query with query_span: enters the span when this future is polled
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                req_id
            );
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", req_id, e);
            HttpResponse::InternalServerError()
        }
    }
    .finish()
}

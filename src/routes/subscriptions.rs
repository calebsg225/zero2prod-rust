//! src/subscriptions.rs

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

// let rust know that FormData is what this post body deserializes into
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .execute(pool)
    // no need to use `instrument()` here: `#[tracing::instrument()]` takes care of that
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

// creates a span at the beginning of the function invocation
#[tracing::instrument(
    // if `name` is ommitied, function name is used as default
    name = "Adding a new subscriber",
    // ignore `form` and `pool` logs
    skip(form, pool),
    // prefix '%' implements Display for logging purposes
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
// pool is the db connection pool
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    // attempt to post data to sqlx db
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}

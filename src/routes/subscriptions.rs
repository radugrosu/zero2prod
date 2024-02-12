use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let log_prefix = format!("request_id {} - ", request_id);
    tracing::info!("{}Adding '{}' '{}' as a new subscriber", log_prefix, form.email, form.name);
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection` wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("{}Saved new subscriber details in the database", log_prefix);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("{}Failed to execute query: {:?}", log_prefix, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

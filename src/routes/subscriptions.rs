use actix_web::{web, HttpResponse};
use serde::Deserialize;

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

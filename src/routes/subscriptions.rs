use actix_web::{web, HttpResponse};
//use chrono::Utc;
//use sqlx::PgPool;
//use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    println!("email: {} & name: {}", _form.email, _form.name);
    HttpResponse::Ok().finish()
}

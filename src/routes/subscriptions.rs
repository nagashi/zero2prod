use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    format!(
        "Hello {}! You have subscribed with email {}",
        _form.name, _form.email
    );
    HttpResponse::Ok().finish()
}

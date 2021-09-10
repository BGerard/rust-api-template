
use crate::settings::App;
use crate::AppState;
use actix_web::{Responder, HttpResponse, web};
use serde::{Deserialize, Serialize};

pub async fn root() -> HttpResponse {
    HttpResponse::Ok().body("/")
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    version: String,
    environment: String,
}

pub async fn health(data: web::Data<AppState>) -> impl Responder {
    let App { version, environment, .. } = &data.settings.app;
    HttpResponse::Ok().json(HealthResponse {
        version: version.to_string(),
        environment: environment.to_string(),
    })
}

pub async fn config(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.settings)
}
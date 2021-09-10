use crate::handlers::api_handlers::greet;
use actix_web::{web};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello", web::get().to(greet));
    cfg.route("/hello/{name}", web::get().to(greet));
}
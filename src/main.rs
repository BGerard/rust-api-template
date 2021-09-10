extern crate config;
extern crate serde;
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate slog;
extern crate slog_term;

mod settings;
mod routes;
mod handlers;

use slog::Drain;
use actix_web::dev::Service;
use std::future::Future;
use actix_web::dev::ServiceResponse;
use actix_web::dev::ServiceRequest;
use settings::Settings;
use actix_web::{web, App, HttpServer};
use routes::{api};
use handlers::{root_handlers};

#[derive(Clone)]
pub struct AppState {
    settings: Settings,
}

fn root_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(root_handlers::root));
    cfg.route("/health", web::get().to(root_handlers::health));
    cfg.route("/config", web::get().to(root_handlers::config));
}

fn logger_middleware(
    request: ServiceRequest,
    service: &mut impl Service<
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
) -> impl Future<Output = Result<ServiceResponse, actix_web::Error>> {
    println!("Request: {}", request.path());
    let fut = service.call(request);
    async {
        let res = fut.await?;
        Ok(res)
    }    
}


async fn start_http_server(state: AppState) -> std::io::Result<()> {
    let port: u16 = state.settings.app.http_port;
    println!("Starting http server on port: {}", port);
    HttpServer::new(move || { 
        App::new()
        .data(state.clone())
        .wrap_fn(logger_middleware)
        .configure(root_config)
        .service(web::scope("/api").configure(api::api_config)) 
    })
    .bind(("127.0.0.1", port))
    .expect("Failed to start http server")
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Failed to load config.");
    // let decorator = slog_term::TermDecorator::new().build();
    // let drain = slog_term::FullFormat::new(decorator).build().fuse();
    // let root_logger = slog::Logger::root(drain, o!());

    // info!(root_logger, "Application started";
        // "started_at" => format!("{}", chrono::Utc::now()));
        
    let state = AppState {
        settings: settings
    };

    start_http_server(state).await
}
mod config;
mod util;
mod service;
mod model;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use actix_web::{web, App, HttpServer};
use config::db_config;
use config::router_config;
use config::auth_config;
use tracing_actix_web::TracingLogger;

pub use crate::config::app_config::APP_CONFIG;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::from_str(&APP_CONFIG.log.level).unwrap())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let pool = web::block(|| {
        db_config::init()
    }).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(auth_config::Auth)
            .app_data(pool.clone())
            .configure(router_config::init)
    })
    .bind((APP_CONFIG.server.bind.as_str(), APP_CONFIG.server.port))?
    .run()
    .await
}
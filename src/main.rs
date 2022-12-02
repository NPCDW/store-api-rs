mod config;
mod util;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use actix_web::{web, App, HttpServer, Responder};
use config::db_config;
use tracing_actix_web::TracingLogger;
use tracing_actix_web::RequestId;

pub use crate::config::app_config::APP_CONFIG;

async fn index(request_id: RequestId) -> impl Responder {
    tracing::trace!("123");
    tracing::debug!("123");
    tracing::info!("123");
    tracing::warn!("123");
    tracing::error!("123");
    format!("Hello world! {}", request_id)
}

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
            .app_data(pool.clone())
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index)),
            )
    })
    .bind((APP_CONFIG.server.bind.as_str(), APP_CONFIG.server.port))?
    .run()
    .await
}
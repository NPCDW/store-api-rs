use actix_web::{get, Responder};
use tracing_actix_web::RequestId;

#[get("")]
async fn index(request_id: RequestId) -> impl Responder {
    tracing::trace!("123");
    tracing::debug!("123");
    tracing::info!("123");
    tracing::warn!("123");
    tracing::error!("123");
    format!("Hello Store Api Rs! request_id: {}", request_id)
}

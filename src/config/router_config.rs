use crate::service;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/storeapi").service(
            web::scope("/")
            .service(service::index_service::index),
        ).service(
            web::scope("/goods")
            .service(service::goods_service::list)
            .service(service::goods_service::get_info)
            .service(service::goods_service::get_info_by_qrcode)
            .service(service::goods_service::create)
            .service(service::goods_service::update)
            .service(service::goods_service::remove),
        )
    );
}
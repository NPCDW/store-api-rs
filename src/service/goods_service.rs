use actix_web::{get, Responder, post, put, delete, web, HttpResponse};
use crate::{mapper::goods_mapper, model::{common::response_result::ResponseResult, entity::goods::Goods}};
use serde::{Deserialize, Serialize};

#[get("/list")]
async fn list() -> impl Responder {
    ""
}

#[get("/getInfo/{id}")]
async fn get_info(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    let goods = web::block(move || {
        goods_mapper::get_by_id(id)
    }).await.unwrap();
    ResponseResult::ok_data(goods)
}

#[derive(Debug, Deserialize, Serialize)]
struct GetInfoByQrcodeQuery {
    qrcode: String,
}

#[get("/getInfoByQRCode")]
async fn get_info_by_qrcode(info: web::Query<GetInfoByQrcodeQuery>) -> HttpResponse {
    let qrcode = info.into_inner().qrcode;
    let goods = web::block(move || {
        goods_mapper::get_by_qrcode(qrcode)
    }).await.unwrap();
    ResponseResult::ok_data(goods)
}

#[post("/create")]
async fn create(info: web::Json<Goods>) -> HttpResponse {
    let goods = info.into_inner();
    let mut qrcode = goods.qrcode.clone();
    if goods.qrcode.is_none() || goods.qrcode.unwrap().is_empty() {
        qrcode = Some(format!("-{}", chrono::Local::now().timestamp_millis()));
    }
    let goods = Goods { qrcode, ..goods };
    let id = web::block(move || {
        goods_mapper::insert(goods)
    }).await.unwrap();
    if id > 0 {
        ResponseResult::ok_data(id)
    } else {
        ResponseResult::error_msg("create fail".to_string())
    }
}

#[put("/update")]
async fn update() -> HttpResponse {
    ResponseResult::ok()
}

#[delete("/remove/{id}")]
async fn remove() -> HttpResponse {
    ResponseResult::ok()
}

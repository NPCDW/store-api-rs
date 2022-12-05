use actix_web::{get, Responder, post, put, delete, web, HttpResponse};
use crate::{mapper::goods_mapper, model::common::response_result::ResponseResult};

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

#[get("/getInfoByQRCode")]
async fn get_info_by_qrcode() -> impl Responder {
    ""
}

#[post("/create")]
async fn create() -> impl Responder {
    ""
}

#[put("/update")]
async fn update() -> impl Responder {
    ""
}

#[delete("/remove/{id}")]
async fn remove() -> impl Responder {
    ""
}

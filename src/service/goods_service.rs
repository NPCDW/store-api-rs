use actix_web::{get, Responder, post, put, delete, web, HttpResponse};
use crate::{mapper::goods_mapper, model::{common::{response_result::ResponseResult, table_info::TableInfo}, entity::goods::Goods}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ListQuery {
    page_number: usize,
    page_size: usize,
    name: Option<String>,
}

#[get("/list")]
async fn list(info: web::Query<ListQuery>) -> impl Responder {
    let query = info.into_inner();
    let name = query.name.clone();
    let count = web::block(move || {
        goods_mapper::count(name)
    }).await.unwrap();
    if count <= 0 {
        return ResponseResult::ok_data(TableInfo::<Goods>::new(count, vec![]));
    }
    let list = web::block(move || {
        goods_mapper::list(query.page_number, query.page_size, query.name)
    }).await.unwrap();
    ResponseResult::ok_data(TableInfo::new(count, list))
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
async fn update(info: web::Json<Goods>) -> HttpResponse {
    let goods = info.into_inner();
    let res = web::block(move || {
        goods_mapper::update(goods)
    }).await.unwrap();
    if res > 0 {
        ResponseResult::ok_data(res)
    } else {
        ResponseResult::error_msg("update fail".to_string())
    }
}

#[delete("/remove/{id}")]
async fn remove(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    let res = web::block(move || {
        goods_mapper::delete(id)
    }).await.unwrap();
    if res > 0 {
        ResponseResult::ok_data(res)
    } else {
        ResponseResult::error_msg("update fail".to_string())
    }
}

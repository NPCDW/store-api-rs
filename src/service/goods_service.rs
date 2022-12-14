use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::{mapper::goods_mapper, model::{common::{response_result::ResponseResult, table_info::TableInfo}, entity::goods::Goods}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ListQuery {
    #[serde(rename = "pageNumber")]
    page_number: u64,
    #[serde(rename = "pageSize")]
    page_size: u64,
    name: Option<String>,
}

#[get("/list")]
async fn list(info: web::Query<ListQuery>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let query = info.into_inner();
    let name = query.name.clone();
    let count = goods_mapper::count(name)?;
    if count <= 0 {
        return ResponseResult::ok_data(TableInfo::<Goods>::new(count, vec![]));
    }
    let list = goods_mapper::list(query.page_number, query.page_size, query.name)?;
    ResponseResult::ok_data(TableInfo::new(count, list))
}

#[get("/getInfo/{id}")]
async fn get_info(path: web::Path<u32>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = path.into_inner();
    let goods = goods_mapper::get_by_id(id)?;
    ResponseResult::ok_data(goods)
}

#[derive(Debug, Deserialize, Serialize)]
struct GetInfoByQrcodeQuery {
    qrcode: String,
}

#[get("/getInfoByQRCode")]
async fn get_info_by_qrcode(info: web::Query<GetInfoByQrcodeQuery>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let qrcode = info.into_inner().qrcode;
    let goods = goods_mapper::get_by_qrcode(qrcode)?;
    ResponseResult::ok_data(goods)
}

#[post("/create")]
async fn create(info: web::Json<Goods>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let mut goods = info.into_inner();
    if goods.qrcode.is_none() || goods.qrcode.as_ref().unwrap().is_empty() {
        goods.qrcode = Some(format!("-{}", chrono::Local::now().timestamp_millis()));
    }
    let id = goods_mapper::insert(goods)?;
    if id > 0 {
        ResponseResult::ok_data(id)
    } else {
        ResponseResult::error_msg("create fail".to_string())
    }
}

#[put("/update")]
async fn update(info: web::Json<Goods>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let goods = info.into_inner();
    let res = goods_mapper::update(goods)?;
    if res > 0 {
        ResponseResult::ok_data(res)
    } else {
        ResponseResult::error_msg("update fail".to_string())
    }
}

#[delete("/remove/{id}")]
async fn remove(path: web::Path<u32>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = path.into_inner();
    let res = goods_mapper::delete(id)?;
    if res > 0 {
        ResponseResult::ok_data(res)
    } else {
        ResponseResult::error_msg("update fail".to_string())
    }
}

#[cfg(test)]
mod goods_service_test {
    use crate::model::entity::goods::Goods;

    #[test]
    fn test() {
        let mut goods = Goods { ..Default::default() };
        if goods.qrcode.is_none() || goods.qrcode.as_ref().unwrap().is_empty() {
            goods.qrcode = Some("123".to_string());
        }
        println!("{:?}", goods);

        let mut qrcode = Some("123").clone();
        if qrcode.is_none() || qrcode.unwrap().is_empty() {
            qrcode = Some("456");
        }
        println!("{:?}", qrcode);
    }
}
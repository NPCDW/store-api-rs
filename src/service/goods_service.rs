use actix_web::{get, Responder, post, put, delete};

#[get("/list")]
async fn list() -> impl Responder {
    ""
}

#[get("/getInfo/{id}")]
async fn get_info() -> impl Responder {
    ""
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

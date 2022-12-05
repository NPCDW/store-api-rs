use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Goods {
    pub id: u32,
    pub create_time: String,
    pub update_time: String,
    pub qrcode: String,
    pub name: String,
    pub cover: String,
    pub price: f64,
    pub unit: String,
}
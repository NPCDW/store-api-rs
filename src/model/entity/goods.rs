use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Goods {
    pub id: Option<u32>,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    pub qrcode: Option<String>,
    pub name: Option<String>,
    pub cover: Option<String>,
    pub price: Option<f64>,
    pub unit: Option<String>,
}
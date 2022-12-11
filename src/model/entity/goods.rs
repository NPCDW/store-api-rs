use chrono::{naive::serde::ts_milliseconds_option, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Goods {
    pub id: Option<u32>,
    #[serde(rename = "createTime", with = "ts_milliseconds_option", default = "Default::default")]
    pub create_time: Option<NaiveDateTime>,
    #[serde(rename = "updateTime", with = "ts_milliseconds_option", default)]
    pub update_time: Option<NaiveDateTime>,
    pub qrcode: Option<String>,
    pub name: Option<String>,
    pub cover: Option<String>,
    pub price: Option<f64>,
    pub unit: Option<String>,
}
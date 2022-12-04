use rust_decimal::Decimal;
use time::OffsetDateTime;

#[allow(dead_code)]
pub struct Goods {
    pub id: u32,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
    pub qrcode: String,
    pub name: String,
    pub cover: String,
    pub price: Decimal,
    pub unit: String,
}
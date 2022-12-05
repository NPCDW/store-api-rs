use crate::{model::entity::goods::Goods, config::db_config::DB_CONN_POOL};

static GOODS_FIELDS: &'static str = "id, create_time, update_time, qrcode, name, cover, price, unit";

fn row_to_entity(row: &rusqlite::Row) -> Result<Goods, rusqlite::Error> {
    Ok(Goods {
        id: row.get("id")?,
        create_time: row.get("create_time")?,
        update_time: row.get("update_time")?,
        qrcode: row.get("qrcode")?,
        name: row.get("name")?,
        cover: row.get("cover")?,
        price: row.get("price")?,
        unit: row.get("unit")?,
    })
}

#[tokio::main]
pub async fn get_by_id(id: u32) -> Option<Goods> {
    let conn = DB_CONN_POOL.get().await.unwrap();
    let res = conn.interact(move |conn| {
        let sql = format!("SELECT {GOODS_FIELDS} FROM goods WHERE id = :id");
        let params = rusqlite::named_params!{":id": &id};

        let mut stmt = conn.prepare(&sql).unwrap();
        let mut result = stmt.query_map(params, row_to_entity).unwrap();
        result.next()
    }).await.unwrap();
    if res.is_none() {
        return None;
    }
    Some(res.unwrap().unwrap())
}

#[tokio::main]
pub async fn get_by_qrcode(qrcode: String) -> Option<Goods> {
    let conn = DB_CONN_POOL.get().await.unwrap();
    let res = conn.interact(move |conn| {
        let sql = format!("SELECT {GOODS_FIELDS} FROM goods WHERE qrcode = :qrcode");
        let params = rusqlite::named_params!{":qrcode": &qrcode};
    
        let mut stmt = conn.prepare(&sql).unwrap();
        let mut result = stmt.query_map(params, row_to_entity).unwrap();
        result.next()
    }).await.unwrap();
    if res.is_none() {
        return None;
    }
    Some(res.unwrap().unwrap())
}

#[cfg(test)]
mod goods_mapper_test {
    use std::str::FromStr;

    use rust_decimal::Decimal;
    // use super::*;
    use time::{macros::format_description, PrimitiveDateTime};
    
    #[test]
    fn test() {
        let format = format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        );
        let time = PrimitiveDateTime::parse("2020-01-02 03:04:05", &format);
        println!("{:?}", time);
    }
    
    #[test]
    fn test2() {
        let dec = Decimal::from_str("19.023").unwrap();
        println!("{:?}", dec);
    }
    
}
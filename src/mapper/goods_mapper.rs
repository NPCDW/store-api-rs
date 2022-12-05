use rusqlite::Row;

use crate::{model::entity::goods::Goods, config::db_config::DB_CONN_POOL};

static GOODS_FIELDS: &'static str = "id, create_time, update_time, qrcode, name, cover, price, unit";

fn row_to_entity(row: &Row) -> Result<Goods, rusqlite::Error> {
    Ok(Goods {
        id: row.get(0)?,
        create_time: row.get(1)?,
        update_time: row.get(2)?,
        qrcode: row.get(3)?,
        name: row.get(4)?,
        cover: row.get(5)?,
        price: row.get(6)?,
        unit: row.get(7)?,
    })
}

#[tokio::main]
pub async fn get_by_id(id: u32) -> Goods {
    let sql = format!("SELECT {GOODS_FIELDS} FROM goods WHERE id = ?");

    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let mut stmt = conn.prepare(&sql)?;
        let mut result = stmt.query_map([id], row_to_entity)?;
        result.next().unwrap()
    }).await.unwrap().unwrap()
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
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

// fn entity_to_parameter<'a>(goods: &'a Goods) -> impl rusqlite::Params + 'a {
//     let Goods {id, ..} = goods;
//     let id: &'a u32  = &goods.id.clone();
//     rusqlite::named_params! {
//         ":id": id,
//         ":create_time": "",
//     }
// }

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

#[tokio::main]
pub async fn insert(goods: Goods) -> i64 {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = format!("insert into goods (qrcode, name, cover, price, unit) values (:qrcode, :name, :cover, :price, :unit)");
        let params = rusqlite::named_params!{
            ":qrcode": &goods.qrcode,
            ":name": &goods.name,
            ":cover": &goods.cover,
            ":price": &goods.price,
            ":unit": &goods.unit,
        };

        conn.execute(&sql, params).unwrap();
        conn.last_insert_rowid()
    }).await.unwrap()
}

#[cfg(test)]
mod goods_mapper_test {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    #[test]
    fn test2() {
        let dec = Decimal::from_str("19.023").unwrap();
        println!("{:?}", dec);
    }
    
    #[test]
    fn test3() {
        let timestamp = chrono::Local::now().timestamp();
        println!("{}", timestamp);
    }
    
}
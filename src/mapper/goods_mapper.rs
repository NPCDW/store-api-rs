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
pub async fn count(name: Option<String>) -> usize {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = format!("SELECT count(*) FROM goods {}", if name.is_none() { "" } else { "WHERE name like '%'||:name||'%'" });
        let params = rusqlite::named_params!{":name": &name};

        let mut stmt = conn.prepare(&sql).unwrap();
        let mut rows = stmt.query(params).unwrap();
        let row = rows.next().unwrap().unwrap();
        row.get(0)
    }).await.unwrap().unwrap()
}

#[tokio::main]
pub async fn list(page_number: usize, page_size: usize, name: Option<String>) -> Vec<Goods> {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = format!("SELECT count(*) FROM goods {} LIMIT :page_size OFFSET :start", if name.is_none() { "" } else { "WHERE name like '%'||:name||'%'" });
        let params = rusqlite::named_params!{
            ":name": &name,
            ":page_size": &page_size,
            ":start": (page_number - 1) * page_size,
        };

        let mut stmt = conn.prepare(&sql).unwrap();
        let rows = stmt.query_map(params, row_to_entity).unwrap();
        let mut result = vec![];
        for item in rows {
            result.push(item.unwrap());
        }
        result
    }).await.unwrap()
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

#[tokio::main]
pub async fn insert(goods: Goods) -> i64 {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = "insert into goods (qrcode, name, cover, price, unit) values (:qrcode, :name, :cover, :price, :unit)";
        let params = rusqlite::named_params!{
            ":qrcode": &goods.qrcode,
            ":name": &goods.name,
            ":cover": &goods.cover,
            ":price": &goods.price,
            ":unit": &goods.unit,
        };

        conn.execute(sql, params).unwrap();
        conn.last_insert_rowid()
    }).await.unwrap()
}

#[tokio::main]
pub async fn update(goods: Goods) -> usize {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = "update goods set update_time = :update_time, qrcode = :qrcode, name = :name, cover = :cover, price = :price, unit = :unit where id = :id";
        let params = rusqlite::named_params!{
            ":id": &goods.id,
            ":update_time": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ":qrcode": &goods.qrcode,
            ":name": &goods.name,
            ":cover": &goods.cover,
            ":price": &goods.price,
            ":unit": &goods.unit,
        };

        conn.execute(sql, params).unwrap()
    }).await.unwrap()
}

#[tokio::main]
pub async fn delete(id: u32) -> usize {
    let conn = DB_CONN_POOL.get().await.unwrap();
    conn.interact(move |conn| {
        let sql = "delete from goods where id = :id";
        let params = rusqlite::named_params!{":id": &id};

        conn.execute(sql, params).unwrap()
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
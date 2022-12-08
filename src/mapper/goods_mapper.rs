use sea_query::{Expr, Iden, Query, SqliteQueryBuilder, Func, Order};
use sea_query_rusqlite::RusqliteBinder;

use crate::{model::entity::goods::Goods, config::db_config::DB_CONN_POOL};

pub enum GoodsFields {
    Table, Id, CreateTime, UpdateTime, Qrcode, Name, Cover, Price, Unit
}

impl Iden for GoodsFields {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}",
            match self {
                Self::Table => "goods",
                Self::Id => "id",
                Self::CreateTime => "create_time",
                Self::UpdateTime => "update_time",
                Self::Qrcode => "qrcode",
                Self::Name => "name",
                Self::Cover => "cover",
                Self::Price => "price",
                Self::Unit => "unit",
            }
        ).unwrap();
    }
}

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

pub fn count(name: Option<String>) -> usize {
    let (sql, params) = Query::select()
        .from(GoodsFields::Table)
        .conditions(
            name.is_none(),
            |_| {},
            |q| {
                q.and_where(Expr::col(GoodsFields::Name).like(format!("%{}%", name.unwrap())));
            },
        )
        .expr(Func::count(Expr::col(GoodsFields::Id)))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    let mut stmt = conn.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();
    let row = rows.next().unwrap().unwrap();
    row.get(0).unwrap()
}

pub fn list(page_number: u64, page_size: u64, name: Option<String>) -> Vec<Goods> {
    let (sql, params) = Query::select()
        .columns([
            GoodsFields::Id,
            GoodsFields::CreateTime,
            GoodsFields::UpdateTime,
            GoodsFields::Qrcode,
            GoodsFields::Name,
            GoodsFields::Cover,
            GoodsFields::Price,
            GoodsFields::Unit,
        ])
        .from(GoodsFields::Table)
        .conditions(
            name.is_none(),
            |_| {},
            |q| {
                q.and_where(Expr::col(GoodsFields::Name).like(format!("%{}%", name.unwrap())));
            },
        )
        .order_by(GoodsFields::CreateTime, Order::Desc)
        .limit(page_size).offset((page_number - 1) * page_size)
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map(&*params.as_params(), row_to_entity).unwrap();
    let mut result = vec![];
    for item in rows {
        result.push(item.unwrap());
    }
    result
}

pub fn get_by_id(id: u32) -> Option<Goods> {
    let (sql, params) = Query::select()
        .columns([
            GoodsFields::Id,
            GoodsFields::CreateTime,
            GoodsFields::UpdateTime,
            GoodsFields::Qrcode,
            GoodsFields::Name,
            GoodsFields::Cover,
            GoodsFields::Price,
            GoodsFields::Unit,
        ])
        .from(GoodsFields::Table)
        .and_where(Expr::col(GoodsFields::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut result = stmt.query_map(&*params.as_params(), row_to_entity).unwrap();
    let res = result.next();
    if res.is_none() {
        return None;
    }
    Some(res.unwrap().unwrap())
}

pub fn get_by_qrcode(qrcode: String) -> Option<Goods> {
    let (sql, params) = Query::select()
        .columns([
            GoodsFields::Id,
            GoodsFields::CreateTime,
            GoodsFields::UpdateTime,
            GoodsFields::Qrcode,
            GoodsFields::Name,
            GoodsFields::Cover,
            GoodsFields::Price,
            GoodsFields::Unit,
        ])
        .from(GoodsFields::Table)
        .and_where(Expr::col(GoodsFields::Qrcode).eq(qrcode))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut result = stmt.query_map(&*params.as_params(), row_to_entity).unwrap();
    let res = result.next();
    if res.is_none() {
        return None;
    }
    Some(res.unwrap().unwrap())
}

pub fn insert(goods: Goods) -> i64 {
    let (sql, params) = Query::insert()
        .into_table(GoodsFields::Table)
        .columns([
            GoodsFields::Qrcode,
            GoodsFields::Name,
            GoodsFields::Cover,
            GoodsFields::Price,
            GoodsFields::Unit,
        ])
        .values_panic([
            goods.qrcode.into(),
            goods.name.into(),
            goods.cover.into(),
            goods.price.into(),
            goods.unit.into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    conn.execute(&sql, &*params.as_params()).unwrap();
    conn.last_insert_rowid()
}

pub fn update(goods: Goods) -> usize {
    let (sql, params) = Query::update()
        .table(GoodsFields::Table)
        .values([
            (GoodsFields::UpdateTime, chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().into()),
            (GoodsFields::Qrcode, goods.qrcode.into()),
            (GoodsFields::Name, goods.name.into()),
            (GoodsFields::Cover, goods.cover.into()),
            (GoodsFields::Price, goods.price.into()),
            (GoodsFields::Unit, goods.unit.into()),
        ])
        .and_where(Expr::col(GoodsFields::Id).eq(goods.id))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get().unwrap();
    conn.execute(&sql, &*params.as_params()).unwrap()
}

pub fn delete(id: u32) -> usize {
    let sql = "delete from goods where id = :id";
    let params = rusqlite::named_params!{":id": &id};

    let conn = DB_CONN_POOL.get().unwrap();
    conn.execute(sql, params).unwrap()
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
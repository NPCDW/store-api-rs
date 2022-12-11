use chrono::prelude::*;
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
    let create_time: String = row.get("create_time")?;
    let update_time: String = row.get("update_time")?;
    Ok(Goods {
        id: row.get("id")?,
        create_time: Some(NaiveDateTime::parse_from_str(&create_time, "%Y-%m-%d %H:%M:%S").unwrap()),
        update_time: Some(NaiveDateTime::parse_from_str(&update_time, "%Y-%m-%d %H:%M:%S").unwrap()),
        qrcode: row.get("qrcode")?,
        name: row.get("name")?,
        cover: row.get("cover")?,
        price: row.get("price")?,
        unit: row.get("unit")?,
    })
}

pub fn count(name: Option<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let (sql, params) = Query::select()
        .from(GoodsFields::Table)
        .conditions(
            name.is_some(), |q| {
                q.and_where(Expr::col(GoodsFields::Name).like(format!("%{}%", name.unwrap())));
            }, |_| {}
        )
        .expr(Func::count(Expr::col(GoodsFields::Id)))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get()?;
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*params.as_params())?;
    let row = rows.next()?.unwrap();
    Ok(row.get(0)?)
}

pub fn list(page_number: u64, page_size: u64, name: Option<String>) -> Result<Vec<Goods>, Box<dyn std::error::Error>> {
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
            name.is_none(), |q| {
                q.and_where(Expr::col(GoodsFields::Name).like(format!("%{}%", name.unwrap())));
            }, |_| {}
        )
        .order_by(GoodsFields::CreateTime, Order::Desc)
        .limit(page_size).offset((page_number - 1) * page_size)
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get()?;
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(&*params.as_params(), row_to_entity)?;
    let mut result = vec![];
    for item in rows {
        result.push(item?);
    }
    Ok(result)
}

pub fn get_by_id(id: u32) -> Result<Option<Goods>, Box<dyn std::error::Error>> {
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

    let conn = DB_CONN_POOL.get()?;
    let mut stmt = conn.prepare(&sql)?;
    let mut result = stmt.query_map(&*params.as_params(), row_to_entity)?;
    let res = result.next();
    if res.is_none() {
        return Ok(None);
    }
    Ok(Some(res.unwrap()?))
}

pub fn get_by_qrcode(qrcode: String) -> Result<Option<Goods>, Box<dyn std::error::Error>> {
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

    let conn = DB_CONN_POOL.get()?;
    let mut stmt = conn.prepare(&sql)?;
    let mut result = stmt.query_map(&*params.as_params(), row_to_entity)?;
    let res = result.next();
    if res.is_none() {
        return Ok(None);
    }
    Ok(Some(res.unwrap()?))
}

pub fn insert(goods: Goods) -> Result<i64, Box<dyn std::error::Error>> {
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

    let conn = DB_CONN_POOL.get()?;
    conn.execute(&sql, &*params.as_params())?;
    Ok(conn.last_insert_rowid())
}

pub fn update(goods: Goods) -> Result<usize, Box<dyn std::error::Error>> {
    let mut values = vec![];
    values.push((GoodsFields::UpdateTime, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().into()));
    if goods.qrcode.is_some() {
        values.push((GoodsFields::Qrcode, goods.qrcode.into()));
    }
    if goods.name.is_some() {
        values.push((GoodsFields::Name, goods.name.into()));
    }
    if goods.cover.is_some() {
        values.push((GoodsFields::Cover, goods.cover.into()));
    }
    if goods.price.is_some() {
        values.push((GoodsFields::Price, goods.price.into()));
    }
    if goods.unit.is_some() {
        values.push((GoodsFields::Unit, goods.unit.into()));
    }

    let (sql, params) = Query::update()
        .table(GoodsFields::Table)
        .values(values)
        .and_where(Expr::col(GoodsFields::Id).eq(goods.id))
        .build_rusqlite(SqliteQueryBuilder);

    let conn = DB_CONN_POOL.get()?;
    let res = conn.execute(&sql, &*params.as_params())?;
    Ok(res)
}

pub fn delete(id: u32) -> Result<usize, Box<dyn std::error::Error>> {
    let sql = "delete from goods where id = :id";
    let params = rusqlite::named_params!{":id": &id};

    let conn = DB_CONN_POOL.get()?;
    let res = conn.execute(sql, params)?;
    Ok(res)
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
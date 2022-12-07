use std::path::{Path};

extern crate r2d2;
extern crate r2d2_sqlite;

use crate::util::file_util;

lazy_static! {
    #[derive(Debug)]
    pub static ref DB_VERSION_LIST: Vec<String> = {
        let mut list = vec![];
        list.push("2022-12-02-01".to_string());
        list
    };
    pub static ref DB_CONN_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = {
        tracing::info!("Init DB Connect Pool");
        let db_filepath = Path::new(&crate::APP_CONFIG.db.sqlite.filepath);
        let manager = r2d2_sqlite::SqliteConnectionManager::file(db_filepath);
        let pool = r2d2::Pool::builder()
            .max_size(crate::APP_CONFIG.db.sqlite.max_pool_size)
            .build(manager).unwrap_or_else(|e| {
                panic!("创建线程池失败，{:?}", e);
            });
        tracing::info!("Finish Init DB Connect Pool");
        pool
    };
}

pub fn init() {
    init_file();

    migrate_db();
}

fn init_file() {
    let filepath = &crate::APP_CONFIG.db.sqlite.filepath;
    let db_filepath = Path::new(&filepath);
    file_util::create_file(db_filepath);
}

fn migrate_db() {
    tracing::info!("Migrate DB");
    let conn = DB_CONN_POOL.get().unwrap();
    let mut stmt = conn.prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name = 'version'").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let exist: i64 = row.get(0).unwrap();
    let current_version: String;
    if exist <= 0 {
        current_version = "".to_string();
    } else {
        let mut stmt = conn.prepare("SELECT version FROM version order by id desc limit 1").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        current_version = row.get(0).unwrap();
    }
    if current_version[..] == DB_VERSION_LIST.last().unwrap()[..] {
        tracing::info!("DB version is match, current version {}", &current_version);
        return;
    }
    tracing::info!("DB version require {}, Current version is {}, start upgrade db", &DB_VERSION_LIST.last().unwrap(), &current_version);
    for item in &*DB_VERSION_LIST {
        if current_version[..] < item[..] {
            let dir = std::env::current_dir().unwrap_or_else(|e| {
                panic!("获取程序目录失败：{:?}", e);
            });
            let sql_file_path = dir.join("resources/db/").join(format!("{}.sql", item));
            let sql = file_util::read_file(&sql_file_path).unwrap();
            let result = conn.execute_batch(&sql);
            if result.is_err() {
                tracing::error!("{:?}", result);
            };
        }
    }
    tracing::info!("DB upgrade finished");
}
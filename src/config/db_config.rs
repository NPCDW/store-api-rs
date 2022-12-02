use std::path::Path;
use deadpool_sqlite::{Config, Runtime, Pool, PoolConfig};

use crate::util::file_util;

lazy_static! {
    #[derive(Debug)]
    pub static ref DB_VERSION_LIST: Vec<String> = {
        let mut list = vec![];
        list.push("2022-12-02-01.sql".to_string());
        list
    };
}

pub fn init() -> Pool {
    let filepath = &crate::APP_CONFIG.db.sqlite.filepath;
    let path = Path::new(&filepath);

    init_file(path);

    let pool = init_pool(path);

    migrate_db(&pool);

    pool
}

fn init_file(path: &Path) {
    file_util::create_file(path);
}

fn init_pool(path: &Path) -> Pool {
    // let cfg = Config::new(path);
    let cfg = Config {
        path: path.to_path_buf(),
        pool: Some(PoolConfig {
            max_size: crate::APP_CONFIG.db.sqlite.max_pool_size,
            ..Default::default()
        })
    };
    cfg.create_pool(Runtime::Tokio1).unwrap()
}

#[tokio::main]
async fn migrate_db(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    let exist: i64 = conn
        .interact(|conn| {
            let mut stmt = conn.prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name = 'version'")?;
            let mut rows = stmt.query([])?;
            let row = rows.next()?.unwrap();
            row.get(0)
        }).await.unwrap().unwrap();
    let current_version: String;
    if exist <= 0 {
        current_version = "".to_string();
    } else {
        current_version = conn
            .interact(|conn| {
                let mut stmt = conn.prepare("SELECT version FROM version order by id desc limit 1")?;
                let mut rows = stmt.query([])?;
                let row = rows.next()?.unwrap();
                row.get(0)
            }).await.unwrap().unwrap();
    }
    for item in &*DB_VERSION_LIST {
        if current_version[..] < item[..] {
            let dir = std::env::current_dir().unwrap_or_else(|e| {
                panic!("获取程序目录失败：{:?}", e);
            });
            let sql = file_util::read_file(&dir.join("resources/db/").join(item)).unwrap();
            // println!("{}", sql);
            conn
            .interact(move |conn| {
                let _ = conn.execute_batch(&sql);
            }).await.unwrap();
        }
    }
}
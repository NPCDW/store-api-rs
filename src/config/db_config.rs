use std::path::Path;
use deadpool_sqlite::{Config, Runtime, Pool, PoolConfig};

use crate::util::file_util;

lazy_static! {
    #[derive(Debug)]
    pub static ref DB_VERSION_LIST: Vec<String> = {
        let mut list = vec![];
        list.push("2022-12-02-01".to_string());
        list
    };
}

pub fn init() -> Pool {
    tracing::info!("Init DB Config");

    let filepath = &crate::APP_CONFIG.db.sqlite.filepath;
    let path = Path::new(&filepath);

    init_file(path);

    let pool = init_pool(path);

    migrate_db(&pool);

    tracing::info!("Finish Init DB Config");
    pool
}

fn init_file(path: &Path) {
    file_util::create_file(path);
}

fn init_pool(path: &Path) -> Pool {
    tracing::info!("Init DB Connect Pool");
    // let cfg = Config::new(path);
    let cfg = Config {
        path: path.to_path_buf(),
        pool: Some(PoolConfig {
            max_size: crate::APP_CONFIG.db.sqlite.max_pool_size,
            ..Default::default()
        })
    };
    let pool = cfg.create_pool(Runtime::Tokio1).unwrap();
    tracing::info!("Finish Init DB Connect Pool");
    pool
}

#[tokio::main]
async fn migrate_db(pool: &Pool) {
    tracing::info!("Migrate DB");
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
            let sql_path = dir.join("resources/db/").join(format!("{}.sql", item));
            let sql = file_util::read_file(&sql_path).unwrap();
            conn
            .interact(move |conn| {
                let result = conn.execute_batch(&sql);
                if result.is_err() {
                    tracing::error!("{:?}", result);
                };
            }).await.unwrap();
        }
    }
    tracing::info!("DB upgrade finished");
}
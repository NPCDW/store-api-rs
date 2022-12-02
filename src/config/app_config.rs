use serde::{Serialize, Deserialize};

use crate::util::file_util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sqlite {
    pub filepath: String,
    pub max_pool_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    pub sqlite: Sqlite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub bind: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub auth: Auth,
    pub db: Db,
    pub log: Log,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref APP_CONFIG: Config = {
        let dir = std::env::current_dir().unwrap_or_else(|e| {
            panic!("获取程序目录失败：{:?}", e);
        });
        let filepath = dir.join("resources/config/config.yml");
        let buf = file_util::read_file(&filepath).unwrap_or_else(|e| {
            panic!("读取配置文件 {} 失败：{:?}", filepath.display(), e)
        });
        serde_yaml::from_str(&buf).unwrap_or_else(|e| {
            panic!("配置文件 {} 转 yaml 格式失败：{:?}", filepath.display(), e);
        })
    };
}

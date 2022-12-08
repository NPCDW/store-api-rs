use serde::{Serialize, Deserialize};

use crate::util::file_util;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigLog {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigDbSqlite {
    pub filepath: String,
    pub max_pool_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigDb {
    pub sqlite: AppConfigDbSqlite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigAuth {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigServer {
    pub bind: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: AppConfigServer,
    pub auth: AppConfigAuth,
    pub db: AppConfigDb,
    pub log: AppConfigLog,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref APP_CONFIG: AppConfig = {
        println!("Get APP Config");
        let dir = std::env::current_dir().unwrap_or_else(|e| {
            panic!("获取程序目录失败：{:?}", e);
        });
        let filepath = dir.join("resources/config/config.yml");
        let buf = file_util::read_file(&filepath).unwrap_or_else(|e| {
            panic!("读取配置文件 {} 失败：{:?}", filepath.display(), e)
        });
        let config = serde_yaml::from_str(&buf).unwrap_or_else(|e| {
            panic!("配置文件 {} 转 yaml 格式失败：{:?}", filepath.display(), e);
        });
        println!("Finish Get APP Config");
        config
    };
}

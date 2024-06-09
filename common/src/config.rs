use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub log: Log,
}


#[derive(Debug, Deserialize)]
pub struct Database {
    // 数据库url
    pub url: String,
    // 连接池最大连接数
    pub max_connections: u32,
    // 连接池最小连接数
    pub min_connections: u32,
    // 连接数据库超时时间（秒）
    pub connect_timeout: u64,
    // 查询数据超时时间（秒）
    pub idle_timeout: u64,
    // 连接存活时间（秒）
    pub max_lifetime: u64,
    // 是否打印查询语句
    pub sqlx_logging: bool,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub debug: bool,
}

impl Config {

    /// 默认读取项目目录下的配置文件config.toml
    pub fn init_config()-> Config {
        // 配置文件默认路径
        let file_path = "config.toml";
       Self::init_config_path(file_path)
    }

    /// 读取项目目录下的配置文件
    pub fn init_config_path(path:&str) -> Self {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", path, e)
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e)
        };
        toml::from_str(&str_val).expect("Parsing the toml file failed")
    }
}

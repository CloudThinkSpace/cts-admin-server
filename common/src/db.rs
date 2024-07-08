pub mod db_type;
pub mod form;
pub mod select;

use std::collections::HashMap;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;
use crate::config::Config;
use crate::db::db_type::DbType;

static GLOBAL_DB: OnceCell<DatabaseConnection> = OnceCell::const_new();


/// 获取数据连接对象
pub async fn get_db() -> DatabaseConnection {
    GLOBAL_DB.get_or_init(|| async {
        let config = Config::init_config();
        get_init_db_pool(&config).await
    }).await.clone()
}

/// 初始化数据库链接函数
pub async fn get_init_db_pool(config: &Config) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&config.database.url);
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(config.database.connect_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime))
        .sqlx_logging(true);

    Database::connect(opt).await.expect("连接数据库失败")
}

/// 创建表sql函数
pub fn create_table_sql(table_name: &str, fields: &Vec<String>, is_form: bool) -> String {
    let mut create_sql = format!("create table if not exists {}(", table_name);
    // 添加 主键
    create_sql.push_str("id varchar not null primary key,");
    for field in fields.iter() {
        create_sql.push_str(&format!("{} varchar,", field));
    }

    if is_form {
        // 表单公共字段
        let common_fields = create_common_fields();
        for common_field in common_fields.into_iter() {
            create_sql.push_str(common_field);
        }
    } else {
        // 移除最后一位逗号
        let current_sql = &create_sql[..create_sql.len() - 1];
        create_sql = current_sql.to_string();
    }
    create_sql.push_str(");");

    create_sql
}

/// 插入数据sql函数
pub fn insert_data_sql(table_name: &str, fields: &Vec<String>, data: &Vec<Box<dyn DbType>>) -> String {
    let mut sql = format!("INSERT INTO {} ", table_name);
    sql.push_str("(");
    for field in fields.iter() {
        sql.push_str(&format!("{},", field));
    }
    // 移除最后的逗号
    let current_sql = &sql[..sql.len() - 1];
    let mut sql = current_sql.to_string();
    sql.push_str(") VALUES ( ");
    // 遍历数据
    for datum in data.iter() {
        sql.push_str(&format!("{},", datum.display()));
    }
    // 移除最后的逗号
    let current_sql = &sql[..sql.len() - 1];
    let mut sql = current_sql.to_string();
    sql.push_str(")");
    sql
}

/// 更新数据sql函数
pub fn update_data_sql(table_name: &str, id: &str, data: HashMap<String, Box<dyn DbType>>) -> String {
    let mut sql = format!("UPDATE {} SET ", table_name);
    for (key, value) in data.into_iter() {
        sql.push_str(&format!("{}={},", key, value.display()));
    }
    // 移除最后的逗号
    let current_sql = &sql[..sql.len() - 1];
    let mut sql = current_sql.to_string();
    sql.push_str(&format!(" WHERE id = '{}'", id));
    sql
}

fn create_common_fields() -> Vec<&'static str> {
    let result = vec![
        "code varchar,",
        "lon varchar,",
        "lat varchar,",
        "status integer,",
        "user_id varchar,",
        "created_at timestamp not null,",
        "updated_at timestamp,",
        "deleted_at timestamp",
    ];
    result
}
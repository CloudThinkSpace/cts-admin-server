use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::SchemaManager;
use tokio::sync::OnceCell;
use crate::config::Config;
static GLOBAL_DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> DatabaseConnection {
    GLOBAL_DB.get_or_init(|| async {
        let config = Config::init_config();
        get_init_db_pool(&config).await
    }).await.clone()
}

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

pub async fn execute_insert(manager: &SchemaManager<'_>, ) {

}
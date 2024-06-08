use web::{log, server};

#[tokio::main]
async fn main() {
    // 日志初始化
    log::init_log();
    server::start().await;
}


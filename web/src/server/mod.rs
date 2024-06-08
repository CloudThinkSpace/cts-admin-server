use tracing::info;
use crate::route::api;

/// 启动服务
pub async  fn start() {
    // 系统api
    let app = api();
    // 监听
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // 打印监听地震
    info!("listening on {}", listener.local_addr().unwrap());
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
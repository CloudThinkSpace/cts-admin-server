use axum::Router;
use axum::routing::{get, post};
use crate::handler::base::upload_download;

/// 基础路由
/// @author tanghy
/// login 登录路由
/// logout 登出路由
/// register 注册路由
pub fn upload_download_route() -> Router {
    Router::new()
        .route("/upload", post(upload_download::upload))
        .route("/download", get(upload_download::download))
}
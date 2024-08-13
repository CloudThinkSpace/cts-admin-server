use crate::handler::base::upload_download;
use axum::routing::{get, post};
use axum::Router;

/// 基础路由
/// @author tanghy
/// login 登录路由
/// logout 登出路由
/// register 注册路由
pub fn upload_download_route() -> Router {
    Router::new()
        .route("/upload", post(upload_download::upload))
        .route("/image/*path", get(upload_download::image))
        .route("/download/*path", get(upload_download::download))
}


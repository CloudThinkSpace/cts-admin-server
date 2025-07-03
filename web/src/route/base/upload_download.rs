use axum::routing::get;
use axum::Router;

use crate::handler::base::upload_download;

/// 基础路由
/// @author tanghy
/// login 登录路由
/// logout 登出路由
/// register 注册路由
pub fn image_route() -> Router {
    Router::new().route("/image/*path", get(upload_download::image))
}

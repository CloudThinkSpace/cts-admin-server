use axum::Router;
use axum::routing::{get, post};
use crate::handler::base;

/// 基础路由
/// @author tanghy
/// login 登录路由
/// logout 登出路由
/// register 注册路由
pub fn login_route() -> Router {
    Router::new()
        .route("/login", post(base::login))
        .route("/logout", get(base::logout))
        .route("/register", post(base::register))
}
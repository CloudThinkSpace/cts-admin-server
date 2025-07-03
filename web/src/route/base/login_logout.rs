use axum::routing::{get, post};
use axum::Router;

use crate::handler::base::login_logout::{login, logout, register};

/// 基础路由
/// @author tanghy
/// login 登录路由
/// logout 登出路由
/// register 注册路由
pub fn login_route() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/register", post(register))
}

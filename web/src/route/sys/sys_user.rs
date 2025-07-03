use crate::handler::sys::sys_user;
use axum::routing::{get, post};
use axum::Router;

/// 用户路由
/// @author tanghy
///
pub fn user_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_user::add))
        .route("/update/:id", post(sys_user::update))
        .route("/delete/:id", get(sys_user::delete))
        .route("/query/:id", get(sys_user::query))
        .route("/getUserInfo", get(sys_user::get_user_info))
        .route("/status/:id", post(sys_user::update_status))
        .route("/search", post(sys_user::search));

    Router::new().nest("/user", router)
}


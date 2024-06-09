use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_user;

/// 用户路由
/// @author tanghy
///
pub fn user_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_user::add))
        .route("/update", post(sys_user::update))
        .route("/delete", get(sys_user::delete))
        .route("/query/:id", get(sys_user::query))
        .route("/search", post(sys_user::search));

    Router::new()
        .nest("/user", router)
}
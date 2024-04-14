use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_role;

/// 角色路由
/// @author tanghy
///
pub fn role_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_role::add))
        .route("/update", post(sys_role::update))
        .route("/delete", get(sys_role::delete))
        .route("/query", get(sys_role::query))
        .route("/search", post(sys_role::search));

    Router::new()
        .nest("/role", router)
}
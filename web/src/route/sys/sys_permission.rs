use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_permission;

/// 权限路由
/// @author tanghy
///
pub fn permission_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_permission::add))
        .route("/update", post(sys_permission::update))
        .route("/delete", get(sys_permission::delete))
        .route("/query", get(sys_permission::query))
        .route("/search", post(sys_permission::search));

    Router::new()
        .nest("/permission", router)
}
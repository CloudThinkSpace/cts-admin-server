use crate::handler::sys::sys_permission;
use axum::routing::{get, post};
use axum::Router;

/// 权限路由
/// @author tanghy
///
pub fn permission_route() -> Router {
    let router = Router::new()
        .route("/set/menu", post(sys_permission::set_menu))
        .route("/set/api", post(sys_permission::set_api))
        .route("/search/menu", get(sys_permission::search_menu))
        .route("/query/menu/:id", get(sys_permission::get_menu_ids))
        .route("/query/api/:id", get(sys_permission::get_api_ids))
        .route("/search/api", get(sys_permission::search_api));

    Router::new().nest("/authorize", router)
}

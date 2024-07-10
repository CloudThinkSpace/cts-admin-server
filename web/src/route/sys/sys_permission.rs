use axum::Router;
use axum::routing::post;
use crate::handler::sys::sys_permission;

/// 权限路由
/// @author tanghy
///
pub fn permission_route() -> Router {
    let router = Router::new()
        .route("/set/menu", post(sys_permission::set_menu))
        .route("/set/api", post(sys_permission::set_api))
        .route("/search/menu", post(sys_permission::search_menu))
        .route("/search/api", post(sys_permission::search_api));

    Router::new()
        .nest("/authorize", router)
}

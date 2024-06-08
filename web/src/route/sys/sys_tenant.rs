use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_tenant;

/// 用户路由
/// @author tanghy
///
pub fn domain_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_tenant::add))
        .route("/update", post(sys_tenant::update))
        .route("/delete", get(sys_tenant::delete))
        .route("/query", get(sys_tenant::query))
        .route("/search", post(sys_tenant::search));

    Router::new()
        .nest("/domain", router)
}
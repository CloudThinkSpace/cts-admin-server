use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_domain;

/// 用户路由
/// @author tanghy
///
pub fn domain_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_domain::add))
        .route("/update", post(sys_domain::update))
        .route("/delete", get(sys_domain::delete))
        .route("/query", get(sys_domain::query))
        .route("/search", post(sys_domain::search));

    Router::new()
        .nest("/domain", router)
}
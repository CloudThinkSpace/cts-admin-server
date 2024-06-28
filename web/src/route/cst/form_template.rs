use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_api;

/// 表单模板路由
/// @author tanghy
///
pub fn form_template_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_api::add))
        .route("/update/:id", post(sys_api::update))
        .route("/delete/:id", get(sys_api::delete))
        .route("/query/:id", get(sys_api::query))
        .route("/search", post(sys_api::search));

    Router::new()
        .nest("/form/template", router)
}
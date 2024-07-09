use axum::Router;
use axum::routing::{get, post};
use crate::handler::cts::form;

/// 表单数据路由
/// @author tanghy
///
pub fn form_data_route() -> Router {
    let router = Router::new()
        .route("/add/:table_id", post(form::add))
        .route("/update/:table_id/:id", post(form::update))
        .route("/delete/:table_id/:id", get(form::delete))
        .route("/query/:table_id/:id", get(form::query))
        .route("/search/:table_id", post(form::search));

    Router::new()
        .nest("/form", router)
}
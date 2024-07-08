use axum::Router;
use axum::routing::{get, post};
use crate::handler::cts::task;

/// 任务路由
/// @author tanghy
///
pub fn task_route() -> Router {
    let router = Router::new()
        .route("/add/:table_id", post(task::add))
        .route("/update/:table_id/:id", post(task::update))
        .route("/delete/:table_id/:id", get(task::delete))
        .route("/query/:table_id/:id", get(task::query))
        .route("/search/:table_id", post(task::search));

    Router::new()
        .nest("/task", router)
}
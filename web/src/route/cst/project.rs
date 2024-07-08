use axum::Router;
use axum::routing::{get, post};
use crate::handler::cts::project;

/// 项目路由
/// @author tanghy
///
pub fn project_route() -> Router {
    let router = Router::new()
        .route("/add", post(project::add))
        .route("/update/:id", post(project::update))
        .route("/delete/:id", get(project::delete))
        .route("/query/:id", get(project::query))
        .route("/search", post(project::search));

    Router::new()
        .nest("/project", router)
}
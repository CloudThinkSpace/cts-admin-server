use crate::handler::cts::project_layer;
use axum::routing::{get, post};
use axum::Router;

/// 项目图层路由
/// @author tanghy
///
pub fn project_layer_route() -> Router {
    let router = Router::new()
        .route("/add/:projectId", post(project_layer::add))
        .route("/delete/:id", get(project_layer::delete))
        .route("/layers/:projectId", post(project_layer::layers));

    Router::new().nest("/projectLayer", router)
}

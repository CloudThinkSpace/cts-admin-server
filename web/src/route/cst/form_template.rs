use axum::Router;
use axum::routing::{get, post};
use crate::handler::cts::form_template;

/// 表单模板路由
/// @author tanghy
///
pub fn form_template_route() -> Router {
    let router = Router::new()
        .route("/add", post(form_template::add))
        .route("/update/:id", post(form_template::update))
        .route("/delete/:id", get(form_template::delete))
        .route("/query/:id", get(form_template::query))
        .route("/search", post(form_template::search));

    Router::new()
        .nest("/form/template", router)
}
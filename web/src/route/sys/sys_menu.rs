use axum::Router;
use axum::routing::{get, post};
use crate::handler::sys::sys_menu;

/// 菜单路由
/// @author tanghy
///
pub fn menu_route() -> Router {
    let router = Router::new()
        .route("/add", post(sys_menu::add))
        .route("/update", post(sys_menu::update))
        .route("/delete", get(sys_menu::delete))
        .route("/query", get(sys_menu::query))
        .route("/search", post(sys_menu::search));

    Router::new()
        .nest("/menu", router)
}
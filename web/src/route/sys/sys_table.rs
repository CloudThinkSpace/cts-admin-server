use axum::{routing::get, Router};

use crate::handler::sys::sys_table;

/// 表结构路由
/// @author tanghy
///
pub fn table_route() -> Router {
    let router = Router::new().route("/query/:id", get(sys_table::get_table_fields));

    Router::new().nest("/table", router)
}

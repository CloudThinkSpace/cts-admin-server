use crate::route::base::login_logout::login_route;
use crate::route::cst::form::form_data_route;
use crate::route::cst::form_template::form_template_route;
use crate::route::cst::project::project_route;
use crate::route::cst::task::task_route;
use crate::route::sys::sys_api::api_route;
use crate::route::sys::sys_menu::menu_route;
use crate::route::sys::sys_permission::permission_route;
use crate::route::sys::sys_role::role_route;
use crate::route::sys::sys_tenant::domain_route;
use crate::route::sys::sys_user::user_route;
use crate::route::sys::upload_download::upload_download_route;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::middleware as axum_middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use middleware::layers as my_layers;
use response_utils::res::ResResult;

use self::sys::sys_table::table_route;

pub mod base;
pub mod cst;
pub mod sys;

/// 系统api router
/// 该api包含授权和非授权
pub fn api() -> Router {
    let all_router = Router::new()
        // 合并无需认证api
        .merge(no_auth_api())
        // 合并需要认证api
        .merge(auth_api());

    Router::new()
        // .nest("/api", all_router)
        .merge(all_router)
        .route("/", get(|| async { "Hello CloudThinkSpace!" }))
        .fallback(handler_404)
}

/// 需要认证api
fn auth_api() -> Router {
    let router = Router::new()
        // 合并用户路由
        .merge(user_route())
        // 合并菜单路由
        .merge(menu_route())
        // 合并角色路由
        .merge(role_route())
        // 合并区域路由
        .merge(domain_route())
        // 合并api路由
        .merge(api_route())
        // 上传文件服务
        .merge(upload_download_route())
        // 表查询接口
        .merge(table_route())
        // 合并权限路由
        .merge(permission_route());

    let cts_router: Router = Router::new()
        // 表单路由
        .merge(form_template_route())
        .merge(task_route())
        .merge(form_data_route())
        .merge(project_route());

    Router::new()
        .nest("/sys", router)
        .nest("/cts", cts_router)
        .route_layer(axum_middleware::from_fn(my_layers::auth_layer::auth))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}

/// 无需认证api
fn no_auth_api() -> Router {
    Router::new().merge(login_route())
}

/// 服务错误处理函数
pub async fn handler_404() -> impl IntoResponse {
    ResResult::<()>::with_error_code("nothing to see here", 500, StatusCode::NOT_FOUND)
}

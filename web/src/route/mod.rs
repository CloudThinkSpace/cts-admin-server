use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use response_utils::res::ResResult;
use crate::route::sys::sys_api::api_route;
use crate::route::sys::sys_tenant::domain_route;
use crate::route::sys::sys_menu::menu_route;
use crate::route::sys::sys_permission::permission_route;
use crate::route::sys::sys_role::role_route;
use crate::route::sys::sys_user::user_route;
use middleware::layers as my_layers;
use axum::middleware as axum_middleware;
use crate::route::base::login_logout::login_route;
use crate::route::cst::form_template::form_template_route;
use crate::route::cst::project::project_route;
use crate::route::sys::upload_download::upload_download_route;

pub mod sys;
pub mod base;
pub mod cst;


/// 系统api router
/// 该api包含授权和非授权
pub fn api() -> Router {
    Router::new()
        // 合并无需认证api
        .merge(no_auth_api())
        // 合并需要认证api
        .merge(auth_api())
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
        // 合并权限路由
        .merge(permission_route());

    let cts_router = Router::new()
        // 表单路由
        .merge(form_template_route())
        .merge(project_route());

    Router::new()
        .nest("/sys", router)
        .nest("/cts", cts_router)
        .route_layer(axum_middleware::from_fn(my_layers::auth_layer::auth))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}

/// 无需认证api
fn no_auth_api() -> Router {
    Router::new()
        .merge(login_route())
}

/// 服务错误处理函数
pub async fn handler_404() -> impl IntoResponse {
    ResResult::<()>::with_error_code("nothing to see here", 500, StatusCode::NOT_FOUND)
}

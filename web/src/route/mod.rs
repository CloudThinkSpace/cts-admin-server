use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use response_utils::res::ResResult;
use crate::route::base::login;

pub mod sys;
pub mod base;


/// 系统api router
/// 该api包含授权和非授权
pub fn api() -> Router {
    Router::new()
        // 合并非认证api
        .merge(no_auth_api())
        // 合并需要认证api
        .merge(auth_api())
        .route("/", get(|| async { "Hello CloudThinkSpace!" }))
        .fallback(handler_404)
}

/// 需要认证api
fn auth_api() -> Router {
    Router::new()
}

/// 无需认证api
fn no_auth_api() -> Router {
    Router::new()
        .merge(login())
}

/// 服务错误处理函数
pub async fn handler_404() -> impl IntoResponse {
    ResResult::<()>::with_error_code("nothing to see here", 500, StatusCode::NOT_FOUND)
}

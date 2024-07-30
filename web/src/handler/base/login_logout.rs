use crate::service::base::login_logout;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use models::dto::sys::request::base::Login;
use response_utils::res::ResResult;

/// 登录函数
pub async fn login(Json(data): Json<Login>) -> impl IntoResponse {
    let result = login_logout::login(data.username, data.password).await;
    match result {
        Ok(data) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}

/// 登出函数
pub async fn logout(headers: HeaderMap) -> impl IntoResponse {
    let result = login_logout::logout(headers).await;
    match result {
        Ok(_) => ResResult::with_success("退出成功"),
        Err(_err) => ResResult::<()>::with_error("token无效或已过期"),
    }
}

/// 注册函数
pub async fn register() -> impl IntoResponse {
    ResResult::with_success("注册成功")
}


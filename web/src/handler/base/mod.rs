use axum::response::IntoResponse;
use response_utils::res::ResResult;

/// 登录函数
pub async  fn login() -> impl IntoResponse {
    ResResult::with_success("登录成功")
}

/// 登出函数
pub async  fn logout() -> impl IntoResponse {
    ResResult::with_success("登出成功")
}

/// 注册函数
pub async  fn register() -> impl IntoResponse {
    ResResult::with_success("注册成功")
}
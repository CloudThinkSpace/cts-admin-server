use axum::extract::Path;
use axum::response::IntoResponse;
use response_utils::res::ResResult;
use crate::dao::sys::sys_user::get_by_id;

pub async fn add() -> impl IntoResponse {
    ResResult::with_success("添加用户成功")
}

pub async fn update() -> impl IntoResponse {
    ResResult::with_success("更新用户成功")
}

pub async fn delete() -> impl IntoResponse {
    ResResult::with_success("删除用户成功")
}

pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = get_by_id(id).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询用户失败")
        }
    }
}

pub async fn search() -> impl IntoResponse {
    ResResult::with_success("查询用户成功")
}
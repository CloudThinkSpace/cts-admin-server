use axum::response::IntoResponse;
use response_utils::res::ResResult;

pub async fn add() -> impl IntoResponse {
    ResResult::with_success("添加菜单成功")
}

pub async fn update() -> impl IntoResponse {
    ResResult::with_success("更新菜单成功")
}

pub async fn delete() -> impl IntoResponse {
    ResResult::with_success("删除菜单成功")
}

pub async fn query() -> impl IntoResponse {
    ResResult::with_success("查询菜单成功")
}

pub async fn search() -> impl IntoResponse {
    ResResult::with_success("查询菜单成功")
}
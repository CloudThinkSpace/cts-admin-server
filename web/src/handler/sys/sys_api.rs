use axum::response::IntoResponse;
use response_utils::res::ResResult;

pub async fn add() -> impl IntoResponse {
    ResResult::with_success("添加api成功")
}

pub async fn update() -> impl IntoResponse {
    ResResult::with_success("更新api成功")
}

pub async fn delete() -> impl IntoResponse {
    ResResult::with_success("删除api成功")
}

pub async fn query() -> impl IntoResponse {
    ResResult::with_success("查询api成功")
}

pub async fn search() -> impl IntoResponse {
    ResResult::with_success("查询api成功")
}
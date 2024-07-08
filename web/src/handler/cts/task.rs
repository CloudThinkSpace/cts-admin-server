use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use models::dto::cts::request::task::SearchTask;

use crate::handler::{handle_force, handle_result};
use crate::service::cts::task;

/// 添加task函数
/// @param data 类型AddTenantDto
/// return IntoResponse
pub async fn add(
    Path(table_id): Path<String>,
    Json(data): Json<Value>,
) -> impl IntoResponse {
    let result = task::add(table_id, data).await;
    handle_result(result)
}

/// 更新task函数
/// @param id 类型String
/// @param data 类型UpdateApiDto
/// return IntoResponse
pub async fn update(Path((table_id, id)): Path<(String, String)>, Json(data): Json<Value>) -> impl IntoResponse {
    let result = task::update(table_id, id, data).await;
    handle_result(result)
}


/// 删除task函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path((table_id, id)): Path<(String, String)>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force = handle_force(params);
    let result = task::delete_by_id(table_id, id, force).await;
    handle_result(result)
}

/// 查询task详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path((task_name, id)): Path<(String, String)>) -> impl IntoResponse {
    let result = task::get_by_id(task_name, id).await;
    handle_result(result)
}

/// 分页查询task函数
/// @param data 类型SearchRoleDto
/// return IntoResponse
pub async fn search(Path(table_id): Path<String>, Json(data): Json<SearchTask>) -> impl IntoResponse {
    let result = task::search(table_id, data).await;
    handle_result(result)
}
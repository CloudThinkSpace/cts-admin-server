use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use models::dto::cts::request::form::{AddFormData, SearchFormData, UpdateFormData};

use crate::handler::{handle_force, handle_result};
use crate::service::cts::form;

/// 添加form data函数
/// @param data 类型Value
/// return IntoResponse
pub async fn add(Path(table_id): Path<String>, Json(data): Json<Value>) -> impl IntoResponse {
    let result = form::add(table_id, data).await;
    handle_result(result)
}

/// 添加数据
///
///
pub async fn add_data(Json(data): Json<AddFormData>) -> impl IntoResponse {
    let result = form::add_data(data).await;
    handle_result(result)
}

/// 更新数据
pub async fn update_data(Json(data): Json<UpdateFormData>) -> impl IntoResponse {
    let result = form::update_data(data).await;
    handle_result(result)
}

/// 更新form data函数
/// @param id 类型String
/// @param table_id 数据表id
/// @param data 数据data
/// return IntoResponse
pub async fn update(
    Path((table_id, id)): Path<(String, String)>,
    Json(data): Json<Value>,
) -> impl IntoResponse {
    let result = form::update(table_id, id, data).await;
    handle_result(result)
}

/// 删除form data函数
/// @param table_id 类型String
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(
    Path((table_id, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 判断是否真删除
    let force = handle_force(params);
    let result = form::delete_by_id(table_id, id, force).await;
    handle_result(result)
}

/// 查询form data详情函数
/// @param table_id 类型String, 表编号
/// @param id 类型String，数据编号
/// return IntoResponse
pub async fn query(Path((table_id, id)): Path<(String, String)>) -> impl IntoResponse {
    let result = form::get_by_id(table_id, id).await;
    handle_result(result)
}

/// 查询form data详情函数
/// @param table_id 类型String, 表编号
/// @param code 编号String，数据编号
/// return IntoResponse
pub async fn get_by_code(Path((table_id, code)): Path<(String, String)>) -> impl IntoResponse {
    let result = form::get_by_code(table_id, code).await;
    handle_result(result)
}

/// 分页查询task函数
/// @param data SearchTask
/// return IntoResponse
pub async fn search(
    Path(table_id): Path<String>,
    Json(data): Json<SearchFormData>,
) -> impl IntoResponse {
    let result = form::search(table_id, data).await;
    handle_result(result)
}

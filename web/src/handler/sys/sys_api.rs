use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use models::dto::sys::request::sys_api::{AddApiDto, SearchApiDto, UpdateApiDto};
use crate::handler::{handle_force, handle_result};
use crate::service::sys::sys_api;

/// 添加Api函数
/// @param data 类型AddTenantDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddApiDto>) -> impl IntoResponse {
    let result = sys_api::add(data).await;
    handle_result(result)
}

/// 更新Api函数
/// @param id 类型String
/// @param data 类型UpdateApiDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateApiDto>) -> impl IntoResponse {
    let result = sys_api::update(id, data).await;
    handle_result(result)
}


/// 删除Api函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force= handle_force(params);
    let result = sys_api::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询Api详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_api::get_by_id(id).await;
    handle_result(result)
}

/// 分页查询Api函数
/// @param data 类型SearchRoleDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchApiDto>) -> impl IntoResponse {
    let result = sys_api::search(data).await;
    handle_result(result)
}
use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use models::dto::sys::request::sys_role::{AddRoleDto, SearchRoleDto, UpdateRoleDto};
use crate::handler::{handle_force, handle_result};
use crate::service::sys::sys_role;

/// 添加角色函数
/// @param data 类型AddRoleDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddRoleDto>) -> impl IntoResponse {
    let result = sys_role::add(data).await;
    handle_result(result)
}

/// 更新角色函数
/// @param id 类型String
/// @param data 类型UpdateRoleDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateRoleDto>) -> impl IntoResponse {
    let result = sys_role::update(id, data).await;
    handle_result(result)
}


/// 删除角色函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force= handle_force(params);
    let result = sys_role::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询角色详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_role::get_by_id(id).await;
    handle_result(result)
}

/// 分页查询角色函数
/// @param data 类型SearchRoleDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchRoleDto>) -> impl IntoResponse {
    let result = sys_role::search(data).await;
    handle_result(result)
}
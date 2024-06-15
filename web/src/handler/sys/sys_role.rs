use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use response_utils::res::ResResult;
use models::dto::sys::request::sys_role::{AddRoleDto, SearchRoleDto, UpdateRoleDto};
use crate::handler::handler_force;
use crate::service::sys::sys_role;

/// 添加角色函数
/// @param data 类型AddRoleDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddRoleDto>) -> impl IntoResponse {
    let result = sys_role::add(data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("添加角色失败")
        }
    }
}

/// 更新角色函数
/// @param id 类型String
/// @param data 类型UpdateRoleDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateRoleDto>) -> impl IntoResponse {
    let result = sys_role::update(id, data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("更新角色失败")
        }
    }
}


/// 删除角色函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force= handler_force(params);
    let result = sys_role::delete_by_id(id, force).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("删除角色失败")
        }
    }
}

/// 查询角色详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_role::get_by_id(id).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询角色失败")
        }
    }
}

/// 分页查询角色函数
/// @param data 类型SearchRoleDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchRoleDto>) -> impl IntoResponse {
    let result = sys_role::search(data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询角色失败")
        }
    }
}
use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use response_utils::res::ResResult;
use common::FORCE;
use models::dto::sys::request_sys_user::{AddUserDto, SearchUserDto, UpdateUserDto, UpdateUserStatusDto};
use crate::service::sys::sys_user;

/// 添加用户函数
/// @param data 类型AddUserDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddUserDto>) -> impl IntoResponse {
    let result = sys_user::add(data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("添加用户失败")
        }
    }
}

/// 更新用户函数
/// @param id 类型String
/// @param data 类型UpdateUserDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateUserDto>) -> impl IntoResponse {
    let result = sys_user::update(id, data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("更新用户失败")
        }
    }
}


/// 删除用户函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force = match params.get(FORCE) {
        None => {
            false
        }
        Some(data) => {
            if data == "true" {
                true
            } else {
                false
            }
        }
    };
    let result = sys_user::delete_by_id(id, force).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("删除用户失败")
        }
    }
}

/// 查询用户详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_user::get_by_id(id).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询用户失败")
        }
    }
}

/// 更新用户状态函数
/// @param id 类型String
/// @param data 类型UpdateUserStatusDto
/// return IntoResponse
pub async fn update_status(Path(id): Path<String>, Json(data): Json<UpdateUserStatusDto>) -> impl IntoResponse {
    let result = sys_user::update_status(id, data.status).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询用户失败")
        }
    }
}

/// 分页查询用户函数
/// @param data 类型SearchUserDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchUserDto>) -> impl IntoResponse {
    let result = sys_user::search(data).await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("查询用户失败")
        }
    }
}
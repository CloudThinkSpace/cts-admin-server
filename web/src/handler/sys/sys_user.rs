use crate::handler::{handle_force, handle_result};
use crate::service::sys::sys_user;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use models::dto::sys::request::sys_user::{
    AddUserDto, SearchUserDto, UpdateUserDto, UpdateUserStatusDto,
};
use models::dto::sys::response::sys_user::ResponseUser;
use std::collections::HashMap;

/// 添加用户函数
/// @param data 类型AddUserDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddUserDto>) -> impl IntoResponse {
    let result = sys_user::add(data).await;
    handle_result(result)
}

/// 更新用户函数
/// @param id 类型String
/// @param data 类型UpdateUserDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateUserDto>) -> impl IntoResponse {
    let result = sys_user::update(id, data).await;
    handle_result(result)
}

/// 删除用户函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 判断是否真删除
    let force = handle_force(params);
    let result = sys_user::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询用户详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_user::get_by_id(id).await;
    handle_result(result)
}

/// 查询用户详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn get_user_info(Extension(user): Extension<ResponseUser>) -> impl IntoResponse {
    let id = user.id;
    let result = sys_user::get_by_id(id).await;
    handle_result(result)
}

/// 更新用户状态函数
/// @param id 类型String
/// @param data 类型UpdateUserStatusDto
/// return IntoResponse
pub async fn update_status(
    Path(id): Path<String>,
    Json(data): Json<UpdateUserStatusDto>,
) -> impl IntoResponse {
    let result = sys_user::update_status(id, data.status).await;
    handle_result(result)
}

/// 分页查询用户函数
/// @param data 类型SearchUserDto
/// return IntoResponse
pub async fn search(
    Extension(user): Extension<ResponseUser>,
    Json(data): Json<SearchUserDto>,
) -> impl IntoResponse {
    let result = sys_user::search(user, data).await;
    handle_result(result)
}

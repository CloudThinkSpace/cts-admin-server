use std::collections::HashMap;

use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;

use models::dto::sys::request::sys_menu::{AddMenuDto, SearchMenuDto, UpdateMenuDto};
use models::dto::sys::response::sys_user::ResponseUser;

use crate::handler::{handle_force, handle_result};
use crate::service::sys::sys_menu;

/// 添加菜单函数
/// @param data 类型AddMenuDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddMenuDto>) -> impl IntoResponse {
    let result = sys_menu::add(data).await;
    handle_result(result)
}

/// 更新菜单函数
/// @param id 类型String
/// @param data 类型UpdateMenuDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateMenuDto>) -> impl IntoResponse {
    let result = sys_menu::update(id, data).await;
    handle_result(result)
}


/// 删除菜单函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force = handle_force(params);
    let result = sys_menu::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询菜单详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = sys_menu::get_by_id(id).await;
    handle_result(result)
}

/// 分页查询菜单函数
/// @param data 类型SearchRoleDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchMenuDto>) -> impl IntoResponse {
    let result = sys_menu::search(data).await;
    handle_result(result)
}

/// 查询角色菜单函数
/// return IntoResponse
pub async fn role_menu_tree(Extension(current_user): Extension<ResponseUser>,) -> impl IntoResponse {
    // 取出role信息
    let role = current_user.role;
    let mut  role_id = String::from("1");
    if let Some(data) = role {
        role_id = data.id
    }
    let result = sys_menu::role_menu_tree(role_id).await;
    handle_result(result)
}
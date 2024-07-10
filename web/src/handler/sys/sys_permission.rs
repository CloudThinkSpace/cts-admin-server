use anyhow::Error;
use axum::{response::IntoResponse, Extension, Json};
use models::dto::sys::{request::sys_permission::{RoleApiDto, RoleMenuDto}, response::sys_user::ResponseUser};

use crate::{handler::handle_result, service::sys::sys_permission};

pub async fn set_menu(Json(data):Json<RoleMenuDto>) -> impl IntoResponse {
    let result = sys_permission::set_menu(data).await;
    handle_result(result)
}

pub async fn set_api(Json(data): Json<RoleApiDto>) -> impl IntoResponse {
    let result = sys_permission::set_api(data).await;
    handle_result(result)
}


pub async fn search_menu(Extension(user):Extension<ResponseUser>) -> impl IntoResponse {
    let role = user.role;
    let role_id = match role {
        Some(data) => {
            data.id
        }
        None => {
            return handle_result(Err(Error::msg("无效用户角色")))
        }
    };

    let result = sys_permission::search_menu(role_id).await;

    handle_result(result)
}

pub async fn search_api(Extension(user):Extension<ResponseUser>) -> impl IntoResponse {
    let role = user.role;
    let role_id = match role {
        Some(data) => {
            data.id
        }
        None => {
            return handle_result(Err(Error::msg("无效用户角色")))
        }
    };
    let result = sys_permission::search_api(role_id).await;
    handle_result(result)
}

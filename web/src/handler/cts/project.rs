use std::collections::HashMap;

use axum::extract::{Multipart, Path, Query};
use axum::response::IntoResponse;
use axum::Json;

use models::dto::cts::request::project::{AddProjectDto, SearchProjectDto, UpdateProjectDto};

use crate::handler::{handle_force, handle_result};
use crate::service::cts::project;

/// 添加项目函数
/// @param data 类型AddProjectDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddProjectDto>) -> impl IntoResponse {
    let result = project::add_project(data).await;
    handle_result(result)
}

/// 更新项目数
/// @param id 类型String
/// @param data 类型UpdateProjectDto
/// return IntoResponse
pub async fn update(
    Path(id): Path<String>,
    Json(data): Json<UpdateProjectDto>,
) -> impl IntoResponse {
    let result = project::update(id, data).await;
    handle_result(result)
}

/// 删除项目函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 判断是否真删除
    let force = handle_force(params);
    let result = project::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询项目详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = project::get_by_id(id).await;
    handle_result(result)
}

/// 分页查询项目函数
/// @param data 类型SearchProjectDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchProjectDto>) -> impl IntoResponse {
    let result = project::search(data).await;
    handle_result(result)
}

pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}


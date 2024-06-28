use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use models::dto::cts::request::form_template::{AddFormTemplateDto, SearchFormTemplateDto, UpdateFormTemplateDto};
use crate::handler::{handle_force, handle_result};
use crate::service::cts::form_template;

/// 添加表单函数
/// @param data 类型AddFormTemplateDto
/// return IntoResponse
pub async fn add(Json(data): Json<AddFormTemplateDto>) -> impl IntoResponse {
    let result = form_template::add(data).await;
    handle_result(result)
}

/// 更新表单函数
/// @param id 类型String
/// @param data 类型UpdateFormTemplateDto
/// return IntoResponse
pub async fn update(Path(id): Path<String>, Json(data): Json<UpdateFormTemplateDto>) -> impl IntoResponse {
    let result = form_template::update(id, data).await;
    handle_result(result)
}


/// 删除表单函数
/// @param id 类型String
/// @param params 类型HashMap<String, String>
/// return IntoResponse
pub async fn delete(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    // 判断是否真删除
    let force= handle_force(params);
    let result = form_template::delete_by_id(id, force).await;
    handle_result(result)
}

/// 查询表单详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn query(Path(id): Path<String>) -> impl IntoResponse {
    let result = form_template::get_by_id(id).await;
    handle_result(result)
}

/// 分页查询表单函数
/// @param data 类型SearchFormTemplateDto
/// return IntoResponse
pub async fn search(Json(data): Json<SearchFormTemplateDto>) -> impl IntoResponse {
    let result = form_template::search(data).await;
    handle_result(result)
}
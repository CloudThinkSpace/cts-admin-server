use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};

use crate::{handler::handle_result, service::sys::sys_table};

/// 查询表结构详情函数
/// @param id 类型String
/// return IntoResponse
pub async fn get_table_fields(
    Path(id): Path<String>,
    Query(param): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 参数处理
    let data = match param.get("type") {
        Some(r#type) => {
            if r#type == "task" {
                Some(false)
            } else if r#type == "data" {
                Some(true)
            } else {
                None
            }
        }
        None => None,
    };
    let result = sys_table::get_table_fields(id, data).await;
    handle_result(result)
}

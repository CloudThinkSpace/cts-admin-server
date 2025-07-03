use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;

use models::dto::cts::request::project_layer::AddProjectLayerDto;

use crate::handler::handle_result;
use crate::service::cts::project_layer;

/// 添加项目图层函数
/// @param data 类型AddProjectDto
/// return IntoResponse
pub async fn add(
    Json(data): Json<AddProjectLayerDto>,
) -> impl IntoResponse {
    let result = project_layer::add_project_layer(data).await;
    handle_result(result)
}
/// 删除项目图层，根据编号id
/// @param id 类型String
pub async fn delete(
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = project_layer::delete_by_id(id).await;
    handle_result(result)
}

/// 查询项目图层函数
/// @param data 类型SearchProjectDto
/// return IntoResponse
pub async fn layers(
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let result = project_layer::get_project_layers(project_id).await;
    handle_result(result)
}

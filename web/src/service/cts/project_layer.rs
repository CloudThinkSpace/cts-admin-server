use anyhow::{bail, Ok, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

use common::db::get_db;
use entity::project_layer::{ActiveModel, Column as ProjectLayerColumn, Entity as ProjectLayer};
use models::dto::cts::request::project_layer::AddProjectLayerDto;

/// 根据项目图层编号删除数据
/// @param id 项目编号
pub async fn delete_by_id(id: String) -> Result<String> {
    let db = get_db().await;
    // 查询项目图层表单信息
    let result = ProjectLayer::find_by_id(id.clone()).one(&db).await?;
    if result.is_some(){
        // 删除项目图层
        let delete_result = ProjectLayer::delete_by_id(id).exec(&db).await?;
        Ok(format!("{}", delete_result.rows_affected))
    } else {
        bail!("该项目图层不存在".to_string())
    }
}

/// 添加项目图层数据
pub async fn add_project_layer(data: AddProjectLayerDto) -> Result<String> {
    let db = get_db().await;
    // 判断Api名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }

    // type类型
    let layer_type = data.r#type.unwrap_or_else(|| "0".to_string());
    // format
    let layer_format = data.format.unwrap_or_else(|| "tile".to_string());

    // checked 默认是否选中
    let layer_checked = data.checked.unwrap_or(true);

    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        project_id: Set(data.project_id),
        name: Set(data.name),
        r#type: Set(layer_type),
        url: Set(data.url),
        format: Set(layer_format),
        checked: Set(layer_checked),
    };

    let add_project_layer = current.insert(&db).await?;
    Ok(add_project_layer.id)
}

/// 获取项目的图层列表
/// @param project_id
pub async fn get_project_layers(project_id: String) -> Result<Vec<serde_json::Value>> {
    let db = get_db().await;
    let project_layers: Vec<serde_json::Value> = ProjectLayer::find()
        .filter(ProjectLayerColumn::ProjectId.eq(project_id))
        .into_json()
        .all(&db)
        .await?;
    Ok(project_layers)
}

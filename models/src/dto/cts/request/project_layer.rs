use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

///
/// 添加项目图层
/// @param project_id 项目编号
/// @param name 图层名称
/// @param type 图层类型
/// @param url 图层url
/// @param format 图层格式
/// @param checked 默认是否选中
/// @param size 图片大小，默认265
#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddProjectLayerDto {
    pub project_id: String,
    pub name: String,
    pub r#type: Option<String>,
    pub url: String,
    pub format: Option<String>,
    pub checked: Option<bool>,
    pub size: Option<i32>,
}

impl Default for AddProjectLayerDto {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            r#type: None,
            url: "".to_string(),
            format: None,
            checked: Some(true),
            project_id: "".to_string(),
            size: Some(265),
        }
    }
}

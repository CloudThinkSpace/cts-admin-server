use crate::dto::Page;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFormData {
    pub page: Option<Page>,
    pub fields: Option<Vec<String>>,
    pub wheres: Option<Vec<String>>,
    // 排序
    pub orders: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFormData {
    // 提交数据对应的任务编号
    pub task_id: String,
    // 提交数据对应的表名编号
    pub name: String,
    // 表单数据
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFormData {
    // 提交数据对应的任务编号
    pub task_id: String,
    // 提交数据对应的表名编号
    pub name: String,
    // 表单数据
    pub data: Value,
}

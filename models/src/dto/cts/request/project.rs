use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::{Order, Page};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub r#type: Option<i32>,
    pub status: Option<i32>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddProjectDto {
    pub name: String,
    pub code: String,
    pub r#type: i32,
    pub status: i32,
    pub form_template_id: String,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub task_code: String,
    pub task_lon: String,
    pub task_lat: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProjectDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub r#type: Option<i32>,
    pub status: Option<i32>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}


impl Default for AddProjectDto {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            code: "".to_string(),
            r#type: 0,
            status: 0,
            form_template_id: "".to_string(),
            description: None,
            remark: None,
            task_code: "".to_string(),
            task_lon: "".to_string(),
            task_lat: "".to_string(),
        }
    }
}
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::dto::{Order, Page};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiDto {
    pub name: Option<String>,
    pub api_path: Option<String>,
    pub api_group: Option<String>,
    pub api_method: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddApiDto {
    pub name: String,
    pub api_path: String,
    pub api_group: String,
    pub api_method: String,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchApiDto {
    pub name: Option<String>,
    pub api_path: Option<String>,
    pub api_group: Option<String>,
    pub api_method: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}

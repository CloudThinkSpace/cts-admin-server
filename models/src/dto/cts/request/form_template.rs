use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::{Order, Page};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFormTemplateDto {
    pub name: Option<String>,
    pub title: Option<String>,
    pub version: Option<String>,
    pub content: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddFormTemplateDto {
    pub name: String,
    pub title: String,
    pub version: String,
    pub content: String,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFormTemplateDto {
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}
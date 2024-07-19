use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::{Order, Page};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTenantDto {
    pub name: Option<String>,
    pub enabled: Option<i32>,
    pub r#type: Option<i32>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddTenantDto {
    pub name: String,
    pub enabled: i32,
    pub r#type: i32,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTenantDto {
    pub name: Option<String>,
    pub enabled: Option<i32>,
    pub r#type: Option<i32>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTenantStatusDto {
    pub status: i32,
}
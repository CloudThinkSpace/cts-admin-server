use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::{Order, Page};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleDto {
    pub name: Option<String>,
    pub tenant_id: Option<String>,
    pub remark: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleDto {
    pub name: String,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRoleDto {
    pub name: Option<String>,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}

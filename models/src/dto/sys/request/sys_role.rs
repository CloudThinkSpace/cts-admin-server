use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::Page;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleDto {
    pub name: Option<String>,
    pub tenant_id: Option<String>,
    pub remark: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct AddRoleDto {
    pub name: String,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRoleDto {
    pub name: Option<String>,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
}

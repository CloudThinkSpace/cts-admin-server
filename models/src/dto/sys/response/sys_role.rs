use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::sys_role::Model;
use crate::dto::sys::response::sys_tenant::ResponseTenant;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRole {
    pub id: String,
    pub name: String,
    pub enabled: i32,
    pub tenant: Option<ResponseTenant>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

impl From<Model> for ResponseRole {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            enabled: value.enabled,
            tenant: None,
            description: value.description,
            remark: value.remark,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
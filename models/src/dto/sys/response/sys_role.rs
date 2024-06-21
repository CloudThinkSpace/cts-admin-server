use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRole {
    pub id: String,
    pub name: String,
    pub enabled: i32,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}
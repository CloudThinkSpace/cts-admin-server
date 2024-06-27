use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::sys_api::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseApi {
    pub id: String,
    pub name: String,
    pub api_path: String,
    pub api_group: String,
    pub api_method: String,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}


impl From<Model> for ResponseApi {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            api_path: value.api_path,
            api_method: value.api_method,
            api_group: value.api_group,
            remark: value.remark,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
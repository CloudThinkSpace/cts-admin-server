use common::date_time_format;
use common::date_time_format_option;
use entity::sys_tenant::Model;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTenant {
    pub id: String,
    pub name: String,
    pub enabled: i32,
    pub r#type: i32,
    pub remark: Option<String>,
    pub description: Option<String>,
    #[serde(with = "date_time_format")]
    pub created_at: DateTime,
    #[serde(with = "date_time_format_option")]
    pub updated_at: Option<DateTime>,
}

impl From<Model> for ResponseTenant {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            enabled: value.enabled,
            r#type: value.r#type,
            remark: value.remark,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}


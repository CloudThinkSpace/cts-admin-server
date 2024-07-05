use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::project::Model;
use crate::dto::cts::response::form_template::ResponseFormTemplate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseProject {
    pub id: String,
    pub name: String,
    pub code: String,
    pub r#type: i32,
    pub status: i32,
    pub form_template: Option<ResponseFormTemplate>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}


impl From<Model> for ResponseProject {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            code: value.code,
            r#type: value.r#type,
            status: value.status,
            remark: value.remark,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
            form_template: None,
        }
    }
}
use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::form_template::Model;

#[derive(Debug, Serialize, Deserialize, Clone, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ResponseFormTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub version: String,
    pub content: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}


impl From<Model> for ResponseFormTemplate {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            title: value.title,
            version: "".to_string(),
            remark: value.remark,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
            content: value.content,
        }
    }
}
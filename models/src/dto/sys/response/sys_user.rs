use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::sys_user::Model;

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub phone: String,
    pub email: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub tenant_id: Option<String>,
}

impl From<Model> for ResponseUser {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            nickname: value.nickname,
            phone: value.phone,
            email: value.email,
            status: value.status,
            remark: value.remark,
            description: value.description,
            avatar: value.avatar,
            created_at: value.created_at,
            updated_at: value.updated_at,
            tenant_id: value.tenant_id,
        }
    }
}
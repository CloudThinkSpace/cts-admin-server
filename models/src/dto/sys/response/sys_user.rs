use crate::dto::sys::response::sys_role::ResponseRole;
use crate::dto::sys::response::sys_tenant::ResponseTenant;
use entity::sys_user::Model;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use common::date_time_format;
use common::date_time_format_option;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(with = "date_time_format")]
    pub created_at: DateTime,
    #[serde(with = "date_time_format_option")]
    pub updated_at: Option<DateTime>,
    pub tenant: Option<ResponseTenant>,
    pub role: Option<ResponseRole>,
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
            role: None,
            tenant: None,
        }
    }
}


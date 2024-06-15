use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::dto::Page;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserStatusDto {
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct AddUserDto {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub phone: String,
    pub email: Option<String>,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchUserDto {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub tenant_id: Option<String>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
}

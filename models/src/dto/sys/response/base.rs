use serde::Serialize;
use crate::dto::sys::response::sys_user::ResponseUser;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseToken {
    pub user_info: ResponseUser,
    pub token: Token,

}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
    pub expire: u64,
}

impl Token {
    pub fn new(access_token: String, expire: u64) -> Self {
        Self {
            access_token,
            expire,
        }
    }
}

impl ResponseToken {
    pub fn new(user: ResponseUser, token: Token) -> Self {
        Self {
            user_info: user,
            token,
        }
    }
}
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use common::auth::jwt::{decode_token, encode_token};
use common::db::get_db;
use common::md5::check_password;
use entity::sys_user::{Column as SysUserColumn, Entity as SysUser};
use models::dto::sys::response::base::{ResponseToken, Token};
use models::dto::sys::response::sys_user::ResponseUser;
use anyhow::{bail, Result};

pub async fn login(username: String, password: String) -> Result<ResponseToken> {
    let db = get_db().await;
    // 查询用户
    let user = SysUser::find()
        .filter(SysUserColumn::Username.eq(username))
        .one(&db).await?;
    if let Some(data) = user {
        // 获取用户密码
        let db_password = data.password.clone();
        // 验证密码
        let is_ok = check_password(db_password, password);
        match is_ok {
            true => {
                // 生成token对象
                let user: ResponseUser = data.into();
                let  exp = (Utc::now() + Duration::seconds(3600)).timestamp() as u64;
                let token = encode_token(user.clone(), exp)?;

                let token = ResponseToken::new(user,Token::new(token, exp));

                Ok(token)
            }
            false => {
                bail!("用户或密码错误".to_string())
            }
        }
    } else {
        bail!("用户名不存在".to_string())
    }
}

pub async fn logout(headers: HeaderMap) -> Result<()> {
    // 获取token
    if headers.contains_key(AUTHORIZATION) {
        let token = &headers[AUTHORIZATION];
        // 判断是否为空
        if token.is_empty() {
            bail!("token不能为空".to_string())
        }else {
            let token_str = token.to_str()?;
            // 验证token是否有效
            let result = decode_token::<ResponseUser>(token_str)?;
            // todo 从redis中删除token
            println!("{:?}", result);
            Ok(())
        }

    }else {
        bail!("退出失败，没有token信息".to_string())
    }
}
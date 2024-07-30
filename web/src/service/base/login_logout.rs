use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use anyhow::{bail, Result};
use common::auth::jwt::{decode_token, encode_token};
use common::db::get_db;
use common::md5::check_password;
use entity::sys_role::Entity as SysRole;
use entity::sys_tenant::Entity as SysTenant;
use entity::sys_user::{Column as SysUserColumn, Entity as SysUser};
use models::dto::sys::response::base::{ResponseToken, Token};
use models::dto::sys::response::sys_user::ResponseUser;

pub async fn login(username: String, password: String) -> Result<ResponseToken> {
    let db = get_db().await;
    // 查询用户
    let user = SysUser::find()
        .filter(SysUserColumn::Username.eq(username))
        .one(&db)
        .await?;
    if let Some(data) = user {
        // 判断用户是否停用
        if data.status == 1 {
            bail!("用户已停用，无法登录")
        }
        // 获取用户密码
        let db_password = data.password.clone();
        // 验证密码
        let is_ok = check_password(db_password, password);
        match is_ok {
            true => {
                // 角色编号
                let role_id = data.role_id.clone();
                // 租户对象
                let tenant_id = data.tenant_id.clone();
                // 查询角色信息
                let role = SysRole::find_by_id(role_id).one(&db).await?;
                // 转换用户对象，data数据转移
                let mut user: ResponseUser = data.into();
                // 判断角色是否存在
                if let Some(role) = role {
                    let current_role = role.into();
                    user.role = Some(current_role);
                }
                // 查询租户信息
                if let Some(id) = tenant_id {
                    let tenant_pt = SysTenant::find_by_id(id).one(&db).await?;
                    if let Some(tenant) = tenant_pt {
                        let current_tenant = tenant.into();
                        user.tenant = Some(current_tenant);
                    }
                }
                // 设置token过期时间1000小时
                let exp = (Utc::now() + Duration::seconds(3600 * 1000)).timestamp() as u64;
                // 加密
                let token = encode_token(user.clone(), exp)?;
                let response_token = ResponseToken::new(user, Token::new(token, exp));
                Ok(response_token)
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
        } else {
            let token_str = token.to_str()?;
            // 验证token是否有效
            let result = decode_token::<ResponseUser>(token_str)?;
            // todo 从redis中删除token
            println!("{:?}", result);
            Ok(())
        }
    } else {
        bail!("退出失败，没有token信息".to_string())
    }
}

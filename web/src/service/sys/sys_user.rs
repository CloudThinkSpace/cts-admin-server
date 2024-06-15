use sea_orm::{EntityTrait};
use tracing::info;
use common::db::get_db;
use entity::sys_user::Entity as SysUser;
use models::sys::sys_user::User;
use crate::dao::sys::ADMIN_ID;
use crate::error::Error;

pub async fn get_by_id(id: String) -> Result<Option<User>, Error> {
    let db = get_db().await;
    let result = SysUser::find_by_id(id)
        .into_model::<User>()
        .one(&db).await;
    match result {
        Ok(data) => {
            Ok(data)
        }
        Err(err) => {
            info!("{}",err.to_string());
            Err(err.into())
        }
    }
}

pub async fn delete_by_id(id: String) -> Result<Option<User>, Error> {
    let db = get_db().await;
    let result = SysUser::find_by_id(id.clone())
        .into_model::<User>()
        .one(&db).await;
    let user = match result {
        Ok(data) => {
            data
        }
        Err(err) => {
            info!("{}", err.to_string());
            return Err(err.into());
        }
    };
    if let Some(data) = &user {
        // 查看用户是否为超级管理员
        if ADMIN_ID == data.id {
            info!("超级管理员无法删除");
            return Err(Error::DatabaseError("该用户无法删除".to_string()));
        } else {
            let result = SysUser::delete_by_id(id)
                .exec(&db).await;
            match result {
                Ok(_data) => {
                    Ok(user)
                }
                Err(err) => {
                    info!("{}", err.to_string());
                    Err(err.into())
                }
            }
        }
    } else {
        info!("该用户不存在");
        Err(Error::DatabaseError("该用户不存在".to_string()))
    }
}
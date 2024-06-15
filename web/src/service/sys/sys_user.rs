use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use tracing::info;
use uuid::Uuid;
use common::db::get_db;
use common::error::CtsError;
use common::md5::generate_md5;
use entity::sys_user::{ActiveModel, Entity as SysUser, Column as SysUserColumn};
use entity::sys_tenant::{Entity as SysTenant};
use models::dto::{PageResult, parse_page};
use models::dto::sys::request_sys_user::{AddUserDto, SearchUserDto, UpdateUserDto};
use models::dto::sys::sys_user::User;
use crate::service::sys::ADMIN_ID;

/// 根据用户编号查询用户数据
/// @param id 用户编号
pub async fn get_by_id(id: String) -> Result<Option<User>, CtsError> {
    let db = get_db().await;
    let result = SysUser::find_by_id(id)
        // 保障删除字段为空
        .filter(SysUserColumn::DeletedAt.is_null())
        .into_model::<User>()
        .one(&db).await;
    match result {
        Ok(data) => {
            Ok(data)
        }
        Err(err) => {
            CtsError::Sql(err.to_string()).into()
        }
    }
}

/// 根据用户编号删除用户数据
/// @param id 用户编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String, CtsError> {
    let db = get_db().await;
    // 查询用户信息
    let result = SysUser::find_by_id(id.clone())
        .one(&db).await;
    let result = match result {
        Ok(data) => {
            data
        }
        Err(_) => {
            return CtsError::Sql("用户不存在".to_string()).into();
        }
    };
    if let Some(data) = result {
        // 查看用户是否为超级管理员
        if ADMIN_ID == data.id {
            info!("超级管理员无法删除");
            return CtsError::Custom("超级管理员无法删除".to_string()).into();
        } else {
            // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
            match force {
                true => {
                    // 删除用户
                    let delete_result = SysUser::delete_by_id(id)
                        .exec(&db).await;
                    match delete_result {
                        Ok(_) => {
                            Ok("删除成功".to_string())
                        }
                        Err(err) => {
                            CtsError::Sql(err.to_string()).into()
                        }
                    }
                }
                false => {
                    // 更新删除字段
                    let mut current_user: ActiveModel = data.into();
                    current_user.deleted_at = Set(Some(Local::now().naive_local()));
                    // 更新删除字段数据
                    let update_result = current_user.update(&db).await;
                    match update_result {
                        Ok(_) => {
                            Ok("删除用户成功".to_string())
                        }
                        Err(err) => {
                            CtsError::Sql(err.to_string()).into()
                        }
                    }
                }
            }
        }
    } else {
        info!("该用户不存在");
        CtsError::DataNotExists("该用户不存在".to_string()).into()
    }
}

/// 添加用户
/// @param 用户对象
pub async fn add(data: AddUserDto) -> Result<String, CtsError> {
    let db = get_db().await;

    // 判断用户名是否为空
    if data.username.is_empty() {
        return CtsError::Request("用户名不能为空".to_string()).into();
    }
    // 用户名查询数据
    let user = SysUser::find()
        .filter(SysUserColumn::Username.eq(data.username.clone()))
        .into_model::<User>()
        .one(&db).await;
    // 判断租户是否存在
    match user {
        Ok(data) => {
            if data.is_some() {
                return CtsError::DataAlreadyExists("用户名不能重复，该用户已经存在".to_string()).into();
            }
        }
        Err(err) => {
            return CtsError::Sql(err.to_string()).into();
        }
    };
    // 判断租户是否存在
    match &data.tenant_id {
        None => {
            return CtsError::Request("租户编号不能为空".to_string()).into();
        }
        Some(data) => {
            let tenant = SysTenant::find_by_id(data)
                .one(&db)
                .await;
            match tenant {
                Ok(data) => {
                    if data.is_none() {
                        return CtsError::Sql("租户编号不存在".to_string()).into();
                    }
                }
                Err(err) => {
                    return CtsError::Sql(err.to_string()).into();
                }
            }
        }
    }

    let current_user: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(data.username),
        nickname: Set(data.nickname),
        password: Set(generate_md5(data.password)),
        phone: Set(data.phone),
        tenant_id: Set(data.tenant_id),
        email: Set(data.email),
        remark: Set(data.remark),
        avatar: Set(data.avatar),
        description: Set(data.description),
        status: Set(0),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
    };
    current_user.insert(&db)
        .await
        .map_err(|err| {
            info!("{}", err.to_string());
            CtsError::Sql(err.to_string())
        }).map(|data| {
        data.id
    })
}


/// 更新用户信息
/// @param update_user 待更新的用户对象
pub async fn update(id: String, update_user: UpdateUserDto) -> Result<String, CtsError> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        info!("用户编号不能为空");
        return Err(CtsError::DataNotExists("用户编号不能为空".to_string()));
    }
    if ADMIN_ID == id {
        info!("超级用户无法更新");
        return Err(CtsError::Custom("超级用户无法更新".to_string()));
    }
    // 查询用户信息
    let result = SysUser::find_by_id(id.clone())
        .one(&db).await?;
    // 更新用户名
    if let Some(current_user) = result {
        let mut current_user: ActiveModel = current_user.into();
        // 昵称是否为空
        if update_user.nickname.is_some() {
            current_user.nickname = Set(update_user.nickname.unwrap());
        }
        // 更新邮箱
        if update_user.email.is_some() {
            current_user.email = Set(Some(update_user.email.unwrap()));
        }
        // 更新电话
        if update_user.phone.is_some() {
            current_user.phone = Set(update_user.phone.unwrap())
        }
        if update_user.avatar.is_some() {
            current_user.avatar = Set(Some(update_user.avatar.unwrap()))
        }
        // 更新备注
        if update_user.remark.is_some() {
            current_user.remark = Set(Some(update_user.remark.unwrap()))
        }
        // 更新描述
        if update_user.description.is_some() {
            current_user.description = Set(Some(update_user.description.unwrap()))
        }

        // 更新时间
        current_user.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current_user.update(&db).await?;
        Ok(update_data.id)
    } else {
        info!("用户数据不存在，无法更新");
        Err(CtsError::Custom("用户数据不存在，无法更新".to_string()))
    }
}

/// 更新用户状态
/// @param id 用户编号
/// @param status 用户状态，值为【0或1】，其他值无效
pub async fn update_status(id: String, status: i32) -> Result<String, CtsError> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        info!("用户编号不能为空");
        return Err(CtsError::DataNotExists("用户编号不能为空".to_string()));
    }
    // 判断是否为超级管理员
    if ADMIN_ID == id {
        info!("超级用户无法变更状态");
        return Err(CtsError::Custom("超级用户无法变更状态".to_string()));
    }
    // 查询用户信息
    let result = SysUser::find_by_id(id)
        .one(&db).await?;
    // 更新用户名
    if let Some(current_user) = result {
        let mut current_user: ActiveModel = current_user.into();
        match status {
            0 | 1 => {
                current_user.status = Set(status)
            }
            _ => {
                info!("用户状态值不合法，只能是【0或1】");
                return Err(CtsError::Custom("用户状态值不合法，只能是【0或1】".to_string()));
            }
        };
        // 更新时间
        current_user.updated_at = Set(Some(Local::now().naive_local()));
        // 更新状态
        let ok = current_user.update(&db).await?;
        Ok(ok.id)
    } else {
        info!("用户数据不存在，无法更新");
        Err(CtsError::DataNotExists("用户数据不存在，无法更新".to_string()))
    }
}

/// 查询用户列表
/// @param data 类型SearchUserDto
pub async fn search(data: SearchUserDto) -> Result<PageResult<User>, CtsError> {
    let db = get_db().await;
    let mut select = SysUser::find();
    // 判断用户名是否为空
    if data.username.is_some() {
        select = select.filter(SysUserColumn::Username.contains(data.username.unwrap()));
    }
    // 判断昵称是否为空
    if data.nickname.is_some() {
        select = select.filter(SysUserColumn::Nickname.contains(data.nickname.unwrap()));
    }
    // 排除已删除用户
    select = select.filter(SysUserColumn::DeletedAt.is_null());
    // 获取事务对象
    let tx = db.begin().await?;
    // 查询数据数量
    let total = select.clone().count(&tx).await?;
    let (page_no, page_size) = parse_page(data.page);
    // 分页总数
    let paginate = select
        .into_model::<User>()
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询用户数据
    let user_list = paginate.fetch_page(page_no - 1).await?;
    let result = PageResult::new(user_list, total, pages, page_no);
    Ok(result)
}
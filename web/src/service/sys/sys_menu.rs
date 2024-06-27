use anyhow::{bail, Result};
use async_recursion::async_recursion;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Expr, IntoCondition};
use uuid::Uuid;

use common::db::get_db;
use common::error::CtsError;
use entity::sys_menu::{ActiveModel, Column as SysMenuColumn, Entity as SysMenu};
use entity::sys_role_menu::{Column as SysRoleMenu, Relation as SysRoleMenuRelation};
use models::dto::{handler_page, PageResult};
use models::dto::sys::request::sys_menu::{AddMenuDto, SearchMenuDto, UpdateMenuDto};
use models::dto::sys::response::sys_menu::ResponseMenu;

use crate::service::sys::{SYSTEM_MENU, SYSTEM_PARENT_MENU_ID};

/// 根据菜单编号查询数据
/// @param id 菜单编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseMenu>> {
    let db = get_db().await;
    let result = SysMenu::find_by_id(id)
        // 保障删除字段为空
        .filter(SysMenuColumn::DeletedAt.is_null())
        .one(&db).await?;
    match result {
        None => {
            bail!("菜单不存在")
        }
        Some(data) => {
            Ok(Some(data.into()))
        }
    }
}

/// 根据菜单编号删除数据
/// @param id 菜单编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询菜单信息
    let result = SysMenu::find_by_id(id.clone())
        .one(&db).await?;
    if let Some(data) = result {
        // 查看菜单是否为系统菜单
        if SYSTEM_MENU == data.default_menu {
            bail!("系统菜单无法删除".to_string())
        } else {
            // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
            match force {
                true => {
                    // 删除菜单
                    let delete_result = SysMenu::delete_by_id(id)
                        .exec(&db).await?;
                    Ok(format!("{}", delete_result.rows_affected))
                }
                false => {
                    // 更新删除字段
                    let mut current: ActiveModel = data.into();
                    current.deleted_at = Set(Some(Local::now().naive_local()));
                    // 更新删除字段数据
                    let update_result = current.update(&db).await?;
                    Ok(update_result.id)
                }
            }
        }
    } else {
        bail!("该菜单不存在".to_string())
    }
}

/// 添加菜单
/// @param 菜单对象
pub async fn add(data: AddMenuDto) -> Result<String> {
    let db = get_db().await;
    // 判断菜单名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }
    // 组织数据
    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        parent_id: Set(data.parent_id),
        sort: Set(data.sort),
        path: Set(data.path),
        hidden: Set(data.hidden),
        component: Set(data.component),
        active_name: Set(data.active_name),
        keep_alive: Set(data.keep_alive),
        title: Set(data.title),
        icon: Set(data.icon),
        // 添加菜单为普通菜单0，系统菜单为1
        default_menu: Default::default(),
        menu_level: Set(data.menu_level),
        remark: Set(data.remark),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
        close_tab: Default::default(),
    };
    // 查询sort位置以及以后的数据
    let list = SysMenu::find()
        .filter(SysMenuColumn::Sort.gte(data.sort))
        .all(&db).await?;
    // 获取事务对象
    let tx = db.begin().await?;
    // sort +1
    for item in list.into_iter() {
        let mut data: ActiveModel = item.into();
        data.sort = Set(data.sort.unwrap() + 1);
        data.update(&tx).await.map_err(|err| {
            CtsError::Custom(err.to_string())
        })?;
    }
    // 插入新菜单
    let add_data = current.insert(&tx).await?;
    // 提交数据
    tx.commit().await?;

    Ok(add_data.id)
}


/// 更新菜单信息
/// @param update_role 待更新的菜单对象
pub async fn update(id: String, update_menu: UpdateMenuDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("菜单编号不能为空".to_string())
    }
    // 查询菜单信息
    let result = SysMenu::find_by_id(id.clone()).one(&db).await?;
    // 更新菜单信息
    if let Some(current) = result {
        if SYSTEM_MENU == current.default_menu {
            bail!("默认菜单不能修改".to_string())
        }
        // 转成成可编辑的对象
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_menu.name.is_some() {
            current.name = Set(update_menu.name.unwrap());
        }
        // 更新 父节点
        if update_menu.parent_id.is_some() {
            current.parent_id = Set(update_menu.parent_id.unwrap());
        }
        // 原排序
        let mut sort: i64 = 0;
        // 排序
        if update_menu.sort.is_some() {
            sort = current.sort.unwrap();
            current.sort = Set(update_menu.sort.unwrap());
        }
        // 路径
        if update_menu.path.is_some() {
            current.path = Set(update_menu.path.unwrap());
        }
        // 是否隐藏
        if update_menu.hidden.is_some() {
            current.hidden = Set(update_menu.hidden.unwrap());
        }
        // 组件更新
        if update_menu.component.is_some() {
            current.component = Set(update_menu.component.unwrap());
        }
        if update_menu.active_name.is_some() {
            current.active_name = Set(update_menu.active_name);
        }
        if update_menu.keep_alive.is_some() {
            current.keep_alive = Set(update_menu.keep_alive.unwrap());
        }
        // 标题
        if update_menu.title.is_some() {
            current.title = Set(update_menu.title.unwrap());
        }
        // 菜单图标
        if update_menu.icon.is_some() {
            current.icon = Set(update_menu.icon);
        }
        // 关闭
        if update_menu.close_tab.is_some() {
            current.close_tab = Set(update_menu.close_tab.unwrap());
        }
        // 更新备注
        if update_menu.remark.is_some() {
            current.remark = Set(Some(update_menu.remark.unwrap()));
        }
        // 更新描述
        if update_menu.description.is_some() {
            current.description = Set(Some(update_menu.description.unwrap()));
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 获取事务对象
        let tx = db.begin().await?;
        if update_menu.sort.is_some() {
            let new_sort = update_menu.sort.unwrap();
            // 查询sort位置以及以后的数据
            let mut select = SysMenu::find();
            if sort > new_sort {
                select = select.filter(SysMenuColumn::Sort.gte(new_sort))
                    .filter(SysMenuColumn::Sort.lte(sort));
            } else {
                select = select.filter(SysMenuColumn::Sort.gte(sort))
                    .filter(SysMenuColumn::Sort.lte(new_sort));
            }

            let list = select.all(&db).await?;

            // sort +1
            for item in list.into_iter() {
                let mut data: ActiveModel = item.into();
                data.sort = Set(data.sort.unwrap() + 1);
                // 更新sort
                data.update(&tx).await?;
            }
        }
        // 更新数据
        let update_data = current.update(&tx).await?;
        tx.commit().await?;
        Ok(update_data.id)
    } else {
        bail!("菜单数据不存在，无法更新".to_string())
    }
}

/// 查询角色列表
/// @param data 类型SearchRoleDto
pub async fn search(data: SearchMenuDto) -> Result<PageResult<ResponseMenu>> {
    let db = get_db().await;
    let mut select = SysMenu::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(SysMenuColumn::Name.contains(data.name.unwrap()));
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(SysMenuColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(SysMenuColumn::Description.contains(data.description.unwrap()));
    }
    // path
    if data.path.is_some() {
        select = select.filter(SysMenuColumn::Path.contains(data.path.unwrap()));
    }
    // hidden
    if data.hidden.is_some() {
        select = select.filter(SysMenuColumn::Hidden.eq(data.hidden.unwrap()));
    }
    // component
    if data.component.is_some() {
        select = select.filter(SysMenuColumn::Component.contains(data.component.unwrap()));
    }
    // active_name
    if data.active_name.is_some() {
        select = select.filter(SysMenuColumn::ActiveName.contains(data.active_name.unwrap()));
    }
    // keep_alive
    if data.keep_alive.is_some() {
        select = select.filter(SysMenuColumn::KeepAlive.eq(data.keep_alive.unwrap()));
    }
    // title
    if data.title.is_some() {
        select = select.filter(SysMenuColumn::Title.contains(data.title.unwrap()));
    }
    // default menu
    if data.default_menu.is_some() {
        select = select.filter(SysMenuColumn::DefaultMenu.eq(data.default_menu.unwrap()));
    }
    // close tab
    if data.close_tab.is_some() {
        select = select.filter(SysMenuColumn::CloseTab.eq(data.close_tab.unwrap()));
    }
    // 排除已删除角色
    select = select.filter(SysMenuColumn::DeletedAt.is_null());
    select = select.order_by_asc(SysMenuColumn::Sort);
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询角色数据
    let list = paginate.fetch_page(page_no - 1).await?.into_iter().map(|data| data.into()).collect();

    let result = PageResult::new(list, total, pages, page_no);

    Ok(result)
}

pub async fn role_menu_tree(role_id: String) -> Result<Vec<ResponseMenu>> {
    let db = get_db().await;
    let result = SysMenu::find()
        .join(JoinType::LeftJoin,
              SysRoleMenuRelation::SysMenu
                  .def().rev()
                  .on_condition(move |_left, right| {
                      Expr::col((right, SysRoleMenu::RoleId))
                          .eq(role_id.clone())
                          .into_condition()
                  }),
        )
        .order_by_asc(SysMenuColumn::Sort)
        .all(&db).await?
        .into_iter()
        .map(|model| model.into())
        .collect();

    let root_nodes = root_tree(&result).await;
    let data = child_tree(root_nodes, &result).await;
    Ok(data)
}

async fn root_tree(nodes: &Vec<ResponseMenu>) -> Vec<ResponseMenu> {
    nodes
        .iter()
        .filter(|item| item.parent_id == SYSTEM_PARENT_MENU_ID)
        .map(|item| item.clone())
        .collect()
}

#[async_recursion]
async fn child_tree(mut roots: Vec<ResponseMenu>, nodes: &Vec<ResponseMenu>) -> Vec<ResponseMenu> {
    for root in roots.iter_mut() {
        let data: Vec<ResponseMenu> = nodes
            .iter()
            .filter(|item| item.parent_id == root.id)
            .map(|item| item.clone())
            .collect();
        let data = child_tree(data.clone(), nodes).await;
        root.children = Some(data);
    }
    return roots;
}


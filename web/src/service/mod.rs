use sea_orm::{DatabaseConnection, EntityTrait};
use entity::prelude::SysTenant;
use anyhow::{bail, Result};
use tracing::info;

pub mod sys;
pub mod base;
pub mod cts;


/// 判断租户是否存在
pub async fn has_tenant(tenant_id: &Option<String>, db: &DatabaseConnection) -> Result<()> {
    // 判断租户是否存在
    match &tenant_id {
        None => {
            info!("租户编号不能为空");
            bail!("租户编号不能为空".to_string())
        }
        Some(data) => {
            let tenant = SysTenant::find_by_id(data)
                .one(db)
                .await?;

            match tenant {
                None => {
                    info!("租户不存在");
                    bail!("租户不存在".to_string())
                }
                Some(_) => {
                    Ok(())
                }
            }
        }
    }
}

// 处理排序
// pub async fn handle_orders<T>(mut select: Select<T>, orders: Option<Vec<Order>>) -> Select<T>
// where
//     T: EntityTrait
// {
//     if orders.is_some() {
//         for order in orders.unwrap().into_iter() {
//             if order.sort.is_some() {
//                 let sort = order.sort.unwrap();
//                 let name = order.name;
//                 if sort == "asc" {
//                     select = select.order_by_asc(order.name)
//                 }
//
//             }
//
//         }
//     }
//     select
// }

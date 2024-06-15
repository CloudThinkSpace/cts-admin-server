use sea_orm::{DatabaseConnection, EntityTrait};
use common::error::CtsError;
use entity::prelude::SysTenant;

pub mod sys;


/// 判断租户是否存在
pub async fn has_tenant(tenant_id: &Option<String>, db: &DatabaseConnection) -> Result<(), CtsError> {
    // 判断租户是否存在
    match &tenant_id {
        None => {
            return CtsError::Request("租户编号不能为空".to_string()).into();
        }
        Some(data) => {
            let tenant = SysTenant::find_by_id(data)
                .one(db)
                .await?;

            match tenant {
                None => {
                    return CtsError::Sql("租户编号不存在".to_string()).into();
                }
                Some(_) => {
                    Ok(())
                }
            }
        }
    }
}

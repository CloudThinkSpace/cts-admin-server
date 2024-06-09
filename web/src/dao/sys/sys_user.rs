use sea_orm::{DbErr, EntityTrait};
use common::db::get_db;
use entity::sys_user::Entity as SysUser;
use models::sys::sys_user::User;

pub async fn get_by_id(id:String) -> Result<Option<User>,DbErr>{
    let db = get_db().await;
    SysUser::find_by_id(id)
        .into_model::<User>()
        .one(&db).await

}
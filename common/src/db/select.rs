use sea_orm::{DatabaseBackend, FromQueryResult, JsonValue, SelectModel, SelectorRaw, Statement};
use serde_json::Value;


pub struct CtsSelect(String);

impl CtsSelect {
    pub fn table(table_name: &str) -> Self {
        Self(table_name.to_string())
    }

    /// 根据编号进行查询
    /// @param id 数据编号
    /// @param query_delete 是否查询被删除的数据
    /// return SelectorRaw
    pub fn find_by_id(&self, id: &str, query_delete: bool) -> SelectorRaw<SelectModel<Value>> {
        let sql = match query_delete {
            true => {
                format!("select * from {} where id='{}'", self.0, id)
            }
            false => {
                format!("select * from {} where deleted_at is null and id='{}'", self.0, id)
            }
        };
        let select = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [],
        ));

        select
    }

    /// 查询数据
    pub fn find(&self) -> SelectorRaw<SelectModel<Value>> {
        let sql = format!("select * from {} where deleted_at is null order by updated_at, created_at", self.0);
        let select = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [],
        ));
        select
    }
}



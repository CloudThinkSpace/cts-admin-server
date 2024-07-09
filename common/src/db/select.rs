use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseConnection, ExecResult, FromQueryResult, JsonValue,
    SelectModel, SelectorRaw, Statement,
};
use serde_json::Value;

use crate::db::db_type::DbType;
use crate::db::form::{parse_value_to_insert_sql, parse_value_to_update_sql, FormCommonField};

pub struct CtsSelect(String, Option<String>);

impl CtsSelect {
    pub fn table(table_name: &str) -> Self {
        Self(table_name.to_string(), None)
    }

    /// 根据编号进行查询
    /// @param id 数据编号
    /// @param query_delete 是否查询被删除的数据
    /// return SelectorRaw
    pub fn select_by_id(
        &mut self,
        id: &str,
        query_delete: bool,
    ) -> SelectorRaw<SelectModel<Value>> {
        let sql = match query_delete {
            true => {
                format!("select * from {} where id='{}'", self.0, id)
            }
            false => {
                format!(
                    "select * from {} where deleted_at is null and id='{}'",
                    self.0, id
                )
            }
        };
        JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [],
        ))
    }

    /// 查询数据
    pub fn select(&self) -> SelectorRaw<SelectModel<Value>> {
        let sql = format!("select * from {} where deleted_at is null order by updated_at desc nulls last, created_at desc", self.0);
        JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [],
        ))
    }

    /// 根据编号进行查询
    /// @param id 数据编号
    /// @param query_delete 是否查询被删除的数据
    /// return Self
    pub fn find_by_id(&mut self, id: &str, query_delete: bool) -> Self {
        self.1 = match query_delete {
            true => Some(format!("select * from {} where id='{}'", self.0, id)),
            false => Some(format!(
                "select * from {} where deleted_at is null and id='{}'",
                self.0, id
            )),
        };
        Self(self.0.clone(), self.1.clone())
    }

    /// 查询数据
    pub fn find(&mut self) -> Self {
        self.1 = Some(format!("select * from {} where deleted_at is null order by updated_at desc nulls last, created_at desc", self.0));
        Self(self.0.clone(), self.1.clone())
    }

    /// 删除数据根据数据编号，
    /// @param id 数据编号
    /// @param force 是否彻底删除
    pub fn delete_by_id(&mut self, id: &str, force: bool) -> Self {
        self.1 = match force {
            true => Some(format!("DELETE FROM {} WHERE id= '{}'", self.0, id)),
            false => {
                let date = Local::now().naive_local();
                Some(format!(
                    "UPDATE {} SET DELETED_AT={} WHERE id= '{}'",
                    self.0,
                    date.display(),
                    id
                ))
            }
        };
        Self(self.0.clone(), self.1.clone())
    }

    /// 更新数据
    /// @param id 数据编号
    /// @param data 数据
    pub fn update(&mut self, id: &str, data: Value) -> Result<Self> {
        let sql = parse_value_to_update_sql(self.0.clone(), id.to_string(), data, |_| Ok(()))?;
        self.1 = Some(sql);
        Ok(Self(self.0.clone(), self.1.clone()))
    }

    /// 添加数据
    /// @param data 数据
    /// @param hande_id 处理数据编号函数，如果不需要处理使用None
    pub fn add<F>(&mut self, data: Value, mut hande_id: F) -> Result<Self>
    where
        F: FnMut(&String),
    {
        let (id, sql) = parse_value_to_insert_sql(
            self.0.clone(),
            data,
            |_| Ok(()),
            |hearders, columns| {
                hearders.push(FormCommonField::Status.to_string());
                columns.push(Box::new(0));
            },
        )?;
        self.1 = Some(sql);
        hande_id(&id);
        Ok(Self(self.0.clone(), self.1.clone()))
    }

    pub async fn one(&self, db: &DatabaseConnection) -> Result<Option<Value>> {
        match &self.1 {
            None => {
                bail!("请先执行find_by_id")
            }
            Some(data) => {
                let select = JsonValue::find_by_statement(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    data,
                    [],
                ));
                let result = select.one(db).await?;
                Ok(result)
            }
        }
    }

    pub async fn all(&self, db: &DatabaseConnection) -> Result<Vec<Value>> {
        match &self.1 {
            None => {
                bail!("请先执行find方法")
            }
            Some(data) => {
                let select = JsonValue::find_by_statement(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    data,
                    [],
                ));
                let result = select.all(db).await?;
                Ok(result)
            }
        }
    }

    pub async fn execute(&self, db: &DatabaseConnection) -> Result<ExecResult> {
        match &self.1 {
            None => {
                bail!("请先执行delete_by_id或者update方法")
            }
            Some(data) => {
                let result = db.execute_unprepared(data).await?;
                Ok(result)
            }
        }
    }
}

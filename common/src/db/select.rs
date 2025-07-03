use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{
    ConnectionTrait, DatabaseBackend, ExecResult, FromQueryResult, JsonValue, SelectModel,
    SelectorRaw, Statement,
};
use serde_json::Value;

use crate::db::db_type::DbType;
use crate::db::form::{parse_value_to_insert_sql, parse_value_to_update_sql};

/// @param 参数1，表名
/// @param 参数2，select 语句
/// @param 参数3，where 参数
pub struct CtsSelect {
    pub table_name: String,
    pub sql: Option<String>,
    pub fields: Option<Vec<String>>,
    pub wheres: Option<String>,
    pub order_by: Option<String>,
}

impl CtsSelect {
    pub fn table(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            sql: None,
            fields: None,
            wheres: None,
            order_by: None,
        }
    }

    pub fn columns(&mut self, fields: Option<Vec<String>>) -> &Self {
        self.fields = fields;
        self
    }

    pub fn filter(&mut self, express: &str) -> &Self {
        match &self.wheres {
            Some(wheres) => {
                self.wheres = Some(format!("{} AND {}", wheres, express));
            }
            None => {
                self.wheres = Some(format!(" WHERE {}", express));
            }
        }
        self
    }

    pub fn defualt_filter(&mut self) -> &Self {
        match &self.wheres {
            Some(wheres) => {
                self.wheres = Some(format!("{} AND {}", wheres, "DELETED_AT IS NULL"));
            }
            None => {
                self.wheres = Some(format!(" WHERE {}", "DELETED_AT IS NULL"));
            }
        }
        self
    }

    pub fn order_by(&mut self, express: &str) -> &Self {
        match &self.order_by {
            Some(order_by) => {
                self.order_by = Some(format!("{}, {}", order_by, express));
            }
            None => {
                self.order_by = Some(format!("ORDER BY {}", express));
            }
        }
        self
    }

    /// 根据编号进行查询
    /// @param id 数据编号
    /// return SelectorRaw
    pub fn select_by_id(&mut self, id: &str) -> SelectorRaw<SelectModel<Value>> {
        // 条件判断
        let wheres = self.wheres.clone().unwrap_or(String::from("WHERE 1=1"));
        let sql = match &self.fields {
            Some(fields) => {
                let field_str = fields.join(",");
                format!(
                    "SELECT {} FROM {} {} AND id='{}'",
                    field_str, self.table_name, wheres, id
                )
            }
            None => {
                format!(
                    "SELECT * FROM {} {} AND id='{}'",
                    self.table_name, wheres, id
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
        let fields = self
            .fields
            .clone()
            .unwrap_or(vec![String::from('*')])
            .join(",");
        let order_by = self.order_by.clone().unwrap_or("".to_string());
        let sql = match &self.wheres {
            Some(wheres) => {
                format!(
                    "SELECT {} FROM {} {} {} ",
                    fields, self.table_name, wheres, order_by
                )
            }
            None => {
                let order_by = self.order_by.clone().unwrap_or("".to_string());
                format!("SELECT {} FROM {} {}", fields, self.table_name, order_by)
            }
        };
        JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [],
        ))
    }

    /// 根据编号进行查询
    /// @param id 数据编号
    /// return Self
    pub fn find_by_id(&mut self, id: &str) -> &Self {
        // 条件判断
        let wheres = self.wheres.clone().unwrap_or(String::from("WHERE 1=1"));

        let sql = match &self.fields {
            Some(fields) => {
                let field_str = fields.join(",");
                format!(
                    "SELECT {} FROM {} {} AND id='{}'",
                    field_str, self.table_name, wheres, id
                )
            }
            None => {
                format!(
                    "SELECT * FROM {} {} AND id='{}'",
                    self.table_name, wheres, id
                )
            }
        };
        self.sql = Some(sql);
        self
    }

    /// 查询数据
    pub fn find(&mut self) -> &Self {
        let fields = self
            .fields
            .clone()
            .unwrap_or(vec![String::from('*')])
            .join(",");
        let order_by = self.order_by.clone().unwrap_or("".to_string());
        let sql = match &self.wheres {
            Some(wheres) => {
                format!(
                    "SELECT {} FROM {} {} {} ",
                    fields, self.table_name, wheres, order_by
                )
            }
            None => {
                format!("SELECT {} FROM {} {}", fields, self.table_name, order_by)
            }
        };
        self.sql = Some(sql);
        self
    }

    /// 查询表结构
    pub fn find_table_field(&mut self) -> &Self {
        let sql = r#"
            select
	a.attnum as "id",
	a.attname as "name",
	concat_ws('', t.typname, SUBSTRING(format_type(a.atttypid, a.atttypmod) from '\(.*\)')) as "type"
from
	pg_attribute a
left join pg_description d on
	d.objoid = a.attrelid
	and d.objsubid = a.attnum
left join pg_class c on
	a.attrelid = c.oid
left join pg_type t on
	a.atttypid = t.oid
where
	a.attnum >= 0
	and c.relname = '{}'
order by
	c.relname desc,
	a.attnum asc
        "#;
        self.sql = Some(sql.replace("{}", &self.table_name));
        self
    }

    /// 删除数据根据数据编号，
    /// @param id 数据编号
    /// @param force 是否彻底删除
    pub fn delete_by_id(&mut self, id: &str, force: bool) -> &Self {
        self.sql = match force {
            true => Some(format!(
                "DELETE FROM {} WHERE id= '{}'",
                self.table_name, id
            )),
            false => {
                let date = Local::now().naive_local();
                Some(format!(
                    "UPDATE {} SET DELETED_AT={} WHERE id= '{}'",
                    self.table_name,
                    date.display(),
                    id
                ))
            }
        };
        self
    }

    /// 更新数据
    /// @param id 数据编号
    /// @param data 数据
    pub fn update(&mut self, id: &str, data: Value) -> Result<&Self> {
        let sql =
            parse_value_to_update_sql(self.table_name.clone(), id.to_string(), data, |_| Ok(()))?;
        self.sql = Some(sql);
        Ok(self)
    }

    /// 添加数据
    /// @param data 数据
    /// @param hande_id 处理数据编号函数，如果不需要处理使用None
    pub fn add<F>(&mut self, data: Value, mut hande_id: F) -> Result<&Self>
    where
        F: FnMut(&String),
    {
        let (id, sql) = parse_value_to_insert_sql(
            self.table_name.clone(),
            data,
            |_| Ok(()),
            |_hearders, _columns| {
                // hearders.push(FormCommonField::Status.to_string());
                // columns.push(Box::new(0));
            },
        )?;
        self.sql = Some(sql);
        hande_id(&id);
        Ok(self)
    }
    /// 查询数据，返回单条数据
    pub async fn one<'a, C>(&self, db: &'a C) -> Result<Option<Value>>
    where
        C: ConnectionTrait,
    {
        match &self.sql {
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
    /// 查询所有数据，根据查询条件进行过滤
    pub async fn all<'a, C>(&self, db: &'a C) -> Result<Vec<Value>>
    where
        C: ConnectionTrait,
    {
        match &self.sql {
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

    pub async fn execute<'a, C>(&self, db: &'a C) -> Result<ExecResult>
    where
        C: ConnectionTrait,
    {
        match &self.sql {
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

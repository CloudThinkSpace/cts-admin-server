use anyhow::{bail, Result};
use axum::extract::Multipart;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use common::db::{create_table_sql, get_db, insert_data_sql};
use entity::form_template::Entity as FormTemplate;
use entity::project::{ActiveModel, Column as ProjectColumn, Entity as Project};
use models::dto::{handler_page, PageResult};
use models::dto::cts::request::project::{SearchProjectDto, UpdateProjectDto};
use models::dto::cts::response::form_template::ResponseFormTemplate;
use models::dto::cts::response::project::ResponseProject;
use project_form::form::form_util::{filter_code_lon_lat, handler_form_data, handler_form_header, parse};
use project_form::project::parse_check_project;
use project_form::request::CsvParse;

/// 根据项目编号查询数据
/// @param id 编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseProject>> {
    let db = get_db().await;
    let result = Project::find_by_id(id.clone())
        // 保障删除字段为空
        .filter(ProjectColumn::DeletedAt.is_null())
        .one(&db).await?;
    match result {
        None => {
            bail!("编号：{}，数据不存在",id)
        }
        Some(data) => {
            let role: ResponseProject = data.into();
            // formTemplate数据
            Ok(Some(role))
        }
    }
}

/// 根据表单编号删除数据
/// @param id 编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询表单信息
    let result = Project::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        match force {
            true => {
                // 删除表单
                let delete_result = Project::delete_by_id(id)
                    .exec(&db)
                    .await?;
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
    } else {
        bail!("该表单不存在".to_string())
    }
}

/// 添加项目
/// @param 项目对象
pub async fn add(mut multipart: Multipart) -> Result<String> {
    let db = get_db().await;
    //  解析数据，如果有文件解析成csv
    let (data, (csv_headers, csv_data)) = multipart.read_csv().await?;
    // 检查项目属性是否正确
    let data = parse_check_project(data)?;
    // 判断名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }
    // 查询表单数据
    let form_template = FormTemplate::find_by_id(data.form_template_id.clone())
        .into_model::<ResponseFormTemplate>()
        .one(&db)
        .await?;
    // 表单字符串
    let form_str = match form_template {
        None => {
            bail!("表单不存在")
        }
        Some(form) => {
            form.content
        }
    };
    // 解析表单数据，转换成表单对象
    let form_template = parse(&form_str)?;
    // 获取事务对象
    let tx = db.begin().await?;
    // 数据表名
    let mut data_table_name = String::from("data_");
    // 任务表名
    let mut task_table_name = String::from("task_");
    // 数据源uuid
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    // 数据表和任务表拼接uuid
    data_table_name.push_str(&uuid);
    task_table_name.push_str(&uuid);
    // 数据表字段，过滤掉无用字段
    let fields: Vec<String> = form_template.form.questions.into_iter().filter(|item| item.r#type!= "SectionType").map(|item| item.name).collect();
    // 根据字段列表，创建数据表sql
    let data_sql = create_table_sql(&data_table_name, &fields, true);
    // 根据字段列表，创建任务表sql
    let task_sql = create_table_sql(&task_table_name, &csv_headers, true);
    // 创建数据表和任务表
    tx.execute_unprepared(&data_sql).await?;
    tx.execute_unprepared(&task_sql).await?;
    // 处理表头，如果有跟公共字段重复的，重命名字段，并且添加公共表字段
    let headers = handler_form_header(&csv_headers);
    // 创建任务字段列表
    let fields_common_index = vec![
        data.task_code.clone(),
        data.task_lon.clone(),
        data.task_lat
    ];
    // 找出code、lon、lat位置列表
    let index_common = filter_code_lon_lat(&headers,&fields_common_index);
    // 收集插入数据sql
    let mut  insert_sqls = Vec::new();
    // 遍历数据列表
    for csv_datum in csv_data.iter() {
        // 转换数据
        let columns = handler_form_data(csv_datum, &index_common);
        // 生成插入数据sql
        let insert_sql = insert_data_sql(&task_table_name, &headers, &columns);
        // 收集sql
        insert_sqls.push(insert_sql);
    }
    // 遍历sql列表
    for insert_sql in insert_sqls.iter() {
        // 执行插入数据sql
        tx.execute_unprepared(&insert_sql).await?;
    }
    // 读入任务数据后赋值该变量
    let total = csv_data.len();
    // 创建项目对象
    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        code: Set(data.code),
        form_template_id: Set(data.form_template_id),
        data_table_name: Set(uuid),
        total: Set(total as i32),
        r#type: Set(data.r#type),
        remark: Set(data.remark),
        status: Set(data.status),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
    };
    // 插入项目数据到数据库
    let add_role = current.insert(&tx).await?;
    // 提交事务
    tx.commit().await?;
    Ok(add_role.id)
}

/// 更新项目信息
/// @param update_project 待更新的表单对象
pub async fn update(id: String, update_project: UpdateProjectDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("编号不能为空".to_string())
    }
    // 查询表单信息
    let result = Project::find_by_id(id).one(&db).await?;
    // 更新表单信息
    if let Some(current) = result {
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_project.name.is_some() {
            current.name = Set(update_project.name.unwrap());
        }
        // 更新code
        if update_project.code.is_some() {
            current.code = Set(update_project.code.unwrap())
        }
        // 更新status
        if update_project.status.is_some() {
            current.status = Set(update_project.status.unwrap())
        }
        // 更新type
        if update_project.r#type.is_some() {
            current.r#type = Set(update_project.r#type.unwrap())
        }
        // 更新备注
        if update_project.remark.is_some() {
            current.remark = Set(Some(update_project.remark.unwrap()))
        }
        // 更新描述
        if update_project.description.is_some() {
            current.description = Set(Some(update_project.description.unwrap()))
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("项目数据不存在，无法更新".to_string())
    }
}

/// 查询项目列表
/// @param data 类型SearchProjectDto
pub async fn search(data: SearchProjectDto) -> Result<PageResult<ResponseProject>> {
    let db = get_db().await;
    let mut select = Project::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(ProjectColumn::Name.contains(data.name.unwrap()))
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(ProjectColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(ProjectColumn::Description.contains(data.description.unwrap()));
    }
    // 判断status是否为空
    if data.status.is_some() {
        select = select.filter(ProjectColumn::Status.eq(data.status.unwrap()))
    }
    // 判断type是否为空
    if data.r#type.is_some() {
        select = select.filter(ProjectColumn::Type.eq(data.r#type.unwrap()))
    }
    // 判断code是否为空
    if data.code.is_some() {
        select = select.filter(ProjectColumn::Code.eq(data.code.unwrap()))
    }
    // 排除已删除角色
    select = select.filter(ProjectColumn::DeletedAt.is_null());
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询Api数据
    let list = paginate.
        fetch_page(page_no - 1)
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect();
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}

// 插入数据
// async fn insert_data(table_name:&str,headers:&Vec<String>, data:Vec<Vec<String>>, tx:&DatabaseTransaction) -> Result<()> {
//
//     let insert_sql = "insert into "
// }

use std::collections::BTreeMap;

use anyhow::{bail, Result};
use axum::body::Bytes;

use models::dto::cts::request::project::AddProjectDto;

use crate::convert::TypeConvert;

pub fn parse_check_project(data: BTreeMap<String, Bytes>) -> Result<AddProjectDto> {
    let mut project = AddProjectDto::default();

    // 核查
    if !data.contains_key("name") {
        bail!("name 字段不能为空")
    } else {
        project.name = data.get("name").unwrap().convert()?;
    }
    if !data.contains_key("code") {
        bail!("code 字段不能为空")
    } else {
        project.code = data.get("code").unwrap().convert()?;
    }
    if !data.contains_key("type") {
        bail!("type 字段不能为空")
    } else {
        project.r#type = data.get("type").unwrap().convert()?;
    }
    if !data.contains_key("formTemplateId") {
        bail!("formTemplateId 字段不能为空")
    } else {
        project.form_template_id = data.get("formTemplateId").unwrap().convert()?;
    }

    if !data.contains_key("taskCode") {
        bail!("taskCode 字段不能为空")
    } else {
        project.task_code = data.get("taskCode").unwrap().convert()?;
    }

    if !data.contains_key("taskLon") {
        bail!("taskLon 字段不能为空")
    } else {
        project.task_lon = data.get("taskLon").unwrap().convert()?;
    }

    if !data.contains_key("taskLat") {
        bail!("taskLat 字段不能为空")
    } else {
        project.task_lat = data.get("taskLat").unwrap().convert()?;
    }

    if data.contains_key("description") {
        project.description = Some(data.get("description").unwrap().convert()?);
    }
    if data.contains_key("remark") {
        project.description = Some(data.get("remark").unwrap().convert()?);
    }

    Ok(project)
}
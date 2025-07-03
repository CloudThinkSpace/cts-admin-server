use std::fs::read;

use anyhow::Result;
use axum::extract::Multipart;

use common::storage::oss::read_oss_object;
use project_form::request::{CtsFile, OssParse};

pub async fn upload(mut multipart: Multipart) -> Result<Vec<CtsFile>> {
    let result = multipart.parse_oss().await?;
    Ok(result)
}

pub async fn read_file(path: String) -> Result<Vec<u8>> {
    let data = read(path)?;
    Ok(data)
}

pub async fn read_oss(path: String) -> Result<Vec<u8>> {
    read_oss_object(&path).await
}

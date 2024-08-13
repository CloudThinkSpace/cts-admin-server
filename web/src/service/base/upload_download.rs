use std::fs::read;

use anyhow::{Ok, Result};
use axum::extract::Multipart;

use project_form::request::{CtsFile, FileParse};

pub async fn upload(mut multipart: Multipart) -> Result<Vec<CtsFile>> {
    let (_, result) = multipart.parse("upload").await?;
    Ok(result)
}

pub async fn read_file(path: String) -> Result<Vec<u8>> {
    let data = read(path)?;
    Ok(data)
}

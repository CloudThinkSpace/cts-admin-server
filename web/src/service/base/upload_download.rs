use anyhow::Result;
use axum::extract::Multipart;

use project_form::request::{CtsFile, FileParse};

pub async fn upload(mut multipart: Multipart) -> Result<Vec<CtsFile>> {

    let (_, result) = multipart.parse("upload").await?;
    Ok(result)
}
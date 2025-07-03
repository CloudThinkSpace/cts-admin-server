use axum::{async_trait, extract::Multipart};
use common::storage::oss::write_oss_object;

use axum::extract::multipart::Field;
use multer::bytes::Buf;

use crate::request::{CtsFile, OssParse};
use anyhow::Result;

#[async_trait]
impl OssParse for Multipart {
    async fn parse_oss(&mut self) -> Result<Vec<CtsFile>> {
        let mut oss_vec = Vec::new();
        while let Some(field) = self.next_field().await.unwrap() {
            // 处理文件
            if let Some(_filename) = field.file_name() {
                let oss_path = stream_to_oss(field).await?;
                oss_vec.push(oss_path);
            }
        }
        Ok(oss_vec)
    }
}

async fn stream_to_oss(stream: Field<'_>) -> Result<CtsFile> {
    let file_name = match &stream.file_name() {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    // 数据
    let body = stream.bytes().await.unwrap();

    let (filename, path) = write_oss_object(&file_name, "/nx", body.chunk()).await?;

    Ok(CtsFile { filename, path })
}

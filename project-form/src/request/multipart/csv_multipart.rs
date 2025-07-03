use async_trait::async_trait;
use std::collections::BTreeMap;

use axum::body::Bytes;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use tokio::io::Error;
use tokio_util::bytes::Buf;

use crate::request::CsvParse;

#[async_trait]
impl CsvParse for Multipart {
    async fn read_csv(
        &mut self,
    ) -> anyhow::Result<(BTreeMap<String, Bytes>, (Vec<String>, Vec<Vec<String>>))> {
        let mut csv_headers = Vec::new();
        let mut csv_data = Vec::new();
        let mut result = BTreeMap::new();
        while let Some(field) = self.next_field().await.unwrap() {
            // 处理文件
            if let Some(_filename) = field.file_name() {
                (csv_headers, csv_data) = stream_to_vec(field).await?;
            } else {
                let name = field.name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                // 匹配属性，不处理文件
                result.insert(name, data);
            }
        }

        Ok((result, (csv_headers, csv_data)))
    }
}

async fn stream_to_vec(stream: Field<'_>) -> Result<(Vec<String>, Vec<Vec<String>>), Error> {
    let mut result_headers = Vec::new();
    let mut result_data = Vec::new();
    let body = stream.bytes().await.unwrap();
    let mut records = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(body.chunk());
    let headers = records.headers().unwrap();
    // 处理表头
    for header in headers.into_iter() {
        result_headers.push(header.to_string())
    }
    // 处理数据
    for record in records.records() {
        let mut row = Vec::new();
        for cell in record.unwrap().into_iter() {
            row.push(cell.to_string());
        }
        result_data.push(row);
    }
    Ok((result_headers, result_data))
}

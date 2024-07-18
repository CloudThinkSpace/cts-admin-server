use async_trait::async_trait;
use std::collections::BTreeMap;

use crate::file_util::create_time_dir;
use axum::body::Bytes;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use axum::BoxError;
use futures_util::stream::Stream;
use futures_util::TryStreamExt;
use tokio::fs::File;
use tokio::io;
use tokio::io::BufWriter;
use tokio_util::bytes::Buf;
use tokio_util::io::StreamReader;
use uuid::Uuid;

use crate::request::{CsvParse, CtsFile, FileParse};

#[async_trait]
impl FileParse for Multipart {
    async fn parse(
        &mut self,
        path: &str,
    ) -> anyhow::Result<(BTreeMap<String, Bytes>, Vec<CtsFile>)> {
        let mut file_paths = Vec::new();
        let mut result = BTreeMap::new();
        while let Some(field) = self.next_field().await.unwrap() {
            // 处理文件
            if let Some(filename) = field.file_name() {
                let file_path = filename.to_string();
                let cts_file = stream_to_file(path, &file_path, field).await?;
                file_paths.push(cts_file)
            } else {
                let name = field.name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                // 匹配属性，不处理文件
                result.insert(name, data);
            }
        }

        Ok((result, file_paths))
    }
}

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

async fn stream_to_file<S, E>(path: &str, filename: &str, stream: S) -> Result<CtsFile, io::Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "文件路径问题".to_string(),
        ));
    }

    async {
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);
        // 分解
        let files: Vec<&str> = filename.split('.').collect();
        if files.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "上次文件错误，该文件没有后缀",
            ));
        }
        let ext = files[1];
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let path_buf = create_time_dir(path).await?;
        let path_buf = std::path::Path::new(&path_buf).join(new_filename);
        let mut file = BufWriter::new(File::create(path_buf.clone()).await?);

        io::copy(&mut body_reader, &mut file).await?;

        Ok(CtsFile {
            path: path_buf.display().to_string(),
            filename: filename.to_string(),
        })
    }
    .await
}

async fn stream_to_vec(stream: Field<'_>) -> Result<(Vec<String>, Vec<Vec<String>>), io::Error> {
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

fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}


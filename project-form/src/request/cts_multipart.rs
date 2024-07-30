use async_trait::async_trait;
use std::collections::BTreeMap;

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
    /// 解析上传文件
    /// @param path 上传的位置
    /// @return 返回form信息和文件列表
    async fn parse(
        &mut self,
        path: &str,
    ) -> anyhow::Result<(BTreeMap<String, Bytes>, Vec<CtsFile>)> {
        // 上传后的文件列表
        let mut file_paths = Vec::new();
        let mut result = BTreeMap::new();
        // 遍历字段
        while let Some(field) = self.next_field().await.unwrap() {
            // 判断是否为文件
            if let Some(filename) = field.file_name() {
                let filename = filename.to_string();
                // 读取数据返回ctsfile对象
                let cts_file = stream_to_file(path, &filename, field).await?;
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
        // 转换错误类型
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        // 流读取对象
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);
        // 分解文件
        let files: Vec<&str> = filename.split('.').collect();
        // 判断是否为文件
        if files.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "上传文件错误，该文件没有后缀",
            ));
        }
        // 文件扩展名
        let ext = files[1];
        // 重命名文件
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        // 创建日期目录
        // let path_buf = create_time_dir(path).await?;
        let path_buf = std::path::Path::new(&path).join(new_filename);
        let mut file = BufWriter::new(File::create(path_buf.clone()).await?);
        // 写入文件
        io::copy(&mut body_reader, &mut file).await?;
        // 组织数据
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

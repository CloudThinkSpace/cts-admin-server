use crate::service::base::upload_download;
use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use response_utils::res::ResResult;

use super::FileType;

/// 上传函数
pub async fn upload(multipart: Multipart) -> impl IntoResponse {
    let result = upload_download::upload(multipart).await;

    match result {
        Ok(data) => ResResult::with_success(data),
        Err(_err) => ResResult::<()>::with_error("用户名或密码错误"),
    }
}

/// 浏览文件函数
pub async fn image(Path(path): Path<String>) -> (HeaderMap, Vec<u8>) {
    let result = upload_download::read_file(path.clone()).await;
    let headers = parse_type(path, FileType::Image);
    match result {
        Ok(data) => (headers, data),
        Err(_err) => (headers, vec![]),
    }
}

/// 下载文件
pub async fn download(Path(path): Path<String>) -> (HeaderMap, Vec<u8>) {
    let result = upload_download::read_file(path.clone()).await;
    let headers = parse_type(path, FileType::Other);
    match result {
        Ok(data) => (headers, data),
        Err(_err) => (headers, vec![]),
    }
}

fn parse_type(path: String, file_type: FileType) -> HeaderMap {
    // 查找是否有点符号
    let index = path.find('.').unwrap_or(usize::MAX);
    //文件扩展名
    let mut ext_name = "png";
    if index != usize::MAX {
        ext_name = &path[index + 1..];
    }
    let content_type = match file_type {
        FileType::Image => {
            format!("image/{}", ext_name)
        }
        FileType::Video => "video/*".to_string(),
        FileType::Other => "application/octet-stream".to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers
}

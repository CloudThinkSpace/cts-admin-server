use axum::extract::Multipart;
use axum::response::IntoResponse;
use response_utils::res::ResResult;
use crate::service::base::upload_download;

/// 上传函数
pub async fn upload(multipart: Multipart) -> impl IntoResponse {
    let result = upload_download::upload(multipart).await;

    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("用户名或密码错误")
        }
    }
}

/// 下载函数
pub async fn download(multipart: Multipart) -> impl IntoResponse {
    let result = upload_download::upload(multipart).await;

    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error("用户名或密码错误")
        }
    }
}
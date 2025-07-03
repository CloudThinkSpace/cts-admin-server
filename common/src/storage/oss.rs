use aliyun_oss_rust_sdk::{oss::OSS, request::RequestBuilder};
use chrono::{Datelike, Local, Timelike};
use uuid::Uuid;

use crate::{config::get_config, file::get_ext};
use anyhow::Result;

/// 创建时间目录
/// @param path 根目录
/// @return path 返回实际路径
pub fn create_oss_dir(path: &str) -> String {
    let date = Local::now();
    let time = format!(
        "{}/{}/{}/{}/{}/{}",
        path,
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute()
    );
    time
}

/// 读取oss对象存储中的对象
/// @param path 对象存储中的路径
/// @return vec<u8> 返回文件二进制
pub async fn read_oss_object(path: &str) -> Result<Vec<u8>> {
    // config
    let config = get_config().await;
    let oss_config = config.oss;
    let oss = OSS::new(
        oss_config.key_id,
        oss_config.key_secret,
        oss_config.endpoint,
        oss_config.bucket,
    );

    let build = RequestBuilder::new();
    let body = oss.get_object(path, build).await?;

    Ok(body)
}

/// 写入oss对象存储
/// @param file_name 文件名
/// @param path 写入oss的路径
/// @para data 写入的数据
pub async fn write_oss_object(
    file_name: &str,
    path: &str,
    data: &[u8],
) -> Result<(String, String)> {
    // config
    let config = get_config().await;
    let oss_config = config.oss;
    let oss = OSS::new(
        oss_config.key_id,
        oss_config.key_secret,
        oss_config.endpoint,
        oss_config.bucket,
    );
    let build = RequestBuilder::new();
    // 根路径
    let oss_path = create_oss_dir(path);
    let uuid = Uuid::new_v4().to_string();
    // 扩展名
    let exp = get_ext(file_name);
    // 文件路径
    let path = format!("{oss_path}/{uuid}.{exp}");
    // 上传文件
    oss.pub_object_from_buffer(&path, data, build).await?;

    Ok((file_name.to_string(), path))
}

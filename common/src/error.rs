use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CtsError {
    #[error("数据查询错误")]
    DbErr(#[from] DbErr),
    #[error("服务器错误：{0}")]
    Server(String),
    #[error("请求错误：{0}")]
    Request(String),
    #[error("系统错误：{0}")]
    Custom(String),
}
use async_trait::async_trait;
use axum::body::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod multipart;

#[derive(Serialize, Deserialize, Debug)]
pub struct CtsFile {
    pub path: String,
    pub filename: String,
}
#[async_trait]
pub trait FileParse {
    async fn parse_file(
        &mut self,
        path: &str,
    ) -> anyhow::Result<(BTreeMap<String, Bytes>, Vec<CtsFile>)>;
}
#[async_trait]
pub trait CsvParse {
    async fn read_csv(
        &mut self,
    ) -> anyhow::Result<(BTreeMap<String, Bytes>, (Vec<String>, Vec<Vec<String>>))>;
}

#[async_trait]
pub trait OssParse {
    async fn parse_oss(&mut self) -> anyhow::Result<Vec<CtsFile>>;
}

use std::io::Cursor;

use crate::storage::oss::read_oss_object;
use anyhow::Result;

pub struct Csv {
    pub header: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl Csv {
    pub async fn read(path: &str) -> Result<Csv> {
        let mut result_headers = Vec::new();
        let mut result_data = Vec::new();

        let body = read_oss_object(path).await?;
        let cursor = Cursor::new(body);
        // 读取数据
        let mut records = csv::Reader::from_reader(cursor);
        // 读取数据表头
        let headers = records.headers().unwrap();
        // 收集表头
        for header in headers.into_iter() {
            result_headers.push(header.to_string())
        }
        // 收集数据
        for record in records.records() {
            let mut row = Vec::new();
            for cell in record.unwrap().into_iter() {
                row.push(cell.to_string());
            }
            result_data.push(row);
        }
        let csv = Csv {
            header: result_headers,
            data: result_data,
        };
        Ok(csv)
    }
}

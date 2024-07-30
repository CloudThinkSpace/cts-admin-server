pub struct Csv {
    pub header: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl Csv {
    pub fn read(path: &str) -> Result<Csv, std::io::Error> {
        let mut result_headers = Vec::new();
        let mut result_data = Vec::new();
        // 读取数据
        let mut records = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;
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

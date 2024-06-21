use serde::{Deserialize, Serialize};

pub mod sys;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    // 分页序号
    pub page_no: Option<u64>,
    // 分页大小
    pub page_size: Option<u64>,
}

#[derive(Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub pages: u64,
    pub page_no: u64,
}

impl<T> PageResult<T> {
    pub fn new(data: Vec<T>, total: u64, pages: u64, page_no: u64) -> Self {
        Self {
            data,
            total,
            page_no,
            pages,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Order {
    // 排序字段
    pub name: String,
    // 排序方向，asc或desc，默认asc
    pub sort: Option<String>,
}

/// 处理分页相关参数
/// return 返回元组，页码和分页大小
pub fn handler_page(page: Option<Page>) -> (u64, u64) {
    if page.is_none() {
        (1, 10)
    } else {
        let page = page.unwrap();
        let mut page_no = 1;
        let mut page_size = 10;
        // 判断页码
        if page.page_no.is_some() {
            page_no = page.page_no.unwrap();
            if page_no == 0 {
                page_no = 1;
            }
        }
        // 判断页数
        if page.page_size.is_some() {
            page_size = page.page_size.unwrap();
            if page_size == 0 {
                page_size = 10;
            }
        }

        (page_no, page_size)
    }
}

use axum::body::Bytes;
use anyhow::{Error, Result};

pub trait TypeConvert<T> {
    fn convert(self) -> Result<T>;
}

impl TypeConvert<String> for &Bytes {
    fn convert(self) -> Result<String> {
        let value = String::from_utf8(self.to_vec()).unwrap_or("".to_string());
        if value.is_empty() {
            Err(Error::msg("值不能为空"))
        } else {
            Ok(value)
        }
    }
}

impl TypeConvert<i32> for &Bytes {
    fn convert(self) -> Result<i32 >{
        let result = String::from_utf8(self.to_vec()).unwrap_or("".to_string());
        if result.is_empty() {
            Err(Error::msg("值不能为空"))
        } else {
            result.parse::<i32>()
                .map_err(|err|Error::msg(err.to_string()))
        }
    }
}


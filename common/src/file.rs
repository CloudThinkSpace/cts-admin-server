use std::path::Path;

use chrono::{Datelike, Local, Timelike};
use tokio::io;

pub mod csv;

pub async fn create_time_dir(path: &str) -> Result<String, io::Error> {
    let date = Local::now();
    let time = format!(
        "{}-{}-{} {}:{}:{}",
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
    );
    let path = Path::new(path).join(time);
    tokio::fs::create_dir_all(path.clone()).await?;
    Ok(path.display().to_string())
}

pub fn get_ext(filename: &str) -> String {
    let index = filename.find('.').unwrap();
    let ext = &filename[index + 1..];
    ext.to_string()
}

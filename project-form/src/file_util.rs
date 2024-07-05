use std::io;
use std::path::Path;
use chrono::{Datelike, Local};

pub async fn create_time_dir(path: &str) -> Result<String, io::Error> {
    let date = Local::now();
    // let time = format!("{}-{}-{} {}:{}:{}", date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second());
    let time = format!("{}-{}-{}", date.year(), date.month(), date.day());
    let path = Path::new(path).join(time);
    tokio::fs::create_dir_all(path.clone()).await?;
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use crate::file_util::create_time_dir;

    #[tokio::test]
    async fn aa() {
        let aa = create_time_dir("upload").await;

        println!("{}", aa.unwrap())
    }
}
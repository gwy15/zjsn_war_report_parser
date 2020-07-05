use std::fs::File;
use std::path::PathBuf;

use log::info;
use serde_json::Value;

use crate::model::War;

pub fn parse_and_write(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = path.to_str().unwrap().to_owned();
    info!("解析文件 {}", path.display());

    let reader = File::open(&path)?;
    let data: Value = serde_json::from_reader(reader)?;

    let _war = War::from(&data, file_name).unwrap_or_else(|| {
        log::error!("文件解析错误：{}", path.display());
        panic!("文件解析错误：{}", path.display())
    });
    // println!("{:#?}", war);

    Ok(())
}

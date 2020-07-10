use std::fs::File;
use std::path::PathBuf;

use glob::glob;
use log::info;
use serde_json::Value;

use crate::model::War;

pub fn parse(path: PathBuf) -> Result<War, Box<dyn std::error::Error>> {
    let file_name = path.to_str().unwrap().to_owned();
    info!("解析文件 {}", path.display());

    let reader = File::open(&path)?;
    let data: Value = serde_json::from_reader(reader)?;

    let war = War::from(&data, file_name).unwrap_or_else(|| {
        log::error!("文件解析错误：{}", path.display());
        panic!("文件解析错误：{}", path.display())
    });
    Ok(war)
}

pub fn format_sheet_name(sheet_name: &str, side: i32) -> String {
    format!("{}_{}", sheet_name, side)
}

pub fn file_finder(root: String) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let paths = ["challenge", "dealto", "spy"];
    let mut files = vec![];
    for path in paths.iter() {
        let pattern = format!("{}/{}/*.json", root, path);
        let msg = format!("Failed to glob files in {}", path);
        for entry in glob(&pattern).expect(&msg) {
            let file = entry?;
            files.push(file);
        }
    }
    Ok(files)
}

pub fn parse_directory(root: String) -> Result<usize, Box<dyn std::error::Error>> {
    // find files
    let mut wars = vec![];
    for path in file_finder(root)? {
        let war = parse(path)?;
        wars.push(war);
    }
    let count = wars.len();
    log::info!("解析了 {} 个文件", wars.len());

    // write
    log::debug!("开始输出");
    let mut writer = crate::writer::Writer::new();
    writer.write(wars);
    log::info!("完成输出");

    Ok(count)
}

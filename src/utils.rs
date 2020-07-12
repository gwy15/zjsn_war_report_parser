use std::fs::File;
use std::path::PathBuf;

use glob::glob;
use log::info;
use serde_json::Value;
use std::io::Read;

use crate::model::War;

pub fn parse(path: PathBuf) -> Result<War, Box<dyn std::error::Error>> {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
    info!("解析文件 {}", path.display());

    let mut reader = File::open(&path)?;
    let mut buf: Vec<u8> = vec![];
    let bytes = reader.read_to_end(&mut buf)?;
    log::debug!("文件 {:?} 读取 {} bytes", path, bytes);
    if buf.starts_with(&[0xef, 0xbb, 0xbf]) {
        log::debug!("检测到 utf-8 with BOM 编码");
        buf = Vec::from(&buf[3..]);
    }

    let data: Value = serde_json::from_slice(&buf)?;

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
        let patterns: Vec<String> = vec![
            format!("{}/{}/*.json", root, path),
            format!("{}/{}/*.txt", root, path),
        ];
        for pattern in patterns {
            let msg = format!("Failed to glob files in {}", path);
            for entry in glob(&pattern).expect(&msg) {
                let file = entry?;
                files.push(file);
            }
        }
    }
    files.sort_unstable();
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

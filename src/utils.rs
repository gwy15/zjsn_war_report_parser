use std::path::PathBuf;

use glob::glob;

use crate::model::War;

#[derive(Debug)]
pub struct ParseTarget {
    pub name: String,
    pub file: PathBuf,
    pub night_file: Option<PathBuf>,
}

impl ParseTarget {
    fn from_path_buf(path: PathBuf) -> Self {
        // 解析名字
        let name = path
            .file_stem()
            .expect("解析文件名错误")
            .to_str()
            .expect("包含非 utf8 字符")
            .replace("_day", "");
        // find night file
        let ext = path.extension().expect("扩展名错误").to_str().unwrap();
        let night_file = path
            .parent()
            .unwrap()
            .join(&format!("{}_night.{}", name, ext));

        let night_file = if night_file.exists() {
            log::debug!("找到夜战文件");
            Some(night_file)
        } else {
            None
        };
        ParseTarget {
            name,
            file: path,
            night_file,
        }
    }

    pub fn from_path(root: String) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        // pve、演习（pvp）、好友演习
        let paths = ["pve", "pvp", "friend",];
        let mut targets = vec![];
        for path in paths.iter() {
            let patterns: Vec<String> = vec![
                format!("./{}/{}/*_day.json", root, path),
                format!("./{}/{}/*_day.txt", root, path),
            ];
            for pattern in patterns {
                let msg = format!("Failed to glob files in {}", path);
                for entry in glob(&pattern).expect(&msg) {
                    let file = entry?;
                    let target = ParseTarget::from_path_buf(file);
                    targets.push(target);
                }
            }
        }
        targets.sort_unstable_by(|l, r| l.name.cmp(&r.name));
        Ok(targets)
    }
}

pub fn format_sheet_name(sheet_name: &str, side: i32) -> String {
    format!("{}_{}", sheet_name, side)
}

pub fn parse_directory(root: String) -> Result<usize, Box<dyn std::error::Error>> {
    // find files
    let mut wars = vec![];
    for target in ParseTarget::from_path(root)? {
        let war = War::from(target.name, target.file, target.night_file)?;
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

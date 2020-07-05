use std::path::PathBuf;

#[macro_use]
extern crate log;
use glob::glob;

mod model;
mod utils;
mod writer;

fn init() {
    use env_logger::{init_from_env, Env};
    init_from_env(Env::default().default_filter_or("info"));
    info!("初始化完成，当前版本: {}", env!("CARGO_PKG_VERSION"));
}

fn file_finder() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let paths = ["challenge", "dealto", "spy"];
    let mut files = vec![];
    for path in paths.iter() {
        let pattern = format!("{}/*.json", path);
        let msg = format!("Failed to glob files in {}", path);
        for entry in glob(&pattern).expect(&msg) {
            let file = entry?;
            files.push(file);
        }
    }
    Ok(files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    // find files
    let mut wars = vec![];
    for path in file_finder()? {
        let war = utils::parse(path)?;
        wars.push(war);
    }
    log::info!("解析了 {} 个文件", wars.len());

    // write
    log::debug!("开始输出");
    let mut writer = writer::Writer::new();
    writer.write(wars);
    log::info!("完成输出");

    // wait for key
    let mut _buf = String::new();
    println!("回车打钱");
    std::io::stdin()
        .read_line(&mut _buf)
        .expect("读取 stdin 错误");

    Ok(())
}

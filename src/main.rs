mod writer;
mod model;
mod utils;

fn init() {
    use env_logger::{init_from_env, Env};
    init_from_env(Env::default().default_filter_or("info"));
    log::info!("初始化完成，当前版本: {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    utils::parse_directory(".".to_string())?;

    // wait for key
    println!("回车打钱");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("读取 stdin 错误");

    Ok(())
}

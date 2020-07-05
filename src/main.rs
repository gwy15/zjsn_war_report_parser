#[macro_use]
extern crate log;

use glob::glob;

mod utils;
mod model;

fn init() {
    use env_logger::{init_from_env, Env};
    init_from_env(Env::default().default_filter_or("info"));
    info!("初始化完成，当前版本: {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    // find files
    for entry in glob("challenge/*.json").expect("Failed to glob files") {
        let path = entry?;
        utils::parse_and_write(path)?;
    }
    Ok(())
}

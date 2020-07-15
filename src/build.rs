#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resource/icon.ico");
    res.compile()?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {}

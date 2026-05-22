use std::fs;
use std::path::{Path, PathBuf};

pub fn read_symlink(path: &std::path::Path) -> std::io::Result<String> {
    let target = std::fs::read_link(path)?;
    Ok(target.to_string_lossy().to_string())
}

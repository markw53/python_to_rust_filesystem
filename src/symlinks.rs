use std::path::Path;

pub fn read_symlink(path: &Path) -> std::io::Result<String> {
    let target = std::fs::read_link(path)?;
    Ok(target.to_string_lossy().to_string())
}

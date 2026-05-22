use filesystem_delta::metadata::{extract_mode, extract_mtime};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_extract_metadata_file() {
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("meta.txt");

    fs::write(&path, "x").unwrap();

    let meta = fs::metadata(&path).unwrap();
    assert!(extract_mode(&meta).is_some());
    assert!(extract_mtime(&meta).is_some());
}

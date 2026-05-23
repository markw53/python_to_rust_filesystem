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

#[test]
fn test_extract_metadata_dir() {
    let tmp = tempdir().unwrap();
    let d = tmp.path().join("folder");
    fs::create_dir(&d).unwrap();

    let meta = fs::symlink_metadata(&d).unwrap();
    assert!(extract_mode(&meta).is_some());
    assert!(extract_mtime(&meta).is_some());
    // dirs have no meaningful size in this implementation
    assert!(meta.is_dir());
}

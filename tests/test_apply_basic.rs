use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_apply_create_file() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();
    fs::write(dst.join("a.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.join("a.txt").is_file());
}

#[test]
fn test_apply_delete_file() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();
    fs::write(src.join("a.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(!src.join("a.txt").exists());
}

use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_edge_case_empty_dirs() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.exists());
}

#[test]
fn test_edge_case_nested() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    let nested = dst.join("a/b/c");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("file.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.join("a/b/c/file.txt").exists());
}

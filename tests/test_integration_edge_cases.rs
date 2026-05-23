use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_edge_case_empty_dirs() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    let ops = compute_delta(
        create_snapshot(desired.to_str().unwrap()),
        create_snapshot(current.to_str().unwrap()),
    );

    apply_patch(desired.to_str().unwrap(), ops).unwrap();

    assert!(desired.exists());
}

#[test]
fn test_edge_case_nested() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    let nested = current.join("a/b/c");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("file.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()),
        create_snapshot(desired.to_str().unwrap()),
    );

    apply_patch(desired.to_str().unwrap(), ops).unwrap();

    assert!(desired.join("a/b/c/file.txt").exists());
}

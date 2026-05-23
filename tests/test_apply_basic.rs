use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_apply_create_file() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();
    fs::write(current.join("a.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()), // desired
        create_snapshot(desired.to_str().unwrap()), // current
    );

    apply_patch(desired.to_str().unwrap(), ops).unwrap();

    assert!(desired.join("a.txt").is_file());
}

#[test]
fn test_apply_delete_file() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();
    fs::write(desired.join("a.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()),
        create_snapshot(desired.to_str().unwrap()),
    );

    apply_patch(desired.to_str().unwrap(), ops).unwrap();

    assert!(!desired.join("a.txt").exists());
}

use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_modify_file_hash_change() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    fs::write(desired.join("a.txt"), "one").unwrap();
    fs::write(current.join("a.txt"), "two").unwrap();

    let ops = compute_delta(
        create_snapshot(desired.to_str().unwrap()),
        create_snapshot(current.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "modify_file" && o.path == "a.txt"));
}

#[test]
fn test_modify_file_no_change() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    fs::write(desired.join("a.txt"), "same").unwrap();
    fs::write(current.join("a.txt"), "same").unwrap();

    let ops = compute_delta(
        create_snapshot(desired.to_str().unwrap()),
        create_snapshot(current.to_str().unwrap()),
    );

    assert!(ops.is_empty());
}

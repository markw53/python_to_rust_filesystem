use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_modify_file_hash_change() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::write(src.join("a.txt"), "one").unwrap();
    fs::write(dst.join("a.txt"), "two").unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "modify_file" && o.path == "a.txt"));
}

#[test]
fn test_modify_file_no_change() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::write(src.join("a.txt"), "same").unwrap();
    fs::write(dst.join("a.txt"), "same").unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    assert!(ops.is_empty());
}

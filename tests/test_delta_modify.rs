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

#[test]
fn test_type_change_file_to_dir() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::write(src.join("x"), "hello").unwrap();
    fs::create_dir(dst.join("x")).unwrap();

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );

    assert_eq!(ops[0].op, "delete_file");
    assert_eq!(ops[1].op, "create_dir");
    assert_eq!(ops[0].path, "x");
    assert_eq!(ops[1].path, "x");
}

#[test]
fn test_type_change_dir_to_file() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::create_dir(src.join("x")).unwrap();
    fs::write(dst.join("x"), "hello").unwrap();

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );

    assert_eq!(ops[0].op, "delete_dir");
    assert_eq!(ops[1].op, "create_file");
    assert_eq!(ops[0].path, "x");
    assert_eq!(ops[1].path, "x");
}

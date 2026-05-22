use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_roundtrip_simple() {
    let tmp = tempdir().unwrap();
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");

    fs::create_dir(&a).unwrap();
    fs::create_dir(&b).unwrap();

    fs::write(a.join("hello.txt"), "world").unwrap();

    let ops = compute_delta(
        create_snapshot(a.to_str().unwrap()),
        create_snapshot(b.to_str().unwrap()),
    );

    apply_patch(b.to_str().unwrap(), ops).unwrap();

    assert_eq!(fs::read_to_string(b.join("hello.txt")).unwrap(), "world");
}

#[test]
fn test_roundtrip_complex() {
    let tmp = tempdir().unwrap();
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    let c = tmp.path().join("c");

    fs::create_dir(&a).unwrap();
    fs::create_dir(&b).unwrap();
    fs::create_dir(&c).unwrap();

    fs::create_dir_all(a.join("nested/dir")).unwrap();
    fs::write(a.join("nested/dir/file.txt"), "data").unwrap();

    let ops_ab = compute_delta(
        create_snapshot(a.to_str().unwrap()),
        create_snapshot(b.to_str().unwrap()),
    );
    apply_patch(b.to_str().unwrap(), ops_ab).unwrap();

    let ops_bc = compute_delta(
        create_snapshot(b.to_str().unwrap()),
        create_snapshot(c.to_str().unwrap()),
    );
    apply_patch(c.to_str().unwrap(), ops_bc).unwrap();

    assert!(c.join("nested/dir/file.txt").exists());
    assert_eq!(
        fs::read_to_string(c.join("nested/dir/file.txt")).unwrap(),
        "data"
    );
}

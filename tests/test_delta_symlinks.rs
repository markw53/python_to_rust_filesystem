use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::symlink;
use tempfile::tempdir;

#[test]
fn test_symlink_create() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    symlink("t.txt", dst.join("link")).unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    assert!(ops.iter().any(|o| o.op == "symlink" && o.path == "link"));
}

#[test]
fn test_symlink_delete() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    symlink("t.txt", src.join("link")).unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "delete_file" && o.path == "link"));
}

#[test]
fn test_symlink_target_change() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");

    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    symlink("a.txt", src.join("link")).unwrap();
    symlink("b.txt", dst.join("link")).unwrap();

    let ops = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "symlink" && o.path == "link" && o.target.as_deref() == Some("b.txt")));
}

use filesystem_delta::PatchOp;
use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_apply_create_file() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    let ops = vec![PatchOp {
        op: "create_file".into(),
        path: "a.txt".into(),
        contents: None,
        target: None,
        mode: None,
        mtime: None,
    }];

    apply_patch(root.to_str().unwrap(), ops).unwrap();

    assert!(root.join("a.txt").exists());
    assert!(root.join("a.txt").is_file());
}

#[test]
fn test_apply_modify_file_truncates() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    fs::write(root.join("a.txt"), "hello").unwrap();

    let ops = vec![PatchOp {
        op: "modify_file".into(),
        path: "a.txt".into(),
        contents: None,
        target: None,
        mode: None,
        mtime: None,
    }];

    apply_patch(root.to_str().unwrap(), ops).unwrap();

    assert_eq!(fs::read(root.join("a.txt")).unwrap(), b"");
}

#[test]
fn test_apply_idempotent_create_dir() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    fs::create_dir(root.join("d")).unwrap();

    let ops = vec![PatchOp {
        op: "create_dir".into(),
        path: "d".into(),
        contents: None,
        target: None,
        mode: None,
        mtime: None,
    }];

    apply_patch(root.to_str().unwrap(), ops).unwrap();

    assert!(root.join("d").is_dir());
}

#[test]
fn test_apply_idempotent_delete_file() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    // file does not exist — should not error
    let ops = vec![PatchOp {
        op: "delete_file".into(),
        path: "ghost.txt".into(),
        contents: None,
        target: None,
        mode: None,
        mtime: None,
    }];

    apply_patch(root.to_str().unwrap(), ops).unwrap();

    assert!(!root.join("ghost.txt").exists());
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

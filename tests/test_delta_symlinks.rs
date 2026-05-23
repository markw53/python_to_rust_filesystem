use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::symlink;
use tempfile::tempdir;

#[test]
fn test_symlink_create() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    symlink("t.txt", current.join("link")).unwrap();

    let snap_desired = create_snapshot(desired.to_str().unwrap());
    let snap_current = create_snapshot(current.to_str().unwrap());

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()), // desired
        create_snapshot(desired.to_str().unwrap()), // current
    );

    assert!(ops.iter().any(|o| o.op == "symlink" && o.path == "link"));
}

#[test]
fn test_symlink_delete() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    symlink("t.txt", desired.join("link")).unwrap();

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()),
        create_snapshot(desired.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "delete_file" && o.path == "link"));
}

#[test]
fn test_symlink_target_change() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    symlink("a.txt", desired.join("link")).unwrap();
    symlink("b.txt", current.join("link")).unwrap();

    let ops = compute_delta(
        create_snapshot(current.to_str().unwrap()),
        create_snapshot(desired.to_str().unwrap()),
    );

    assert!(ops
        .iter()
        .any(|o| o.op == "symlink" && o.path == "link" && o.target.as_deref() == Some("b.txt")));
}

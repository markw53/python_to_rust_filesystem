use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::tempdir;

#[test]
fn test_chmod_change() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    fs::write(desired.join("a.txt"), "x").unwrap();
    fs::write(current.join("a.txt"), "x").unwrap();

    let mut perms = fs::metadata(current.join("a.txt")).unwrap().permissions();
    perms.set_mode(0o777);
    fs::set_permissions(current.join("a.txt"), perms).unwrap();

    let ops = compute_delta(
        create_snapshot(desired.to_str().unwrap()),
        create_snapshot(current.to_str().unwrap()),
    );

    assert!(ops.iter().any(|o| o.op == "chmod" && o.path == "a.txt"));
}

#[test]
fn test_utimes_change() {
    let tmp = tempdir().unwrap();
    let desired = tmp.path().join("desired");
    let current = tmp.path().join("current");

    fs::create_dir(&desired).unwrap();
    fs::create_dir(&current).unwrap();

    fs::write(desired.join("a.txt"), "x").unwrap();
    fs::write(current.join("a.txt"), "x").unwrap();

    // Force a real mtime difference (1 hour earlier).
    let earlier = filetime::FileTime::from_unix_time(1_000_000_000, 0);
    filetime::set_file_mtime(current.join("a.txt"), earlier).unwrap();

    let ops = compute_delta(
        create_snapshot(desired.to_str().unwrap()),
        create_snapshot(current.to_str().unwrap()),
    );

    assert!(ops.iter().any(|o| o.op == "utimes" && o.path == "a.txt"));
}

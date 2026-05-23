use filesystem_delta::{apply_patch, compute_delta, create_snapshot};
use std::fs;
use std::os::unix::fs::symlink;
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

#[test]
fn test_file_dir_conflict() {
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
    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.join("x").is_dir());
}

#[test]
fn test_dir_file_conflict() {
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
    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.join("x").is_file());
}

#[test]
fn test_symlink_loop() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    symlink("b", dst.join("a")).unwrap();
    symlink("a", dst.join("b")).unwrap();

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );
    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src.join("a").is_symlink());
    assert!(src.join("b").is_symlink());
}

#[test]
fn test_large_tree() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    let mut base = dst.clone();
    for i in 0..20 {
        base = base.join(format!("d{}", i));
        fs::create_dir(&base).unwrap();
        fs::write(base.join(format!("f{}.txt", i)), i.to_string()).unwrap();
    }

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );
    apply_patch(src.to_str().unwrap(), ops).unwrap();

    assert!(src
        .join("d0/d1/d2/d3/d4/d5/d6/d7/d8/d9/d10/d11/d12/d13/d14/d15/d16/d17/d18/d19/f19.txt")
        .exists());
}

#[test]
fn test_patch_stability() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::write(dst.join("a.txt"), "hello").unwrap();
    fs::write(dst.join("b.txt"), "world").unwrap();

    let ops1 = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );
    let ops2 = compute_delta(
        create_snapshot(src.to_str().unwrap()),
        create_snapshot(dst.to_str().unwrap()),
    );

    let op_names1: Vec<&str> = ops1.iter().map(|o| o.op.as_str()).collect();
    let op_names2: Vec<&str> = ops2.iter().map(|o| o.op.as_str()).collect();
    let paths1: Vec<&str> = ops1.iter().map(|o| o.path.as_str()).collect();
    let paths2: Vec<&str> = ops2.iter().map(|o| o.path.as_str()).collect();

    assert_eq!(op_names1, op_names2);
    assert_eq!(paths1, paths2);
}

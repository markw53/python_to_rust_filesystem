use filesystem_delta::{compute_delta, create_snapshot};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_delta_create_delete() {
    let tmp = tempdir().unwrap();
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");

    fs::create_dir(&a).unwrap();
    fs::create_dir(&b).unwrap();

    // a has file1, b has file2
    fs::write(a.join("file1.txt"), "one").unwrap();
    fs::write(b.join("file2.txt"), "two").unwrap();

    let ops = compute_delta(
        create_snapshot(a.to_str().unwrap()),
        create_snapshot(b.to_str().unwrap()),
    );

    // Expect: delete file1, create file2
    let mut deletes = 0;
    let mut creates = 0;

    for op in ops {
        if op.op == "delete_file" || op.op == "delete_dir" {
            deletes += 1;
        }
        if op.op == "create_file" {
            creates += 1;
        }
    }

    assert_eq!(deletes, 1);
    assert_eq!(creates, 1);
}

#[test]
fn test_nested_create_ordering() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::create_dir_all(dst.join("a/b")).unwrap();
    fs::write(dst.join("a/b/c.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );

    let paths: Vec<&str> = ops.iter().map(|o| o.path.as_str()).collect();
    assert_eq!(paths, vec!["a", "a/b", "a/b/c.txt"]);
    assert_eq!(ops[0].op, "create_dir");
    assert_eq!(ops[1].op, "create_dir");
    assert_eq!(ops[2].op, "create_file");
}

#[test]
fn test_nested_delete_ordering() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    fs::create_dir(&src).unwrap();
    fs::create_dir(&dst).unwrap();

    fs::create_dir_all(src.join("a/b")).unwrap();
    fs::write(src.join("a/b/c.txt"), "x").unwrap();

    let ops = compute_delta(
        create_snapshot(dst.to_str().unwrap()),
        create_snapshot(src.to_str().unwrap()),
    );

    let paths: Vec<&str> = ops.iter().map(|o| o.path.as_str()).collect();
    assert_eq!(paths, vec!["a/b/c.txt", "a/b", "a"]);
    assert_eq!(ops[0].op, "delete_file");
    assert_eq!(ops[1].op, "delete_dir");
    assert_eq!(ops[2].op, "delete_dir");
}

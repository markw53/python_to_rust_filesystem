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
        if op.op == "delete" {
            deletes += 1;
        }
        if op.op == "create_file" {
            creates += 1;
        }
    }

    assert_eq!(deletes, 1);
    assert_eq!(creates, 1);
}

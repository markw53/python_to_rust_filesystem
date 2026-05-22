use filesystem_delta::symlinks::read_symlink;
use std::fs;
use std::os::unix::fs::symlink;
use tempfile::tempdir;

#[test]
fn test_read_symlink() {
    let tmp = tempdir().unwrap();
    let real = tmp.path().join("real.txt");
    let link = tmp.path().join("link");

    fs::write(&real, "x").unwrap();
    symlink(&real, &link).unwrap();

    let target = read_symlink(link.as_path()).unwrap();

    assert_eq!(target, real.to_string_lossy().to_string());
}

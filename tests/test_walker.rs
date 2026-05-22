use filesystem_delta::walker::walk;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_walk_files_and_dirs() {
    let tmp = tempdir().unwrap();
    let d = tmp.path().join("d");

    fs::create_dir(&d).unwrap();
    fs::write(d.join("a.txt"), "x").unwrap();

    let paths = walk(d.to_str().unwrap());

    assert!(paths.iter().any(|p| p.ends_with("a.txt")));
}

use filesystem_delta::walker::walk;
use std::fs;
use std::os::unix::fs::symlink;
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

#[test]
fn test_walk_symlink() {
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("real.txt");
    fs::write(&target, "data").unwrap();

    let link = tmp.path().join("link.txt");
    symlink(&target, &link).unwrap();

    let entries = walk(tmp.path().to_str().unwrap());
    let types: std::collections::HashMap<_, _> =
        entries.iter().map(|e| (e.as_str(), e.as_str())).collect();

    assert!(entries.contains(&"link.txt".to_string()));
    assert!(entries.contains(&"real.txt".to_string()));
}

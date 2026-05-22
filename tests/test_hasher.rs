use filesystem_delta::hasher::sha256_file;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_sha256_file() {
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("testfile.txt");

    fs::write(&path, "hello").unwrap();

    let hash = sha256_file(path.to_str().unwrap()).unwrap();

    assert_eq!(
        hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

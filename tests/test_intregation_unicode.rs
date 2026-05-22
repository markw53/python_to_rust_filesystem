use filesystem_delta::create_snapshot;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_unicode_paths() {
    let tmp = tempdir().unwrap();
    let d = tmp.path().join("d");

    fs::create_dir(&d).unwrap();
    fs::write(d.join("こんにちは.txt"), "hello").unwrap();

    let snap = create_snapshot(d.to_str().unwrap());

    assert!(snap.entries.iter().any(|e| e.path.contains("こんにちは")));
}

#[test]
fn test_unicode_nested() {
    let tmp = tempdir().unwrap();
    let d = tmp.path().join("d");

    fs::create_dir_all(d.join("привет/世界")).unwrap();
    fs::write(d.join("привет/世界/a.txt"), "x").unwrap();

    let snap = create_snapshot(d.to_str().unwrap());

    assert!(snap.entries.iter().any(|e| e.path.contains("привет")));
    assert!(snap.entries.iter().any(|e| e.path.contains("世界")));
}

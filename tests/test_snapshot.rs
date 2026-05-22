use filesystem_delta::create_snapshot;
use filesystem_delta::filetypes::FileType;
use std::fs;
use std::os::unix::fs::symlink;
use tempfile::tempdir;

#[test]
fn test_snapshot_simple() {
    let tmp = tempdir().unwrap();
    let d = tmp.path().join("d");

    fs::create_dir(&d).unwrap();
    fs::write(d.join("a.txt"), "hello").unwrap();

    let snap = create_snapshot(d.to_str().unwrap());

    assert_eq!(snap.entries.len(), 1);
    let e = &snap.entries[0];

    assert_eq!(e.path, "a.txt");
    assert_eq!(e.file_type, FileType::File);
    assert!(e.hash.is_some());
}

#[test]
fn test_snapshot_symlink() {
    let tmp = tempdir().unwrap();

    let real = tmp.path().join("real.txt");
    let link = tmp.path().join("link");

    fs::write(&real, "x").unwrap();
    symlink(&real, &link).unwrap();

    let snap = create_snapshot(tmp.path().to_str().unwrap());

    let link_entry = snap.entries.iter().find(|e| e.path == "link").unwrap();

    assert_eq!(link_entry.file_type, FileType::Symlink);
    assert_eq!(
        link_entry.target.as_deref(),
        Some(real.to_string_lossy().as_ref())
    );
}

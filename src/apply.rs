use crate::patchop::PatchOp;
use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

fn remove_if_exists(path: &Path) -> io::Result<()> {
    if let Ok(md) = fs::symlink_metadata(path) {
        if md.file_type().is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

fn apply_metadata(path: &Path, mode: Option<u32>, mtime: Option<u64>) -> io::Result<()> {
    if let Some(m) = mode {
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(m);
        fs::set_permissions(path, perms)?;
    }

    if let Some(t) = mtime {
        let atime = fs::metadata(path)?.atime() as u64;
        filetime::set_file_times(
            path,
            filetime::FileTime::from_unix_time(t as i64, 0),
            filetime::FileTime::from_unix_time(atime as i64, 0),
        )?;
    }

    Ok(())
}

fn depth(p: &str) -> usize {
    Path::new(p.trim_start_matches('/').trim_start_matches("./"))
        .components()
        .count()
}

pub fn apply_patch(root: &str, mut ops: Vec<PatchOp>) -> io::Result<()> {
    let root = PathBuf::from(root);

    // Ensure root exists
    if !root.exists() {
        fs::create_dir_all(&root)?;
    }

    // Sort:
    //   Phase 0: deletes (deepest first, so children are removed before parents).
    //   Phase 1: creates/modifies (shallowest first, so parents exist before children;
    //            within the same depth, directories before files/symlinks).
    ops.sort_by(|a, b| {
        fn phase(op: &str) -> u8 {
            match op {
                "delete_file" | "delete_dir" => 0,
                _ => 1,
            }
        }
        fn kind_rank(op: &str) -> u8 {
            match op {
                "create_dir" => 0,
                "file" | "modify_file" | "symlink" => 1,
                "chmod" | "utimes" => 2,
                _ => 3,
            }
        }

        let pa = phase(&a.op);
        let pb = phase(&b.op);
        if pa != pb {
            return pa.cmp(&pb);
        }

        let da = depth(&a.path);
        let db = depth(&b.path);

        if pa == 0 {
            // deletes: deepest first
            db.cmp(&da).then_with(|| a.path.cmp(&b.path))
        } else {
            // creates: shallowest first; dirs before files at the same depth
            da.cmp(&db)
                .then_with(|| kind_rank(&a.op).cmp(&kind_rank(&b.op)))
                .then_with(|| a.path.cmp(&b.path))
        }
    });

    for op in ops {
        let rel = op.path.trim_start_matches('/').trim_start_matches("./");
        let full = root.join(rel);

        println!("APPLY {:?} → {:?}", op.op, full);

        match op.op.as_str() {
            "file" | "create_file" => {
                ensure_parent(&full)?;
                remove_if_exists(&full)?;
                fs::write(&full, op.contents.clone().unwrap_or_default())?;
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "modify_file" => {
                ensure_parent(&full)?;
                fs::write(&full, b"")?; // truncate to zero bytes, matching Python
            }

            "create_dir" => {
                // Don't blow away an existing directory — it may already
                // contain files placed earlier in this same patch.
                // Only replace if something non-directory is sitting in the way.
                if let Ok(md) = fs::symlink_metadata(&full) {
                    if !md.file_type().is_dir() {
                        remove_if_exists(&full)?;
                        fs::create_dir_all(&full)?;
                    }
                } else {
                    fs::create_dir_all(&full)?;
                }
                apply_metadata(&full, op.mode, op.mtime)?;
            }

            "symlink" => {
                ensure_parent(&full)?;
                remove_if_exists(&full)?;
                std::os::unix::fs::symlink(op.target.clone().unwrap(), &full)?;
                // Don't call apply_metadata on symlinks — can't chmod a symlink on Linux
                // and following it to get metadata will loop on circular symlinks
            }

            "chmod" => {
                apply_metadata(&full, op.mode, None)?;
            }

            "utimes" => {
                apply_metadata(&full, None, op.mtime)?;
            }

            "delete_file" => {
                if full.exists() || fs::symlink_metadata(&full).is_ok() {
                    let _ = fs::remove_file(&full);
                }
            }

            "delete_dir" => {
                if full.exists() {
                    let _ = fs::remove_dir_all(&full);
                }
            }

            other => panic!("unknown op {}", other),
        }
    }

    Ok(())
}

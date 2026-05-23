use crate::filetypes::FileType;
use crate::patchop::PatchOp;
use crate::snapshot::{Snapshot, SnapshotEntry};

pub fn compute_delta(desired: Snapshot, current: Snapshot) -> Vec<PatchOp> {
    let mut ops = Vec::new();

    let desired_map = desired
        .entries
        .into_iter()
        .map(|e| (e.path.clone(), e))
        .collect::<std::collections::HashMap<_, _>>();
    let current_map = current
        .entries
        .into_iter()
        .map(|e| (e.path.clone(), e))
        .collect::<std::collections::HashMap<_, _>>();

    // Creations + modifications: walk desired, compare to current
    for (path, d) in desired_map.iter() {
        if path.is_empty() {
            continue;
        }
        if let Some(c) = current_map.get(path) {
            match (&d.file_type, &c.file_type) {
                (FileType::File, FileType::File) => {
                    if d.contents != c.contents {
                        ops.push(PatchOp {
                            op: "modify_file".into(),
                            path: path.clone(),
                            contents: d.contents.clone(),
                            target: None,
                            mode: d.mode,
                            mtime: d.mtime,
                        });
                    } else {
                        if d.mode != c.mode {
                            ops.push(PatchOp {
                                op: "chmod".into(),
                                path: path.clone(),
                                contents: None,
                                target: None,
                                mode: d.mode,
                                mtime: None,
                            });
                        }
                        if d.mtime != c.mtime {
                            ops.push(PatchOp {
                                op: "utimes".into(),
                                path: path.clone(),
                                contents: None,
                                target: None,
                                mode: None,
                                mtime: d.mtime,
                            });
                        }
                    }
                }

                (FileType::Directory, FileType::Directory) => {}

                (FileType::Symlink, FileType::Symlink) => {
                    if d.target != c.target {
                        ops.push(PatchOp {
                            op: "symlink".into(),
                            path: path.clone(),
                            target: d.target.clone(),
                            contents: None,
                            mode: d.mode,
                            mtime: d.mtime,
                        });
                    }
                }

                _ => {
                    ops.push(delete_op_entry(c)); // delete old (current) type
                    ops.push(create_from_entry(d)); // create new (desired) type
                }
            }
        } else {
            ops.push(create_from_entry(d));
        }
    }

    // Deletions: anything in current but not desired
    for (path, c) in current_map.iter() {
        if path.is_empty() {
            continue;
        }
        if !desired_map.contains_key(path) {
            ops.push(delete_op_entry(c));
        }
    }

    ops.sort_by_key(|o| {
        fn depth(p: &str) -> usize {
            p.chars().filter(|&c| c == '/').count()
        }

        match o.op.as_str() {
            "delete_file" | "delete_dir" => {
                // deepest first: negate depth using large number minus depth
                (0u8, 100usize.saturating_sub(depth(&o.path)), o.path.clone())
            }
            "create_dir" | "create_file" | "symlink" => (1u8, depth(&o.path), o.path.clone()),
            _ => (2u8, 0usize, o.path.clone()),
        }
    });

    ops
}

fn create_from_entry(e: &SnapshotEntry) -> PatchOp {
    match e.file_type {
        FileType::File => PatchOp {
            op: "create_file".into(),
            path: e.path.clone(),
            contents: e.contents.clone(),
            target: None,
            mode: e.mode,
            mtime: e.mtime,
        },
        FileType::Directory => PatchOp {
            op: "create_dir".into(),
            path: e.path.clone(),
            contents: None,
            target: None,
            mode: e.mode,
            mtime: e.mtime,
        },
        FileType::Symlink => PatchOp {
            op: "symlink".into(),
            path: e.path.clone(),
            target: e.target.clone(),
            contents: None,
            mode: e.mode,
            mtime: e.mtime,
        },
    }
}

fn delete_op_entry(e: &SnapshotEntry) -> PatchOp {
    match e.file_type {
        FileType::File => PatchOp {
            op: "delete_file".into(),
            path: e.path.clone(),
            contents: None,
            target: None,
            mode: None,
            mtime: None,
        },
        FileType::Directory => PatchOp {
            op: "delete_dir".into(),
            path: e.path.clone(),
            contents: None,
            target: None,
            mode: None,
            mtime: None,
        },
        FileType::Symlink => PatchOp {
            op: "delete_file".into(),
            path: e.path.clone(),
            contents: None,
            target: None,
            mode: None,
            mtime: None,
        },
    }
}

use anyhow::{Context, Result};
use filesystem_delta::{apply_patch, compute_delta, create_snapshot, PatchOp};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: filesystem-delta <compute|apply> [options]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "compute" => {
            let src = get_flag(&args, "--src")?;
            let dst = get_flag(&args, "--dst")?;
            let out = get_flag(&args, "--out")?;

            let snap_src = create_snapshot(&src);
            let snap_dst = create_snapshot(&dst);
            let ops = compute_delta(snap_src, snap_dst);

            let data = serde_json::to_string_pretty(&ops).context("Failed to serialize ops")?;
            std::fs::write(&out, data).context("Failed to write output file")?;
        }

        "apply" => {
            let root = get_flag(&args, "--root")?;
            let patch = get_flag(&args, "--patch")?;
            let dry_run = args.contains(&"--dry-run".to_string());

            let raw = std::fs::read_to_string(&patch).context("Failed to read patch file")?;
            let ops: Vec<PatchOp> =
                serde_json::from_str(&raw).context("Failed to parse patch file")?;

            if dry_run {
                for op in &ops {
                    println!("{:?}", op);
                }
            } else {
                apply_patch(&root, ops).context("Failed to apply patch")?;
            }
        }

        cmd => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn get_flag(args: &[String], flag: &str) -> Result<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
        .with_context(|| format!("Missing required flag: {}", flag))
}

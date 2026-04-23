//! Patch application helpers for build-time source preparation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, bail};
use xshell::{Shell, cmd};

use crate::common::{Result, project_root};

pub fn apply_vendor_patches(sh: &Shell) -> Result<()> {
    let patch_dir = project_root().join("patches/vendor");
    if !patch_dir.exists() {
        return Ok(());
    }

    let mut patch_files = collect_patch_files(&patch_dir)?;
    patch_files.sort();

    for patch in patch_files {
        apply_patch_if_needed(sh, &patch)?;
    }

    Ok(())
}

fn collect_patch_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut patches = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("patch") {
            patches.push(path);
        }
    }
    Ok(patches)
}

fn apply_patch_if_needed(sh: &Shell, patch: &Path) -> Result<()> {
    let vendor_dir =
        PathBuf::from("vendor").join(patch.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default());

    if try_apply_root_patch(sh, patch)? || try_apply_vendor_patch(sh, patch, &vendor_dir)? {
        return Ok(());
    }

    if vendor_patch_looks_applied(patch, &vendor_dir)? {
        return Ok(());
    }

    bail!(
        "vendor patch {} does not apply cleanly and is not already present",
        patch.display()
    );
}

fn try_apply_root_patch(sh: &Shell, patch: &Path) -> Result<bool> {
    if git_apply_check(&["--check", "--recount"], patch, None)? {
        println!("xtask: applying vendor patch {}", patch.display());
        cmd!(sh, "git apply --recount {patch}").run()?;
        return Ok(true);
    }

    if git_apply_check(&["--reverse", "--check", "--recount"], patch, None)? {
        return Ok(true);
    }

    Ok(false)
}

fn try_apply_vendor_patch(sh: &Shell, patch: &Path, vendor_dir: &Path) -> Result<bool> {
    if !vendor_dir.exists() {
        return Ok(false);
    }

    if git_apply_check(&["--check", "--recount", "-p1"], patch, Some(vendor_dir))? {
        println!("xtask: applying vendor patch {} -> {}", patch.display(), vendor_dir.display());
        cmd!(sh, "git apply --recount -p1 --directory {vendor_dir} {patch}").run()?;
        return Ok(true);
    }

    if git_apply_check(&["--reverse", "--check", "--recount", "-p1"], patch, Some(vendor_dir))? {
        return Ok(true);
    }

    Ok(false)
}

fn git_apply_check(args: &[&str], patch: &Path, vendor_dir: Option<&Path>) -> Result<bool> {
    let mut cmd = Command::new("git");
    cmd.current_dir(project_root())
        .arg("apply")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if let Some(vendor_dir) = vendor_dir {
        cmd.arg("--directory").arg(vendor_dir);
    }

    let status = cmd.arg(patch).status().context("failed to run git apply")?;
    Ok(status.success())
}

fn vendor_patch_looks_applied(patch: &Path, vendor_dir: &Path) -> Result<bool> {
    let Some(name) = patch.file_name().and_then(|name| name.to_str()) else {
        return Ok(false);
    };

    match name {
        "crossterm.patch" => {
            Ok(file_contains(vendor_dir.join("src/event/source/thingos.rs"), "UnixInternalEventSource")?
                && file_contains(vendor_dir.join("src/event/sys/unix/waker/thingos.rs"), "pub(crate) struct Waker")?
                && file_contains(vendor_dir.join("src/terminal/sys/thingos.rs"), "pub fn supports_keyboard_enhancement() -> io::Result<bool>")?
                && file_contains(vendor_dir.join("src/tty.rs"), "#[cfg(all(target_os = \"thingos\", feature = \"libc\"))]")?)
        }
        "runa.patch" => {
            Ok(file_contains(vendor_dir.join("Cargo.toml"), "name = \"rn\"")?
                && file_contains(
                    vendor_dir.join("Cargo.toml"),
                    "[target.'cfg(not(target_os = \"thingos\"))'.dependencies]",
                )?
                && file_contains(vendor_dir.join("src/core/worker.rs"), "#[cfg(target_os = \"thingos\")]")?)
        }
        "lsv.patch" => {
            Ok(file_contains(
                vendor_dir.join("Cargo.toml"),
                "[target.'cfg(not(target_os = \"thingos\"))'.dependencies]",
            )?
                && file_contains(vendor_dir.join("src/lib.rs"), "#[cfg(not(target_os = \"thingos\"))]")?
                && file_contains(vendor_dir.join("src/main.rs"), "#[cfg(target_os = \"thingos\")]")?)
        }
        _ => Ok(false),
    }
}

fn file_contains(path: PathBuf, needle: &str) -> Result<bool> {
    let contents =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(contents.contains(needle))
}

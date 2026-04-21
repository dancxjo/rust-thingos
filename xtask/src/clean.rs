//! Clean tasks.

use xshell::{Shell, cmd};

use crate::common::Result;

/// Clean build artifacts (cargo target, bran binaries, iso_root).
///
/// This is a fast, everyday clean — it does not touch vendor trees or
/// downloaded assets.
pub fn clean(sh: &Shell) -> Result<()> {
    println!("Cleaning build artifacts...");
    cmd!(sh, "cargo clean").run()?;
    sh.remove_path("build")?;
    sh.remove_path("iso_root")?;

    // Remove bran arch-specific build directories.
    sh.remove_path("thingos/bran/bin-x86_64")?;
    sh.remove_path("thingos/bran/bin-aarch64")?;
    sh.remove_path("thingos/bran/bin-riscv64")?;
    sh.remove_path("thingos/bran/bin-loongarch64")?;

    // Remove all iso_root_* staging directories and generated images.
    println!("Removing ISO/HDD staging directories and images...");
    for entry in std::fs::read_dir(".").into_iter().flatten() {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("iso_root_") || name_str.starts_with("iso_root") {
                println!("  Removing {}", name_str);
                let _ = sh.remove_path(entry.path());
            } else if name_str.starts_with("thing-os-")
                && (name_str.ends_with(".iso") || name_str.ends_with(".hdd"))
            {
                println!("  Removing {}", name_str);
                let _ = sh.remove_path(entry.path());
            }
        }
    }

    println!("Clean complete.");
    Ok(())
}

/// Deep clean — everything `clean` does, plus vendor trees, downloaded assets.
pub fn distclean(sh: &Shell) -> Result<()> {
    clean(sh)?;

    println!("Cleaning fetched vendor trees...");
    sh.remove_path("vendor/limine")?;
    sh.remove_path("vendor/ovmf")?;
    sh.remove_path("vendor/future-cursors")?;

    println!("Cleaning downloaded assets (preserving wallpapers)...");
    sh.remove_path("assets/cursors")?;
    sh.remove_path("assets/fonts")?;

    if std::path::Path::new("assets/icons").exists() {
        for entry in std::fs::read_dir("assets/icons")? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if name != "thingos" {
                    if path.is_dir() {
                        sh.remove_path(path)?;
                    } else {
                        std::fs::remove_file(path)?;
                    }
                }
            }
        }
    }
    sh.remove_path("assets/pci")?;

    println!("Distclean complete.");
    Ok(())
}

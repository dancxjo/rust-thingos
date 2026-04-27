//! Build and package a stage-1 `rustc` cross-compiler for Thing-OS targets.
//!
//! The primary artifact is produced from `x.py build --stage 1`:
//!
//! 1. Linux-hosted cross-compiler (`target/rustc-thingos/rustc`): runs on
//!    `x86_64-unknown-linux-gnu`, targets Thing-OS target JSONs.
//! 2. ThingOS-native compiler (`target/rustc-thingos/thingos-rustc`): an
//!    optional stage-1 `rustc` binary cross-compiled for
//!    `x86_64-unknown-thingos`.
//!
//! Unlike the old out-of-tree setup, this repository is the Rust source tree.
//! There is no `vendor/rust` indirection here.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use xshell::{Shell, cmd};

const RUSTC_BINARY: &str = "target/rustc-thingos/rustc";
const RUSTC_WRAPPER: &str = "target/rustc-thingos/rustc-wrapper";
const CACHE_KEY_FILE: &str = "target/rustc-thingos/.cache-key";
const OUTPUT_DIR: &str = "target/rustc-thingos";
const RUSTLIB_CACHE_DIR: &str = "target/rustc-thingos/lib/rustlib";
const LIB_CACHE_DIR: &str = "target/rustc-thingos/lib";
pub const THINGOS_RUSTC_BINARY: &str = "target/rustc-thingos/thingos-rustc";
pub const THINGOS_CARGO_BINARY: &str = "target/rustc-thingos/thingos-cargo";
pub const THINGOS_RUSTLIB_CACHE_DIR: &str = "target/rustc-thingos/thingos-rustlib";
const THINGOS_TARGET_SUFFIX: &str = "-unknown-thingos";
const ROOT_CONFIG_TOML: &str = "config.toml";
const ROOT_CONFIG_BACKUP: &str = "target/rustc-thingos/config.toml.backup";

fn compute_cache_key() -> String {
    let mut hasher = DefaultHasher::new();

    for path in &["targets/x86_64-unknown-thingos.json", "rust-toolchain.toml"] {
        if let Ok(content) = std::fs::read_to_string(path) {
            content.hash(&mut hasher);
        }
    }

    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            output.stdout.hash(&mut hasher);
        }
    }

    format!("{:016x}", hasher.finish())
}

fn is_cache_valid() -> bool {
    if !Path::new(RUSTC_BINARY).exists() {
        return false;
    }

    match std::fs::read_to_string(CACHE_KEY_FILE) {
        Ok(stored) => stored.trim() == compute_cache_key(),
        Err(_) => false,
    }
}

fn write_cache_key() -> std::io::Result<()> {
    std::fs::write(CACHE_KEY_FILE, compute_cache_key())
}

fn should_attempt_native_recovery() -> bool {
    std::env::var("BUILD_THINGOS_NATIVE_RUSTC").as_deref() == Ok("1")
}

fn write_rustc_wrapper(cwd: &Path) -> Result<()> {
    let wrapper_path = cwd.join(RUSTC_WRAPPER);
    let compiler_path = cwd.join(RUSTC_BINARY);
    let sysroot = cwd.join("target/rustc-thingos");
    let libdir = cwd.join(LIB_CACHE_DIR);

    let script = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\nexport LD_LIBRARY_PATH=\"{libdir}:${{LD_LIBRARY_PATH:-}}\"\nexec \"{compiler}\" --sysroot \"{sysroot}\" \"$@\"\n",
        libdir = libdir.display(),
        compiler = compiler_path.display(),
        sysroot = sysroot.display(),
    );

    std::fs::write(&wrapper_path, script)?;
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&wrapper_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&wrapper_path, perms)?;
    }

    Ok(())
}

fn xpy_build_root(cwd: &Path) -> PathBuf {
    cwd.join("build/x86_64-unknown-linux-gnu")
}

fn locate_stage1_rustc(cwd: &Path) -> Option<PathBuf> {
    let root_build = xpy_build_root(cwd);
    let candidates = [
        root_build.join("stage1/bin/rustc"),
        root_build.join("stage1-rustc/x86_64-unknown-linux-gnu/release/rustc-main"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn locate_thingos_rustc(cwd: &Path) -> Option<PathBuf> {
    let root_build = xpy_build_root(cwd);
    let candidates = [
        root_build.join("stage1-rustc/x86_64-unknown-thingos/release/rustc-main"),
        cwd.join("build/x86_64-unknown-thingos/stage1/bin/rustc"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn locate_thingos_cargo(cwd: &Path) -> Option<PathBuf> {
    let root_build = xpy_build_root(cwd);
    let candidates = [
        root_build.join("stage1-tools/x86_64-unknown-thingos/release/cargo"),
        cwd.join("build/x86_64-unknown-thingos/stage1/bin/cargo"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn cache_rustlib_tree(sh: &Shell, cwd: &Path) -> Result<()> {
    let build_root = xpy_build_root(cwd);
    let cached_rustlib = cwd.join(RUSTLIB_CACHE_DIR);
    let cached_lib = cwd.join(LIB_CACHE_DIR);

    sh.remove_path(&cached_rustlib)?;
    sh.create_dir(&cached_rustlib)?;
    sh.create_dir(&cached_lib)?;

    let host_rustlib = build_root.join("stage1/lib/rustlib");
    if host_rustlib.exists() {
        let host_src = host_rustlib.to_str().context("non-utf8 rustlib path")?;
        let host_dst = cached_rustlib.to_str().context("non-utf8 rustlib cache path")?;
        cmd!(sh, "cp -r {host_src}/. {host_dst}").run()?;
    }

    let host_lib = build_root.join("stage1/lib");
    if host_lib.exists() {
        for entry in std::fs::read_dir(&host_lib)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(path.extension().and_then(|ext| ext.to_str()), Some("so"))
                || path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.contains(".so."));
            if keep {
                let file_name = path.file_name().context("missing shared library file name")?;
                std::fs::copy(&path, cached_lib.join(file_name))?;
            }
        }
    }

    let thingos_lib = build_root.join("stage1-std/x86_64-unknown-thingos/release/deps");
    if thingos_lib.exists() {
        let thingos_dst = cached_rustlib.join("x86_64-unknown-thingos/lib");
        sh.create_dir(thingos_dst.parent().context("missing ThingOS rustlib parent")?)?;
        sh.create_dir(&thingos_dst)?;
        for entry in std::fs::read_dir(&thingos_lib)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("rlib") | Some("rmeta")
            );
            if keep {
                let file_name = path.file_name().context("missing ThingOS rustlib file name")?;
                std::fs::copy(&path, thingos_dst.join(file_name))?;
            }
        }
    }

    Ok(())
}

fn cache_thingos_rustc_tree(
    sh: &Shell,
    cwd: &Path,
    thingos_rustc: &Path,
    thingos_cargo: Option<&Path>,
) -> Result<()> {
    let cached_dir = cwd.join(THINGOS_RUSTLIB_CACHE_DIR);

    sh.remove_path(&cached_dir)?;
    sh.create_dir(&cached_dir)?;
    sh.copy_file(thingos_rustc, cwd.join(THINGOS_RUSTC_BINARY))?;

    if let Some(cargo) = thingos_cargo {
        sh.copy_file(cargo, cwd.join(THINGOS_CARGO_BINARY))?;
    }

    #[cfg(unix)]
    {
        for bin in &[THINGOS_RUSTC_BINARY, THINGOS_CARGO_BINARY] {
            let binary_path = cwd.join(bin);
            if binary_path.exists() {
                let mut perms = std::fs::metadata(&binary_path)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&binary_path, perms)?;
            }
        }
    }

    let thingos_rlibs_dst = cached_dir.join("x86_64-unknown-thingos/lib");
    sh.create_dir(thingos_rlibs_dst.parent().context("missing native rustlib parent")?)?;
    sh.create_dir(&thingos_rlibs_dst)?;

    let build_root = xpy_build_root(cwd);
    let thingos_std = build_root.join("stage1-std/x86_64-unknown-thingos/release/deps");
    if thingos_std.exists() {
        for entry in std::fs::read_dir(&thingos_std)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("rlib") | Some("rmeta")
            );
            if keep {
                let file_name = path.file_name().context("missing native rustlib file name")?;
                std::fs::copy(&path, thingos_rlibs_dst.join(file_name))?;
            }
        }
    } else {
        println!(
            "rustc-thingos: warning: ThingOS std rlibs not found at {:?}; the ThingOS-native rustc will ship without prebuilt libraries",
            thingos_std
        );
    }

    Ok(())
}

fn bootstrap_config(include_thingos_host: bool, local_rebuild: bool, linker_path: &Path) -> String {
    let host_list = if include_thingos_host {
        "[\"x86_64-unknown-linux-gnu\", \"x86_64-unknown-thingos\"]"
    } else {
        "[\"x86_64-unknown-linux-gnu\"]"
    };

    let local_rebuild = if local_rebuild { "true" } else { "false" };

    format!(
        r#"change-id = "ignore"
# Auto-generated by `cargo run -p xtask -- rustc-thingos` - do not edit by hand.
[build]
build = "x86_64-unknown-linux-gnu"
host = {host_list}
target = ["x86_64-unknown-linux-gnu", "x86_64-unknown-thingos"]
local-rebuild = {local_rebuild}
docs = false
compiler-docs = false

[rust]
optimize = true
debug-assertions = false
codegen-units = 1
lto = "off"
deny-warnings = false
rpath = false

[target.x86_64-unknown-thingos]
linker = "{linker_path}"
sanitizers = false
profiler = false

[llvm]
download-ci-llvm = true
"#,
        linker_path = linker_path.display()
    )
}

fn seed_stage0_thingos_sysroot(cwd: &Path) -> Result<()> {
    let build_root = xpy_build_root(cwd);
    let src = build_root.join("stage0-std/x86_64-unknown-thingos/release/deps");
    let dst = build_root.join("stage0-sysroot/lib/rustlib/x86_64-unknown-thingos/lib");

    if !src.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let keep =
            matches!(path.extension().and_then(|ext| ext.to_str()), Some("rlib") | Some("rmeta"));
        if keep {
            let file_name = path.file_name().context("missing stage0 rustlib file name")?;
            std::fs::copy(&path, dst.join(file_name))?;
        }
    }

    Ok(())
}

fn backup_root_config(sh: &Shell, cwd: &Path) -> Result<bool> {
    let config = cwd.join(ROOT_CONFIG_TOML);
    let backup = cwd.join(ROOT_CONFIG_BACKUP);
    sh.create_dir(backup.parent().context("missing backup parent")?)?;

    if config.exists() {
        std::fs::copy(&config, &backup)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn restore_root_config(cwd: &Path, had_backup: bool) -> Result<()> {
    let config = cwd.join(ROOT_CONFIG_TOML);
    let backup = cwd.join(ROOT_CONFIG_BACKUP);

    if had_backup {
        std::fs::copy(&backup, &config)?;
        std::fs::remove_file(&backup)?;
    } else if config.exists() {
        std::fs::remove_file(&config)?;
    }

    Ok(())
}

fn try_build_thingos_native_rustc(sh: &Shell, cwd: &Path) -> Result<bool> {
    let target_dir = cwd.join("targets");
    let target_dir_str = target_dir.to_str().context("non-utf8 target dir")?;

    let linker_path = cwd.join(
        "build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld",
    );
    std::fs::write(cwd.join(ROOT_CONFIG_TOML), bootstrap_config(true, true, &linker_path))?;
    seed_stage0_thingos_sysroot(cwd)?;

    let native_build = cmd!(sh, "python3 x.py build --stage 1 compiler/rustc src/tools/cargo")
        .env("RUST_TARGET_PATH", target_dir_str)
        .run();

    if let Err(err) = native_build {
        println!(
            "rustc-thingos: warning: native rustc build failed; continuing without ISO rustc: {err}"
        );
        std::fs::write(cwd.join(ROOT_CONFIG_TOML), bootstrap_config(false, true, &linker_path))?;
        return Ok(false);
    }

    match locate_thingos_rustc(cwd) {
        Some(thingos_binary) => {
            let thingos_cargo = locate_thingos_cargo(cwd);
            cache_thingos_rustc_tree(sh, cwd, &thingos_binary, thingos_cargo.as_deref())?;
            println!(
                "rustc-thingos: ThingOS-native binaries (rustc, cargo: {}) cached at {}",
                thingos_cargo.is_some(),
                OUTPUT_DIR
            );
            std::fs::write(
                cwd.join(ROOT_CONFIG_TOML),
                bootstrap_config(false, true, &linker_path),
            )?;
            Ok(true)
        }
        None => {
            println!(
                "rustc-thingos: warning: native rustc build finished but artifact was not found; ISO will not include a native compiler."
            );
            std::fs::write(
                cwd.join(ROOT_CONFIG_TOML),
                bootstrap_config(false, true, &linker_path),
            )?;
            Ok(false)
        }
    }
}

pub fn build_rustc_thingos(sh: &Shell, _arch: &str) -> Result<Option<PathBuf>> {
    if std::env::var("SKIP_RUSTC_THINGOS").as_deref() == Ok("1") {
        return Ok(None);
    }

    let cwd = std::env::current_dir()?;

    if is_cache_valid() {
        println!("rustc-thingos: cache hit, reusing {}", RUSTC_BINARY);
        if !Path::new(THINGOS_RUSTC_BINARY).exists() && should_attempt_native_recovery() {
            println!(
                "rustc-thingos: ThingOS-native rustc missing from cache; attempting a recovery build..."
            );
            let had_backup = backup_root_config(sh, &cwd)?;
            let recovery = try_build_thingos_native_rustc(sh, &cwd);
            restore_root_config(&cwd, had_backup)?;
            recovery?;
        }
        return Ok(Some(PathBuf::from(RUSTC_BINARY)));
    }

    if !cwd.join(".git").exists() {
        bail!("repository root .git directory not found");
    }

    println!("rustc-thingos: building stage-1 rustc (Linux-hosted cross-compiler) ...");
    std::fs::create_dir_all(OUTPUT_DIR)?;

    let had_backup = backup_root_config(sh, &cwd)?;
    let build_result = (|| -> Result<Option<PathBuf>> {
        let linker_path = cwd.join("build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld");
        std::fs::write(cwd.join(ROOT_CONFIG_TOML), bootstrap_config(false, true, &linker_path))?;

        let target_dir = cwd.join("targets");
        let target_dir_str = target_dir.to_str().context("non-utf8 target dir")?;

        cmd!(sh, "python3 x.py build --stage 1 library compiler/rustc")
            .env("RUST_TARGET_PATH", target_dir_str)
            .run()
            .context("x.py stage1 build failed")?;

        let stage1_rustc = locate_stage1_rustc(&cwd).ok_or_else(|| {
            anyhow::anyhow!(
                "stage-1 rustc binary not found after bootstrap under {:?}",
                xpy_build_root(&cwd)
            )
        })?;

        sh.copy_file(&stage1_rustc, RUSTC_BINARY)?;
        cache_rustlib_tree(sh, &cwd)?;
        write_rustc_wrapper(&cwd)?;

        if should_attempt_native_recovery() {
            let _ = try_build_thingos_native_rustc(sh, &cwd)?;
        } else {
            println!(
                "rustc-thingos: ThingOS-native rustc recovery disabled; set BUILD_THINGOS_NATIVE_RUSTC=1 to attempt building {}",
                THINGOS_RUSTC_BINARY
            );
        }

        write_cache_key()?;
        println!("rustc-thingos: Linux-hosted binary cached at {}", RUSTC_BINARY);
        Ok(Some(PathBuf::from(RUSTC_BINARY)))
    })();

    restore_root_config(&cwd, had_backup)?;
    build_result
}

#[allow(dead_code)]
pub fn stage_rustc_for_iso(sh: &Shell, iso_root: &Path) -> Result<()> {
    if std::env::var("SKIP_RUSTC_THINGOS").as_deref() == Ok("1") {
        return Ok(());
    }

    if std::env::var("INCLUDE_RUST_TOOLCHAIN").as_deref() != Ok("1") {
        return Ok(());
    }

    let thingos_rustc = Path::new(THINGOS_RUSTC_BINARY);
    let thingos_cargo = Path::new(THINGOS_CARGO_BINARY);

    if !thingos_rustc.exists() {
        println!(
            "rustc-thingos: ThingOS-native rustc not in cache ({}); skipping toolchain staging.",
            THINGOS_RUSTC_BINARY
        );
        return Ok(());
    }

    let iso_bin = iso_root.join("bin");
    sh.create_dir(&iso_bin)?;

    // Stage rustc
    let iso_rustc = iso_bin.join("rustc");
    sh.copy_file(thingos_rustc, &iso_rustc)?;

    // Stage cargo
    let staged_cargo = thingos_cargo.exists();
    if staged_cargo {
        let iso_cargo = iso_bin.join("cargo");
        sh.copy_file(thingos_cargo, &iso_cargo)?;
        #[cfg(unix)]
        {
            let mut perms = std::fs::metadata(&iso_cargo)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&iso_cargo, perms)?;
        }
    }

    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&iso_rustc)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&iso_rustc, perms)?;
    }

    let thingos_rustlib_src = Path::new(THINGOS_RUSTLIB_CACHE_DIR);
    let staged_rustlib = thingos_rustlib_src.exists();
    if thingos_rustlib_src.exists() {
        let iso_rustlib = iso_root.join("lib/rustlib");
        sh.create_dir(iso_rustlib.parent().context("missing ISO lib parent")?)?;
        sh.create_dir(&iso_rustlib)?;
        let src = thingos_rustlib_src.to_str().context("non-utf8 ThingOS rustlib cache path")?;
        let dst = iso_rustlib.to_str().context("non-utf8 ISO rustlib path")?;
        cmd!(sh, "cp -r {src}/. {dst}").run()?;
    } else {
        println!(
            "rustc-thingos: warning: ThingOS sysroot cache not found at {}; staged rustc will lack prebuilt rlibs",
            THINGOS_RUSTLIB_CACHE_DIR
        );
    }

    validate_staged_iso_toolchain(iso_root, staged_cargo, staged_rustlib)?;

    Ok(())
}

fn validate_staged_iso_toolchain(
    iso_root: &Path,
    expect_cargo: bool,
    expect_thingos_rustlib: bool,
) -> Result<()> {
    let rustc_path = iso_root.join("bin/rustc");
    if !rustc_path.exists() {
        bail!(
            "rustc-thingos: ISO staging validation failed: expected staged rustc at {}",
            rustc_path.display()
        );
    }

    if expect_cargo {
        let cargo_path = iso_root.join("bin/cargo");
        if !cargo_path.exists() {
            bail!(
                "rustc-thingos: ISO staging validation failed: expected staged cargo at {}",
                cargo_path.display()
            );
        }
    }

    if expect_thingos_rustlib {
        let rustlib_root = iso_root.join("lib/rustlib");
        if !rustlib_root.exists() {
            bail!(
                "rustc-thingos: ISO staging validation failed: expected staged rustlib root at {}",
                rustlib_root.display()
            );
        }

        let mut staged_thingos_lib_dirs = Vec::new();
        for entry in std::fs::read_dir(&rustlib_root).with_context(|| {
            format!("failed to read staged rustlib root {}", rustlib_root.display())
        })? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let triple = entry.file_name();
            let triple = triple.to_string_lossy();
            if triple.ends_with(THINGOS_TARGET_SUFFIX) {
                let lib_path = path.join("lib");
                if lib_path.exists() {
                    staged_thingos_lib_dirs.push(lib_path);
                }
            }
        }

        if staged_thingos_lib_dirs.is_empty() {
            bail!(
                "rustc-thingos: ISO staging validation failed: expected a staged ThingOS target rustlib under {}",
                rustlib_root.display()
            );
        }

        for lib_dir in staged_thingos_lib_dirs {
            let mut has_entries = false;
            for entry in std::fs::read_dir(&lib_dir).with_context(|| {
                format!("failed to read staged rustlib directory {}", lib_dir.display())
            })? {
                entry?;
                has_entries = true;
                break;
            }
            if !has_entries {
                bail!(
                    "rustc-thingos: ISO staging validation failed: staged ThingOS rustlib is empty at {}",
                    lib_dir.display()
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::validate_staged_iso_toolchain;

    struct TestTempDir {
        path: PathBuf,
    }

    impl TestTempDir {
        fn new(name: &str) -> Self {
            Self { path: temp_dir_path(name) }
        }

        fn path(&self) -> &std::path::Path {
            &self.path
        }
    }

    impl Drop for TestTempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    fn temp_dir_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let millis_u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to get system time")
            .as_millis();
        let millis = millis_u128.min(u64::MAX as u128) as u64;
        let pid = std::process::id();
        let tid = format!("{:?}", std::thread::current().id());
        path.push(format!("thingos-xtask-{name}-{pid}-{millis}-{tid}"));
        std::fs::create_dir_all(&path).expect("create temp dir");
        path
    }

    #[test]
    fn validate_staged_iso_toolchain_accepts_expected_layout() {
        let iso_root = TestTempDir::new("stage-validate-ok");
        let target_triple = "x86_64-unknown-thingos";
        std::fs::create_dir_all(iso_root.path().join("bin")).expect("create bin");
        std::fs::write(iso_root.path().join("bin/rustc"), b"rustc").expect("write rustc");
        std::fs::write(iso_root.path().join("bin/cargo"), b"cargo").expect("write cargo");
        std::fs::create_dir_all(iso_root.path().join(format!("lib/rustlib/{target_triple}/lib")))
            .expect("create rustlib");
        std::fs::write(
            iso_root.path().join(format!("lib/rustlib/{target_triple}/lib/libcore.rlib")),
            b"rlib",
        )
        .expect("write rustlib artifact");

        let result = validate_staged_iso_toolchain(iso_root.path(), true, true);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_staged_iso_toolchain_rejects_missing_expected_rustlib() {
        let iso_root = TestTempDir::new("stage-validate-missing-rustlib");
        std::fs::create_dir_all(iso_root.path().join("bin")).expect("create bin");
        std::fs::write(iso_root.path().join("bin/rustc"), b"rustc").expect("write rustc");

        let result = validate_staged_iso_toolchain(iso_root.path(), false, true);

        assert!(result.is_err());
    }
}

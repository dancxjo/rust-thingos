//! Build task - compiles the kernel for target architectures.

use crate::common::{Result, profile_subdir, rust_target};
use xshell::{Shell, cmd};

/// Build the kernel for a target architecture.
pub fn build(sh: &Shell, arch: &str, profile: &str) -> Result<()> {
    let target = rust_target(arch);
    let subdir = profile_subdir(profile);

    println!("Building bran kernel for {} ({} profile)...", arch, profile);

    let mut cmd = cmd!(
        sh,
        "cargo build --target {target} --profile {profile} -p bran"
    );
    if cfg!(feature = "diagnostic-apps") {
        cmd = cmd.arg("--features").arg("diagnostic-apps");
    }
    // Enable scheduler contention telemetry (histograms, per-source IPI counters,
    // wait-time tracking) when SCHED_TELEMETRY=1 is set in the environment.
    if std::env::var("SCHED_TELEMETRY").map_or(false, |v| v == "1" || v == "true" || v == "yes") {
        println!("sched-telemetry: enabled (histogram buckets active)");
        cmd = cmd.arg("--features").arg("sched-telemetry");
    }
    cmd.env(
        "RUSTFLAGS",
        "-Awarnings -C relocation-model=static -C panic=abort",
    )
    .run()?;

    let bin_dir = format!("thingos/bran/bin-{}", arch);
    sh.create_dir(&bin_dir)?;

    let src = format!("target/{}/{}/bran", target, subdir);
    let dst = format!("{}/kernel", bin_dir);
    sh.copy_file(&src, &dst)?;

    println!("Kernel built: {}", dst);
    Ok(())
}

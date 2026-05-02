use std::path::PathBuf;
use std::time::Duration;

use bdd::hunter::{FreezeHunterConfig, Ps2InputMode, run};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "freeze_hunter")]
#[command(about = "Stress Thing-OS in QEMU with BDD-shared QMP and serial interactions")]
struct Args {
    #[arg(long, default_value = "x86_64")]
    arch: String,
    #[arg(long, default_value = "15")]
    timeout_secs: u64,
    #[arg(long, default_value = "180")]
    desktop_ready_timeout_secs: u64,
    #[arg(long, default_value = "5")]
    loglevel: String,
    #[arg(long, default_value = "900")]
    action_interval_ms: u64,
    #[arg(long)]
    sessions: Option<u64>,
    #[arg(long)]
    seed: Option<u64>,
    #[arg(long, default_value = "freeze_logs")]
    log_dir: PathBuf,
    /// PS/2 input injection mode.
    ///
    /// normal               – inject as soon as the kernel is seen (default)
    /// wait_for_ready       – wait for ps2.phase=ready before injecting input
    /// adversarial_during_init – inject during PS/2 init to stress the i8042 path
    #[arg(long, default_value = "normal")]
    ps2_input_mode: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ps2_input_mode = args.ps2_input_mode.parse::<Ps2InputMode>().unwrap_or_else(|e| {
        eprintln!("[freeze_hunter] invalid --ps2-input-mode: {e}; using 'normal'");
        Ps2InputMode::Normal
    });
    let config = FreezeHunterConfig {
        arch: args.arch,
        timeout: Duration::from_secs(args.timeout_secs),
        desktop_ready_timeout: Duration::from_secs(args.desktop_ready_timeout_secs),
        loglevel: args.loglevel,
        action_interval: Duration::from_millis(args.action_interval_ms),
        sessions: args.sessions,
        seed: args.seed,
        log_dir: args.log_dir,
        ps2_input_mode,
    };

    if let Err(err) = run(config).await {
        eprintln!("[hunter] fatal: {err}");
        std::process::exit(1);
    }
}

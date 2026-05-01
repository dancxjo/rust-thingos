use std::path::PathBuf;
use std::time::Duration;

use bdd::hunter::{FreezeHunterConfig, run};
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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = FreezeHunterConfig {
        arch: args.arch,
        timeout: Duration::from_secs(args.timeout_secs),
        desktop_ready_timeout: Duration::from_secs(args.desktop_ready_timeout_secs),
        loglevel: args.loglevel,
        action_interval: Duration::from_millis(args.action_interval_ms),
        sessions: args.sessions,
        seed: args.seed,
        log_dir: args.log_dir,
    };

    if let Err(err) = run(config).await {
        eprintln!("[hunter] fatal: {err}");
        std::process::exit(1);
    }
}

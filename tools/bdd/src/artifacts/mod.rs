//! Artifact collection for BDD test runs.
//!
//! Manages screenshots, serial logs, and generates markdown reports.
//! Outputs to `/docs/behavior/${ARCH}/${FEATURE}/${SCENARIO}/${STEP}/`
//! with markdown summaries at each level.

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use tokio::sync::Mutex;

mod collector;
pub mod qmp;
mod types;

pub use collector::*;
pub use qmp::*;
pub use types::*;

/// Global artifact collector instance.
static COLLECTOR: OnceLock<Mutex<ArtifactCollector>> = OnceLock::new();

/// Global serial log cache (updated by world, read by reporter).
static SERIAL_LOG: OnceLock<Mutex<String>> = OnceLock::new();

/// Initialize the global artifact collector for the given architecture.
pub fn init_global(arch: &str) {
    let collector = ArtifactCollector::new(arch);
    let init_result = if std::env::var_os("BDD_FEATURE").is_none() {
        collector.init_clean()
    } else {
        collector.init()
    };
    init_result.unwrap_or_else(|e| {
        panic!("failed to initialize BDD artifact directory for arch {arch}: {e}");
    });
    COLLECTOR
        .set(Mutex::new(collector))
        .unwrap_or_else(|_| panic!("BDD artifact collector already initialized"));
    SERIAL_LOG
        .set(Mutex::new(String::new()))
        .unwrap_or_else(|_| panic!("BDD serial log cache already initialized"));
    QMP_STREAM
        .set(Mutex::new(None))
        .unwrap_or_else(|_| panic!("BDD QMP stream cache already initialized"));
}

/// Get the global artifact collector.
pub fn global() -> &'static Mutex<ArtifactCollector> {
    COLLECTOR.get().expect("ArtifactCollector not initialized - call init_global first")
}

/// Update the global serial log cache (called from world).
pub async fn set_latest_serial(log: &str) {
    if let Some(cache) = SERIAL_LOG.get() {
        let mut serial = cache.lock().await;
        *serial = log.to_string();
    }
}

/// Clear the global serial log cache at scenario boundaries.
pub async fn clear_latest_serial() {
    if let Some(cache) = SERIAL_LOG.get() {
        let mut serial = cache.lock().await;
        serial.clear();
    }
}

/// Get the latest serial log (for reporter to use).
pub async fn get_latest_serial() -> String {
    if let Some(cache) = SERIAL_LOG.get() { cache.lock().await.clone() } else { String::new() }
}

/// Print README content inline to stderr.
///
/// `label` is printed once with the README path, and each README line is then
/// printed with `line_prefix` to keep output grouped and readable in CI logs.
pub fn print_readme_inline(label: &str, readme_path: &Path, line_prefix: &str) {
    eprintln!("{}{} ({})", line_prefix, label, readme_path.display());
    match fs::read_to_string(readme_path) {
        Ok(contents) => {
            for line in contents.lines() {
                eprintln!("{}{}", line_prefix, line);
            }
        }
        Err(e) => eprintln!("{}WARNING: Failed to read README inline: {}", line_prefix, e),
    }
}

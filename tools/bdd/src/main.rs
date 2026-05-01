//! BDD Test Runner for Thing-OS
//!
//! Runs cucumber-rs tests against the OS in QEMU.
//!
//! Configuration via environment variables:
//! - BDD_ARCH: Target architecture (default: x86_64)
//! - BDD_FEATURE: Specific feature file to run (optional)

mod artifacts;
mod reporter;
mod steps;
mod world;

use std::fs;
use std::path::PathBuf;

use cucumber::World;
use reporter::ThingOsReporter;
use world::ThingOsWorld;

#[tokio::main]
async fn main() {
    // Get configuration from environment
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    let feature = std::env::var("BDD_FEATURE").ok();

    eprintln!("[bdd] Running tests for architecture: {}", arch);
    eprintln!("[bdd] CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));

    // Build features path
    // features are in docs/behavior/features relative to workspace root.
    // CARGO_MANIFEST_DIR is tools/bdd. So we need ../../docs/behavior/features
    let features_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // tools
        .parent() // root
        .unwrap()
        .join("docs/behavior/features");
    let features_path = if let Some(f) = feature {
        features_dir.join(format!("{}.feature", f))
    } else {
        features_dir
    };
    eprintln!("[bdd] Features path: {:?}", features_path);

    if !features_path.exists() {
        eprintln!(
            "[bdd] ERROR: requested feature path does not exist: {}",
            features_path.display()
        );
        std::process::exit(2);
    }

    // Create custom reporter with artifact collection
    let reporter = ThingOsReporter::new(&arch);

    // Create output directory
    let output_dir = PathBuf::from("docs/behavior").join(&arch);
    let _ = fs::create_dir_all(&output_dir);

    // Run cucumber
    ThingOsWorld::cucumber()
        .max_concurrent_scenarios(1) // Force sequential execution to avoid global artifact race conditions
        .before(|feature, _rule, scenario, world: &mut ThingOsWorld| {
            Box::pin(async move {
                world.force_bootfb = feature
                    .tags
                    .iter()
                    .chain(scenario.tags.iter())
                    .any(|tag: &String| tag.trim_start_matches('@') == "bootfb");
                world.scenario_timeout_secs = feature
                    .tags
                    .iter()
                    .chain(scenario.tags.iter())
                    .filter_map(|tag: &String| ThingOsWorld::parse_timeout_tag(tag))
                    .last();
            })
        })
        .with_writer(reporter)
        .after(|_feature, _rule, _scenario, _ev, world: Option<&mut ThingOsWorld>| {
            Box::pin(async move {
                if let Some(w) = world {
                    w.shutdown().await;
                }
            })
        })
        .run(features_path)
        .await;

    // ALWAYS generate the report
    let collector = artifacts::global().lock().await;
    let mut arch_readme = None;
    // Generate architecture report
    let report_failed = match collector.generate_arch_readme() {
        Ok(path) => {
            eprintln!("[bdd] Generated: {}", path.display());
            arch_readme = Some(path);
            false
        }
        Err(e) => {
            eprintln!("[bdd] ERROR: Failed to generate README: {}", e);
            true
        }
    };

    let (passed, pending, failed_count) = collector.count_scenarios();
    let total = collector.total_scenarios();
    let has_run_errors = !collector.run_errors().is_empty();
    let no_scenarios_executed = total == 0;
    let failed_run =
        report_failed || has_run_errors || no_scenarios_executed || pending > 0 || failed_count > 0;

    if no_scenarios_executed {
        eprintln!("[bdd] ERROR: no scenarios executed");
    }
    if pending > 0 {
        eprintln!("[bdd] ERROR: {pending} pending/skipped scenario(s)");
    }
    if failed_count > 0 {
        eprintln!("[bdd] ERROR: {failed_count} failed scenario(s)");
    }
    if has_run_errors {
        eprintln!("[bdd] ERROR: {} run-level error(s)", collector.run_errors().len());
    }
    eprintln!("[bdd] Scenario summary: {passed} passed, {pending} pending, {failed_count} failed");

    if failed_run {
        if let Some(path) = arch_readme {
            artifacts::print_readme_inline("[bdd] Inline architecture README", &path, "[bdd] ");
        }
    }

    if failed_run {
        std::process::exit(1);
    }
}

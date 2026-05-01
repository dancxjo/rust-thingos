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

use std::path::PathBuf;
use std::{fs, io};

use cucumber::World;
use reporter::ThingOsReporter;
use world::ThingOsWorld;

#[derive(Default, Debug)]
struct PngCompressionStats {
    scanned: usize,
    optimized: usize,
    before_bytes: u64,
    after_bytes: u64,
    failed: usize,
}

fn collect_png_files(dir: &std::path::Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_png_files(&path, out)?;
            continue;
        }
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn compress_png_lossless(path: &std::path::Path) -> io::Result<(u64, u64, bool)> {
    use image::ImageEncoder;
    use image::codecs::png::{CompressionType, FilterType, PngEncoder};

    let before = fs::metadata(path)?.len();
    let rgba = image::open(path)
        .map_err(|e| io::Error::other(format!("failed to decode PNG {}: {e}", path.display())))?
        .to_rgba8();

    let tmp_path = path.with_extension("png.tmp");
    {
        let file = fs::File::create(&tmp_path)?;
        let encoder =
            PngEncoder::new_with_quality(file, CompressionType::Best, FilterType::Adaptive);
        encoder
            .write_image(
                rgba.as_raw(),
                rgba.width(),
                rgba.height(),
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| {
                io::Error::other(format!("failed to encode compressed PNG {}: {e}", path.display()))
            })?;
    }

    let compressed = fs::metadata(&tmp_path)?.len();
    if compressed < before {
        // Replace atomically where supported, otherwise remove then rename.
        if let Err(err) = fs::rename(&tmp_path, path) {
            let _ = fs::remove_file(path);
            fs::rename(&tmp_path, path).map_err(|_| err)?;
        }
        Ok((before, compressed, true))
    } else {
        let _ = fs::remove_file(&tmp_path);
        Ok((before, before, false))
    }
}

fn compress_png_artifacts(root: &std::path::Path) -> PngCompressionStats {
    let mut stats = PngCompressionStats::default();
    let mut pngs = Vec::new();
    if let Err(err) = collect_png_files(root, &mut pngs) {
        eprintln!(
            "[bdd] WARNING: failed to enumerate PNG artifacts under {}: {}",
            root.display(),
            err
        );
        return stats;
    }

    for png in pngs {
        stats.scanned += 1;
        match compress_png_lossless(&png) {
            Ok((before, after, optimized)) => {
                stats.before_bytes += before;
                stats.after_bytes += after;
                if optimized {
                    stats.optimized += 1;
                }
            }
            Err(err) => {
                stats.failed += 1;
                eprintln!(
                    "[bdd] WARNING: failed to compress PNG artifact {}: {}",
                    png.display(),
                    err
                );
            }
        }
    }

    stats
}

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

    // Create output directory and JSON file
    let output_dir = PathBuf::from("docs/behavior").join(&arch);
    let _ = fs::create_dir_all(&output_dir);
    let json_file =
        fs::File::create(output_dir.join("results.json")).expect("Failed to create results.json");
    let json_writer = cucumber::writer::Json::new::<ThingOsWorld>(json_file);

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
        .with_writer(cucumber::writer::Tee::new(reporter, json_writer))
        .after(|_feature, _rule, _scenario, _ev, world: Option<&mut ThingOsWorld>| {
            Box::pin(async move {
                if let Some(w) = world {
                    w.shutdown().await;
                }
            })
        })
        .run(features_path)
        .await;

    // Final artifact stage: keep repository screenshots small via lossless PNG recompression.
    let compression_stats = compress_png_artifacts(&output_dir);
    if compression_stats.scanned > 0 {
        let bytes_saved =
            compression_stats.before_bytes.saturating_sub(compression_stats.after_bytes);
        eprintln!(
            "[bdd] PNG compression: scanned={} optimized={} failed={} saved={} bytes",
            compression_stats.scanned,
            compression_stats.optimized,
            compression_stats.failed,
            bytes_saved
        );
    }

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

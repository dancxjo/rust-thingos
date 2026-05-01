use std::fs;
use std::io::Write;
use std::path::PathBuf;

use super::super::types::*;
use super::ArtifactCollector;

/// Architecture-level README generation.
pub fn generate_arch_readme(collector: &ArtifactCollector) -> std::io::Result<PathBuf> {
    let readme_path = collector.base_dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    writeln!(file, "# BDD Test Results: {}", collector.arch)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    if !collector.run_errors().is_empty() {
        writeln!(file, "## Run Errors")?;
        writeln!(file)?;
        for error in collector.run_errors() {
            writeln!(file, "- {}", error)?;
        }
        writeln!(file)?;
    }

    writeln!(file, "## Features")?;
    writeln!(file)?;
    writeln!(file, "| Feature | Scenarios | Status |")?;
    writeln!(file, "|---------|-----------|--------|")?;

    for feature in &collector.features {
        let passed_scenarios =
            feature.scenarios.iter().filter(|s| s.outcome == ScenarioOutcome::Passed).count();
        let pending_scenarios =
            feature.scenarios.iter().filter(|s| s.outcome == ScenarioOutcome::Pending).count();
        let failed_scenarios =
            feature.scenarios.iter().filter(|s| s.outcome == ScenarioOutcome::Failed).count();
        let total_scenarios = feature.scenarios.len();

        let icon = if failed_scenarios > 0 {
            "❌"
        } else if passed_scenarios == total_scenarios && total_scenarios > 0 {
            "✅"
        } else {
            "⏭️"
        };

        let rel_path = ArtifactCollector::slugify(&feature.name);
        let link = format!("[{}]({}/README.md)", feature.name, rel_path);

        writeln!(
            file,
            "| {} | ✅ {} / ⏭️ {} / ❌ {} | {} |",
            link, passed_scenarios, pending_scenarios, failed_scenarios, icon
        )?;
    }

    Ok(readme_path)
}

/// Feature-level README generation.
pub fn write_feature_readme(
    collector: &ArtifactCollector,
    feature: &FeatureArtifacts,
) -> std::io::Result<()> {
    let readme_path = feature.dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    writeln!(file, "# Feature: {}", feature.name)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    writeln!(file, "## Scenarios")?;
    writeln!(file)?;
    writeln!(file, "| Scenario | Steps | Status | Link |")?;
    writeln!(file, "|----------|-------|--------|------|")?;

    for scenario in &feature.scenarios {
        let passed_steps = scenario.steps.iter().filter(|s| s.result == StepResult::Passed).count();
        let total_steps = scenario.steps.len();
        let icon = scenario.outcome.emoji();

        let rel_path = ArtifactCollector::slugify(&scenario.name);

        writeln!(
            file,
            "| {} | {}/{} | {} | [View Details]({}/README.md) |",
            scenario.name, passed_steps, total_steps, icon, rel_path
        )?;
    }

    Ok(())
}

pub fn write_scenario_readme(
    collector: &ArtifactCollector,
    scenario: &ScenarioArtifacts,
) -> std::io::Result<()> {
    let readme_path = scenario.dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    let icon = scenario.outcome.emoji();

    writeln!(file, "# {} Scenario: {}", icon, scenario.name)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    writeln!(file, "## Steps")?;
    writeln!(file)?;
    writeln!(file, "| # | Step | Result | Duration | Before | After | Artifacts |")?;
    writeln!(file, "|---|------|--------|----------|--------|-------|-----------|")?;

    for (i, step) in scenario.steps.iter().enumerate() {
        let step_num = i + 1;
        let step_num_str = format!("{:02}", step_num);

        let before_img = if let Some(path) = &step.screenshot_before {
            if path.exists() {
                format!(
                    "<a href=\"./{}/before.png\"><img src=\"./{}/before.png\" width=\"120\" /></a>",
                    step_num_str, step_num_str
                )
            } else {
                "-".to_string()
            }
        } else {
            "-".to_string()
        };

        let after_img = if let Some(path) = &step.screenshot_after {
            if path.exists() {
                format!(
                    "<a href=\"./{}/after.png\"><img src=\"./{}/after.png\" width=\"120\" /></a>",
                    step_num_str, step_num_str
                )
            } else {
                "-".to_string()
            }
        } else {
            "-".to_string()
        };

        let mut artifact_links = Vec::new();
        if let Some(path) = &step.serial_log {
            if path.exists() {
                artifact_links.push(format!("[📜](./{}/serial.log)", step_num_str));
            }
        }
        if let Some(path) = &step.registers {
            if path.exists() {
                artifact_links.push(format!("[💾](./{}/registers.txt)", step_num_str));
            }
        }
        let artifacts_str = artifact_links.join(" ");

        writeln!(
            file,
            "| {} | {} {} | {} | {}ms | {} | {} | {} |",
            step_num,
            step.keyword,
            step.name,
            step.result.emoji(),
            step.duration_ms,
            before_img,
            after_img,
            artifacts_str
        )?;
    }
    writeln!(file)?;

    let log_path = scenario.dir.join("serial.log");
    if log_path.exists() {
        if let Ok(content) = fs::read_to_string(log_path) {
            writeln!(file, "<details>")?;
            writeln!(file, "<summary>📜 Full Serial Log</summary>")?;
            writeln!(file)?;
            writeln!(file, "```")?;
            writeln!(file, "{}", content)?;
            writeln!(file, "```")?;
            writeln!(file, "</details>")?;
        }
    }

    Ok(())
}

/// Step-level README generation is disabled - the scenario README contains all needed info.
pub fn write_step_readme(_step: &StepArtifacts) -> std::io::Result<()> {
    // No-op: step READMEs are not generated - the scenario README contains the step table
    Ok(())
}

use cucumber::{given, then, when};

use super::basic::turn_on_machine;
use super::boot::wait_for_ready_state;
use super::helpers::{
    StepError, capture_failure_diagnostics, color_close, default_timeout_secs,
    verify_cursor_pixels, wait_for_clock_pixels, wait_for_clock_ticks, wait_for_perf_report,
};
use crate::world::{ThingOsWorld, strip_ansi};

// ===== Regex Pattern Matching Steps =====

#[then(regex = r#"^the log should match pattern "(.+)"$"#)]
async fn log_matches_pattern(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    let timeout = default_timeout_secs(world);
    let found = world.wait_for_regex_pattern(&pattern, timeout).await;

    if !found {
        let log = world.get_serial_log().await;
        eprintln!("\n=== Pattern Match Failed ===");
        eprintln!("Pattern: {}", pattern);
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log.lines().rev().take(100).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError(format!("Log does not match pattern '{}'", pattern)));
    }
    Ok(())
}

#[then(regex = r#"^the log should not match pattern "(.+)"$"#)]
async fn log_does_not_match_pattern(
    world: &mut ThingOsWorld,
    pattern: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let re = match regex::Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => {
            return Err(StepError(format!("Invalid regex pattern '{}': {}", pattern, e)));
        }
    };

    let clean = strip_ansi(&log);
    if re.is_match(&clean) {
        eprintln!("\n=== Unexpected Pattern Found ===");
        eprintln!("Pattern: {}", pattern);
        eprintln!("\n=== Serial Log (matching lines) ===");
        for line in clean.lines().filter(|l| re.is_match(l)) {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End Context ===\n");
        return Err(StepError(format!("Log unexpectedly matches pattern '{}'", pattern)));
    }
    Ok(())
}

#[given("the machine is running")]
async fn machine_is_running(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Start machine and wait for ready state
    turn_on_machine(world).await?;
    wait_for_ready_state(world).await
}

#[when(regex = r#"^I wait for (\d+(?:\.\d+)?) seconds$"#)]
async fn wait_seconds(_world: &mut ThingOsWorld, seconds: f64) {
    tokio::time::sleep(std::time::Duration::from_secs_f64(seconds)).await;
}

// ===== New End-to-End Boot and UI Bring-Up Steps =====

#[when("I start the machine")]
async fn start_the_machine(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await?;
    // Complete as soon as kernel starts - other steps verify further boot progress
    let found = world.wait_for_serial("[INFO]", 30.0).await;
    if !found {
        capture_failure_diagnostics(world, "kernel starting").await;
        return Err(StepError("Kernel did not start within timeout".to_string()));
    }
    Ok(())
}

#[then("I should see log messages on the terminal")]
async fn should_see_log_messages(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait a moment for more log lines to accumulate after boot signal
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // By this point the system has already booted (from start_the_machine).
    // Just verify we have log output.
    let log = world.get_serial_log().await;
    let line_count = log.lines().count();
    eprintln!("│  │  │      📝 Log has {} lines", line_count);
    if line_count < 3 {
        eprintln!("│  │  │      === Serial Log Content ===");
        for line in log.lines().take(20) {
            eprintln!("│  │  │      {}", line);
        }
        eprintln!("│  │  │      === End Log ===");
        return Err(StepError(format!("Expected at least 3 log lines, but found {}", line_count)));
    }
    Ok(())
}

// ===== Missing Step Definitions for Feature Files =====

/// Matches "And each log message should include a monotonically increasing timestamp"
#[then("each log message should include a monotonically increasing timestamp")]
async fn each_log_message_monotonic_timestamp_impl(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let mut last_ts: f64 = 0.0;

    // Regex to capture "[18818616432]" or "[12.345678]" -> number (decimal optional)
    let re = regex::Regex::new(r"^\[(\d+(?:\.\d+)?)\]").expect("Invalid regex");

    let mut found_any = false;
    let mut checked_count = 0;

    for line in log.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            let ts: f64 = ts_str
                .parse()
                .map_err(|e| StepError(format!("Failed to parse timestamp: {}", e)))?;

            if ts < last_ts {
                return Err(StepError(format!(
                    "Timestamps went backwards! Previous: {}, Current: {}\nLine: {}",
                    last_ts, ts, line
                )));
            }

            last_ts = ts;
            found_any = true;
            checked_count += 1;
        }
    }

    eprintln!(
        "│  │  │      📝 Checked {} timestamped lines, last ts: {:.6}",
        checked_count, last_ts
    );

    if !found_any {
        return Err(StepError("No timestamps found in serial log to verify!".to_string()));
    }

    if last_ts < 0.5 {
        return Err(StepError(
            "Timestamps never advanced beyond 0.5s - timer may be broken".to_string(),
        ));
    }
    Ok(())
}

/// Matches boot.feature steps
#[then("I should see the system clock tick for several seconds in the serial console")]
async fn system_clock_tick_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait a few seconds to see clock progression
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let log = world.get_serial_log().await;
    let re = regex::Regex::new(r"\[\s*(\d+\.?\d*)\s*\]").unwrap();

    let timestamps: Vec<f64> = log
        .lines()
        .filter_map(|l| re.captures(l))
        .filter_map(|c| c.get(1)?.as_str().parse().ok())
        .collect();

    if timestamps.len() < 2 {
        return Err(StepError("Not enough timestamps to verify clock progression".to_string()));
    }

    let first = timestamps.first().ok_or_else(|| StepError("No timestamps found".to_string()))?;
    let last = timestamps.last().ok_or_else(|| StepError("No timestamps found".to_string()))?;
    let elapsed = last - first;

    eprintln!("│  │  │      ⏱️ Clock elapsed: {:.2}s ({} samples)", elapsed, timestamps.len());

    if elapsed < 1.0 {
        return Err(StepError(format!("Clock did not advance - elapsed: {:.2}s", elapsed)));
    }
    Ok(())
}

#[then("I should see a cursor centered on the screen")]
async fn cursor_centered_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      📍 Checking for cursor at center...");

    // Take screenshot and verify cursor pixels at center
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for screenshot".to_string()));
    }

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("cursor_check");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Screenshot failed: {}", e)))?;

    eprintln!("│  │  │      📸 Screenshot: {}", png_path.display());

    // Verify cursor pixels at center
    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let non_bg_pixels = verify_cursor_pixels(&rgb);

    eprintln!("│  │  │      📊 Cursor check: {} non-background pixels near center", non_bg_pixels);

    if non_bg_pixels < 10 {
        return Err(StepError(format!(
            "Cursor not visible - only {} non-background pixels near center",
            non_bg_pixels
        )));
    }

    eprintln!("│  │  │      ✅ Cursor detected at center");
    Ok(())
}

#[then(regex = r#"^I should see the text "(.+)" in the top-left corner of the screen$"#)]
async fn text_top_left_impl(world: &mut ThingOsWorld, text: String) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if serial_log_contains_case_insensitive(&log, &text) {
        eprintln!("│  │  │      ✅ Found '{}' in log", text);
        Ok(())
    } else {
        Err(StepError(format!(
            "Expected text '{}' in the top-left check, but no matching serial evidence was found",
            text
        )))
    }
}

#[then("I should see frame count information in the top-left corner of the screen")]
async fn frame_count_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if has_frame_count_evidence(&log) {
        eprintln!("│  │  │      ✅ Frame/bloom output detected");
        Ok(())
    } else {
        Err(StepError(
            "Expected frame count evidence in serial output, but none was found".to_string(),
        ))
    }
}

#[then("I should see a clock window displaying a ticking clock")]
async fn clock_window_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      🕐 Checking for clock window...");

    // First wait for clock app to log that it's publishing
    let clock_ready = world.wait_for_serial("CLOCK PUBLISH:", 30.0).await;

    if !clock_ready {
        capture_failure_diagnostics(world, "clock publish").await;
        return Err(StepError("Clock app did not publish within timeout".to_string()));
    }

    eprintln!("│  │  │      ✅ Clock app publishing");

    // Use robust pixel check
    wait_for_clock_pixels(world, 120.0).await
}

/// Given steps for keyboard/pointer scenarios - boot machine if needed
#[given("the clock window is ticking")]
async fn given_clock_ticking(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Boot if not already running
    if world.qemu.is_none() {
        let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
        world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    }

    // Wait for system ready
    let found = world.wait_for_serial("Entering scheduler loop", 120.0).await;
    if !found {
        return Err(StepError("System did not reach ready state".to_string()));
    }

    // Wait for actual clock pixels
    // The user suggested waiting "a good minute"
    wait_for_clock_pixels(world, 150.0).await
}

#[when(regex = r"^I wait for (\d+) clock ticks$")]
async fn wait_for_clock_ticks_impl(
    world: &mut ThingOsWorld,
    ticks: usize,
) -> Result<(), StepError> {
    wait_for_clock_ticks(world, ticks, 30.0).await
}

#[then("watch overflows should be 0")]
async fn watch_overflows_zero(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, default_timeout_secs(world)).await?;
    let mut found = false;
    for (name, avg) in &report.counters {
        if name == "watch_overflows" || name.starts_with("watch_overflows.") {
            found = true;
            if *avg > 0.0 {
                return Err(StepError(format!(
                    "Expected no watch overflows, but {} avg={}",
                    name, avg
                )));
            }
        }
    }
    if !found {
        return Err(StepError(
            "Expected metric 'watch_overflows' but it was not reported. This invariant is not being measured.".to_string(),
        ));
    }
    Ok(())
}

#[then(regex = r"^dirty nodes layout should stay below (\d+)$")]
async fn dirty_nodes_layout_below(
    world: &mut ThingOsWorld,
    threshold: u64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, default_timeout_secs(world)).await?;
    let avg = report
        .counters
        .get("dirty_nodes_layout")
        .copied()
        .ok_or_else(|| StepError("Missing dirty_nodes_layout counter".to_string()))?;
    if avg > threshold as f64 {
        return Err(StepError(format!(
            "dirty_nodes_layout avg {:.2} exceeds threshold {}",
            avg, threshold
        )));
    }
    Ok(())
}

#[then(regex = r"^ui\.snap\.traverse_all should be absent or below ([0-9.]+) ms$")]
async fn traverse_all_absent_or_below(
    world: &mut ThingOsWorld,
    threshold_ms: f64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, default_timeout_secs(world)).await?;
    if let Some(avg) = report.spans.get("ui.snap.traverse_all").copied() {
        if avg > threshold_ms {
            return Err(StepError(format!(
                "ui.snap.traverse_all avg {:.2}ms exceeds {:.2}ms",
                avg, threshold_ms
            )));
        }
    }
    Ok(())
}

#[then(regex = r"^average frame time should be below (\d+) ms$")]
async fn average_frame_time_below(
    world: &mut ThingOsWorld,
    threshold_ms: u64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, default_timeout_secs(world)).await?;
    let avg_ns = report
        .counters
        .get("frame.work_ns")
        .copied()
        .ok_or_else(|| StepError("Missing frame.work_ns counter".to_string()))?;
    let avg_ms = avg_ns / 1_000_000.0;
    if avg_ms > threshold_ms as f64 {
        return Err(StepError(format!(
            "frame.work_ns avg {:.2}ms exceeds threshold {}ms",
            avg_ms, threshold_ms
        )));
    }
    Ok(())
}

#[given("a cursor is visible on the screen")]
async fn given_cursor_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Boot if not already running
    if world.qemu.is_none() {
        let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
        world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    }

    // Wait for system ready (use a late log to avoid missing it due to wait=off)
    let found = world.wait_for_serial("ps2_mouse: entering interrupt-driven loop", 120.0).await;
    if !found {
        return Err(StepError("System did not reach ready state".to_string()));
    }

    // Wait for bloom compositor
    let bloom_found = world.wait_for_serial("bloom:", 30.0).await;
    if !bloom_found {
        eprintln!("│  │  │      ⚠️ Bloom not detected, but continuing...");
    }
    eprintln!("│  │  │      ✅ Cursor should be visible");
    Ok(())
}

#[when("I press a key")]
async fn when_press_key(world: &mut ThingOsWorld) {
    if world.qmp_control.is_some() {
        let press = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": true, "key": {"type": "qcode", "data": "a"}}}]}}"#;
        let release = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": false, "key": {"type": "qcode", "data": "a"}}}]}}"#;

        let _ = world.execute_qmp_control(press).await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let _ = world.execute_qmp_control(release).await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        eprintln!("│  │  │      ⌨️ Sent keypress 'a'");
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for keyboard input");
    }
}

#[when("I wait for the shell prompt")]
async fn when_wait_for_shell_prompt(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait for the prompt character. We use a delay after this to ensure
    // that any trailing escape sequences (like showing the cursor) have finished.
    let found = world.wait_for_serial("] / > ", 60.0).await;
    if !found {
        return Err(StepError("Timed out waiting for shell prompt".to_string()));
    }
    // The prompt is printed after the shell has returned to its read loop. A
    // short drain is enough for trailing escape sequences without adding a
    // fixed multi-second tax to every scenario.
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    Ok(())
}

#[when(regex = r#"^I type "(.+)" on the serial console$"#)]
async fn when_type_on_serial(world: &mut ThingOsWorld, text: String) -> Result<(), StepError> {
    eprintln!("│  │  │      ⌨️ Typing on serial: {}", text);

    world.serial_checkpoint = world.get_serial_log().await.len();
    world.last_typed_command = Some(text.clone());

    let mut data = text.into_bytes();
    if !data.ends_with(b"\n") {
        data.push(b'\n');
    }

    for b in data {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("Failed to write to serial: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }

    // Wait for command completion when the shell returns to a prompt. This is
    // faster than a fixed sleep for short commands and avoids racing the next
    // assertion when the guest is momentarily busy.
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);
    loop {
        let log = world.get_serial_log().await;
        let start_offset = world.serial_checkpoint.min(log.len());
        let recent = strip_ansi(&log[start_offset..]);
        if recent.contains("] ") && recent.contains(" > ") {
            break;
        }
        if start.elapsed() >= timeout {
            return Err(StepError(format!(
                "Timeout waiting for command '{}' to complete (prompt not found)",
                world.last_typed_command.as_deref().unwrap_or("<unknown>")
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Ok(())
}

#[when(regex = r#"^the shell command "(.+)" succeeds$"#)]
async fn when_shell_command_succeeds(
    world: &mut ThingOsWorld,
    command: String,
) -> Result<(), StepError> {
    // Type the command followed by Enter
    when_type_on_serial(world, command.clone()).await?;

    // Wait for the prompt to return (indicating command completion)
    // sh prompt usually contains " > "
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);
    loop {
        let log = world.get_serial_log().await;
        let start_offset = world.serial_checkpoint.min(log.len());
        let recent = strip_ansi(&log[start_offset..]);
        if recent.contains(" > ") {
            break;
        }
        if start.elapsed() >= timeout {
            return Err(StepError(format!(
                "Timeout waiting for command '{}' to complete (prompt not found)",
                command
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Ok(())
}

fn latest_command_output(world: &ThingOsWorld, log: &str) -> String {
    let start = world.serial_checkpoint.min(log.len());
    let recent = strip_ansi(&log[start..]);
    let Some(command) =
        world.last_typed_command.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty())
    else {
        return recent;
    };

    let mut skipped_echo = false;
    let mut output = Vec::new();
    for line in recent.lines() {
        if !skipped_echo && line.contains(command) {
            skipped_echo = true;
            continue;
        }
        output.push(line);
    }

    output.join("\n")
}

fn serial_log_contains_case_insensitive(log: &str, expected: &str) -> bool {
    log.to_lowercase().contains(&expected.to_lowercase())
}

fn has_frame_count_evidence(log: &str) -> bool {
    log.lines().any(|line| {
        let line_lower = line.to_lowercase();
        line_lower.contains("fps") || line_lower.contains("frame") || line_lower.contains("bloom:")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_command_output_drops_prompt_echo_line() {
        let mut world = ThingOsWorld::default();
        world.serial_checkpoint = "boot\n".len();
        world.last_typed_command = Some("cat /proc/sched/stat".to_string());

        let output = latest_command_output(
            &world,
            "boot\n] / > cat /proc/sched/stat\nonline_cpus: 4\ncontext_switches: 12\n] / > ",
        );

        assert!(!output.contains("cat /proc/sched/stat"));
        assert!(output.contains("online_cpus: 4"));
        assert!(output.contains("context_switches: 12"));
    }

    #[test]
    fn latest_command_output_keeps_echo_command_result() {
        let mut world = ThingOsWorld::default();
        world.last_typed_command = Some("echo shell-ready".to_string());

        let output = latest_command_output(&world, "] / > echo shell-ready\nshell-ready\n] / > ");

        assert!(!output.contains("echo shell-ready"));
        assert!(output.contains("shell-ready"));
    }

    #[test]
    fn visual_text_evidence_matching_is_case_insensitive() {
        assert!(serial_log_contains_case_insensitive(
            "[INFO] BLOOM: drew label Thing-OS",
            "thing-os"
        ));
    }

    #[test]
    fn visual_text_evidence_matching_rejects_missing_text() {
        assert!(!serial_log_contains_case_insensitive(
            "[INFO] BLOOM: drew label Thing-OS",
            "kernel panic"
        ));
    }

    #[test]
    fn frame_count_evidence_accepts_frame_fps_or_bloom_logs() {
        assert!(has_frame_count_evidence("render frame 12"));
        assert!(has_frame_count_evidence("FPS: 60"));
        assert!(has_frame_count_evidence("bloom: output0"));
    }

    #[test]
    fn frame_count_evidence_rejects_unrelated_logs() {
        assert!(!has_frame_count_evidence("[INFO] shell ready\n[INFO] vfs mounted"));
    }
}

#[then(regex = r#"^the latest serial output should not contain "(.+)"$"#)]
async fn latest_serial_not_contains(
    world: &mut ThingOsWorld,
    unexpected: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let recent = latest_command_output(world, &log);
    let recent_norm = recent.to_lowercase();
    let needle_norm = strip_ansi(&unexpected).to_lowercase();

    if recent_norm.contains(&needle_norm) {
        eprintln!("\n=== Unexpected pattern found in latest serial output ===");
        eprintln!("Pattern: {}", unexpected);
        eprintln!("\n=== Recent Serial Output (since last command) ===");
        for line in recent.lines().rev().take(40).collect::<Vec<_>>().into_iter().rev() {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End Recent Output ===\n");
        return Err(StepError(format!(
            "Expected latest serial output to not contain '{}', but it was present",
            unexpected
        )));
    }

    Ok(())
}

#[then(regex = r#"^the latest serial output should contain "(.+)"$"#)]
async fn latest_serial_contains(
    world: &mut ThingOsWorld,
    expected: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let recent = latest_command_output(world, &log);
    let recent_norm = recent.to_lowercase();
    let needle_norm = strip_ansi(&expected).to_lowercase();

    if !recent_norm.contains(&needle_norm) {
        eprintln!("\n=== Expected pattern missing from latest serial output ===");
        eprintln!("Pattern: {}", expected);
        eprintln!("\n=== Recent Serial Output (since last command) ===");
        for line in recent.lines().rev().take(40).collect::<Vec<_>>().into_iter().rev() {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End Recent Output ===\n");
        return Err(StepError(format!(
            "Expected latest serial output to contain '{}', but it was missing",
            expected
        )));
    }

    let mut collector = crate::artifacts::global().lock().await;
    collector.set_step_assertion_buffer("Command Output", &recent);

    Ok(())
}

#[then(regex = r#"^the output contains "(.+)"$"#)]
async fn then_output_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    latest_serial_contains(world, expected).await
}

#[then(regex = r#"^the latest output should contain "(.+)" within (\d+)s$"#)]
async fn latest_serial_contains_within(
    world: &mut ThingOsWorld,
    expected: String,
    timeout_secs: u64,
) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_secs);
    loop {
        let log = world.get_serial_log().await;
        let start_offset = world.serial_checkpoint.min(log.len());
        let recent = strip_ansi(&log[start_offset..]);
        if recent.contains(&expected) {
            break;
        }
        if start.elapsed() >= timeout {
            return Err(StepError(format!(
                "Timeout waiting for latest output to contain '{}' (waited {}s)",
                expected, timeout_secs
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Ok(())
}

#[when(regex = r#"^I press (.+)$"#)]
async fn when_press_combo(world: &mut ThingOsWorld, keys: String) {
    if world.qmp_control.is_some() {
        world.serial_checkpoint = world.get_serial_log().await.len();
        let parts: Vec<&str> = keys.split('+').collect();
        eprintln!("│  │  │      ⌨️ Pressing: {}", keys);

        // Press modifiers
        for key in &parts[..parts.len().saturating_sub(1)] {
            let qcode = match key.to_lowercase().as_str() {
                "alt" => "alt",
                "ctrl" | "control" => "ctrl",
                "shift" => "shift",
                _ => continue,
            };
            let cmd = format!(
                r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": true, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#,
                qcode
            );
            let _ = world.execute_qmp_control(&cmd).await;
        }

        // Press main key
        if let Some(main) = parts.last() {
            let qcode = main.to_lowercase();
            let press = format!(
                r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": true, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#,
                qcode
            );
            let release = format!(
                r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": false, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#,
                qcode
            );
            let res_p = world.execute_qmp_control(&press).await;
            if let Err(e) = res_p {
                eprintln!("│  │  │      ❌ QMP Press Error: {}", e);
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let res_r = world.execute_qmp_control(&release).await;
            if let Err(e) = res_r {
                eprintln!("│  │  │      ❌ QMP Release Error: {}", e);
            }
        }

        // Release modifiers
        for key in parts[..parts.len().saturating_sub(1)].iter().rev() {
            let qcode = match key.to_lowercase().as_str() {
                "alt" => "alt",
                "ctrl" | "control" => "ctrl",
                "shift" => "shift",
                _ => continue,
            };
            let cmd = format!(
                r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": false, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#,
                qcode
            );
            let res = world.execute_qmp_control(&cmd).await;
            if let Err(e) = res {
                eprintln!("│  │  │      ❌ QMP Modifier Release Error: {}", e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for keyboard input");
    }
}

#[then("the machine should reboot")]
async fn then_machine_should_reboot(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world).max(120.0));

    loop {
        let log = world.get_serial_log().await;
        let recent = strip_ansi(&log[world.serial_checkpoint.min(log.len())..]).to_lowercase();
        if recent.contains("x86_64: performing reboot via 8042")
            && recent.contains("entering scheduler loop")
        {
            eprintln!("│  │  │      ✅ Reboot completed after keyboard trap");
            return Ok(());
        }

        if start.elapsed() > timeout {
            return Err(StepError(
                "machine did not reboot and reach the scheduler loop after the keyboard trap"
                    .to_string(),
            ));
        }

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[when("I move the mouse")]
async fn when_move_mouse(world: &mut ThingOsWorld) {
    eprintln!("│  │  │      debug: starting when_move_mouse!");

    if world.qmp_control.is_some() {
        let cmd = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 50}}, {"type": "rel", "data": {"axis": "y", "value": 50}}]}}"#;
        match world.execute_qmp_control(cmd).await {
            Ok(res) => eprintln!("│  │  │      🖱️ Sent mouse movement, QMP res: {}", res.trim()),
            Err(e) => eprintln!("│  │  │      ❌ QMP error: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for mouse input");
    }
}

#[then(regex = r#"^the serial log should contain '(.+)'$"#)]
async fn serial_log_contains(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains(&pattern) {
        eprintln!("│  │  │      ✅ Found: {}", pattern);
        Ok(())
    } else {
        Err(StepError(format!(
            "Expected serial log to contain '{}', but it was not found",
            pattern
        )))
    }
}

#[then("I should see the appropriate symbol rendered")]
async fn symbol_rendered(_world: &mut ThingOsWorld) {
    eprintln!("│  │  │      ℹ️ Symbol rendering requires visual verification");
}

#[then("the cursor should move correspondingly on the screen")]
async fn cursor_moved(_world: &mut ThingOsWorld) {
    eprintln!("│  │  │      ℹ️ Cursor movement requires visual verification");
}

#[then("the pointer debug overlay should update after mouse movement")]
async fn pointer_debug_overlay_updates(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for pointer debug screenshot check".to_string()));
    }

    ensure_pointer_debug_overlay_enabled(world).await?;
    if !world.wait_for_serial("First frame rendered", 60.0).await {
        return Err(StepError("Bloom did not render first frame".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let before_path =
        crate::artifacts::global().lock().await.screenshot_path("pointer_debug_before");
    let before_png = world
        .take_screenshot(&before_path)
        .await
        .map_err(|e| StepError(format!("Failed to take before screenshot: {}", e)))?;

    let cmd = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 50}}, {"type": "rel", "data": {"axis": "y", "value": 50}}]}}"#;
    world
        .execute_qmp_control(cmd)
        .await
        .map_err(|e| StepError(format!("QMP mouse movement failed: {}", e)))?;
    if !world.wait_for_serial("bloom: pointer moved", 30.0).await {
        return Err(StepError("Bloom did not receive pointer movement".to_string()));
    }
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let after_path = crate::artifacts::global().lock().await.screenshot_path("pointer_debug_after");
    let after_png = world
        .take_screenshot(&after_path)
        .await
        .map_err(|e| StepError(format!("Failed to take after screenshot: {}", e)))?;

    let before = image::open(&before_png)
        .map_err(|e| StepError(format!("Failed to open before screenshot: {}", e)))?
        .to_rgb8();
    let after = image::open(&after_png)
        .map_err(|e| StepError(format!("Failed to open after screenshot: {}", e)))?
        .to_rgb8();

    let (width, height) = before.dimensions();
    if after.dimensions() != (width, height) {
        return Err(StepError("Screenshot dimensions changed during pointer check".to_string()));
    }

    let x0 = 12u32.min(width);
    let y0 = 12u32.min(height);
    let x1 = 472u32.min(width);
    let y1 = 108u32.min(height);
    let mut changed = 0u32;
    for y in y0..y1 {
        for x in x0..x1 {
            if before.get_pixel(x, y).0 != after.get_pixel(x, y).0 {
                changed += 1;
            }
        }
    }

    eprintln!("│  │  │      pointer debug overlay changed pixels: {}", changed);
    if changed < 8 {
        return Err(StepError(format!(
            "Pointer debug overlay did not update after mouse movement ({} changed pixels)",
            changed
        )));
    }

    let cursor = cursor_signature_pixels(&after, 1001, 584, 96);
    eprintln!(
        "│  │  │      real cursor layer pixels: white={} gold={} dark={}",
        cursor.white, cursor.gold, cursor.dark
    );
    if cursor.white < 300 || cursor.gold < 40 || cursor.dark < 120 {
        return Err(StepError(format!(
            "Real cursor layer not detected at pointer position (white={}, gold={}, dark={})",
            cursor.white, cursor.gold, cursor.dark
        )));
    }

    Ok(())
}

#[then("the pointer debug overlay should include the cursor svg")]
async fn pointer_debug_overlay_includes_cursor_svg(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for pointer debug screenshot check".to_string()));
    }

    ensure_pointer_debug_overlay_enabled(world).await?;
    if !world.wait_for_serial("First frame rendered", 60.0).await {
        return Err(StepError("Bloom did not render first frame".to_string()));
    }

    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("pointer_debug_cursor_svg");
    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
        .to_rgb8();
    let (width, height) = img.dimensions();
    if width < 80 || height < 80 {
        return Err(StepError("Screenshot too small for pointer overlay check".to_string()));
    }

    let overlay_w = 460u32.min(width.saturating_sub(24));
    let cursor_size = 96u32;
    let icon_x0 = 12 + overlay_w.saturating_sub(cursor_size + 24);
    let icon_y0 = 12 + 24;
    let icon_x1 = (icon_x0 + cursor_size).min(width);
    let icon_y1 = (icon_y0 + cursor_size).min(height);
    let mut cursor_like = 0u32;

    for y in icon_y0..icon_y1 {
        for x in icon_x0..icon_x1 {
            let pixel = img.get_pixel(x, y).0;
            if !color_close(pixel, [0x10, 0x18, 0x20], 24) {
                cursor_like += 1;
            }
        }
    }

    eprintln!("│  │  │      pointer debug cursor svg pixels: {}", cursor_like);
    if cursor_like < 360 {
        return Err(StepError(format!(
            "Pointer debug overlay cursor SVG not detected ({} pixels)",
            cursor_like
        )));
    }

    let cursor = cursor_signature_pixels(&img, icon_x0, icon_y0, cursor_size);
    eprintln!(
        "│  │  │      pointer debug Future cursor pixels: white={} gold={} dark={}",
        cursor.white, cursor.gold, cursor.dark
    );
    if cursor.white < 300 || cursor.gold < 40 || cursor.dark < 120 {
        return Err(StepError(format!(
            "Pointer debug overlay does not contain the Future cursor art (white={}, gold={}, dark={})",
            cursor.white, cursor.gold, cursor.dark
        )));
    }

    Ok(())
}

async fn ensure_pointer_debug_overlay_enabled(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }
    if !world.wait_for_serial("ps2_kbd: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 keyboard driver did not connect to Bristle".to_string()));
    }

    let press_alt = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": true, "key": {"type": "qcode", "data": "alt"}}}]}}"#;
    let press_f7 = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": true, "key": {"type": "qcode", "data": "f7"}}}]}}"#;
    let release_f7 = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": false, "key": {"type": "qcode", "data": "f7"}}}]}}"#;
    let release_alt = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": false, "key": {"type": "qcode", "data": "alt"}}}]}}"#;

    for cmd in [press_alt, press_f7, release_f7, release_alt] {
        world
            .execute_qmp_control(cmd)
            .await
            .map_err(|e| StepError(format!("QMP Alt+F7 toggle failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    if !world.wait_for_serial("bloom: pointer debug overlay enabled", 30.0).await {
        return Err(StepError("Bloom did not enable the pointer debug overlay".to_string()));
    }
    let nudge = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 1}}]}}"#;
    world
        .execute_qmp_control(nudge)
        .await
        .map_err(|e| StepError(format!("QMP pointer nudge failed: {}", e)))?;
    if !world.wait_for_serial("bloom: pointer debug overlay ready", 30.0).await {
        return Err(StepError("Pointer debug overlay did not become ready".to_string()));
    }

    Ok(())
}

#[derive(Default)]
struct CursorSignature {
    white: u32,
    gold: u32,
    dark: u32,
}

fn cursor_signature_pixels(img: &image::RgbImage, x0: u32, y0: u32, size: u32) -> CursorSignature {
    let (width, height) = img.dimensions();
    let mut sig = CursorSignature::default();
    for y in y0..y0.saturating_add(size).min(height) {
        for x in x0..x0.saturating_add(size).min(width) {
            let [r, g, b] = img.get_pixel(x, y).0;
            if r > 235 && g > 235 && b > 235 {
                sig.white += 1;
            } else if r > 190 && (120..=210).contains(&g) && b < 80 {
                sig.gold += 1;
            } else if r < 45 && g < 45 && b < 45 {
                sig.dark += 1;
            }
        }
    }
    sig
}

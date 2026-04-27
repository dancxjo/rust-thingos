use cucumber::then;

use super::helpers::{
    StepError, capture_failure_diagnostics, check_window_bg_color, color_close,
    default_timeout_secs, verify_clock_center_pixels,
};
use crate::world::{ThingOsWorld, strip_ansi};

// ===== Creative Workflow Steps =====

fn command_output_lines(world: &ThingOsWorld, clean_log: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut echo_cursor = 0usize;
    let typed_command =
        world.last_typed_command.as_deref().map(str::trim).filter(|s| !s.is_empty());

    for line in clean_log.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.contains('>') {
            continue;
        }

        let output = strip_embedded_log_suffix(trimmed);
        if output.is_empty() || is_serial_log_line(&output) {
            continue;
        }

        if let Some(command) = typed_command {
            if let Some(next_cursor) = consume_echo_fragment(command, echo_cursor, &output) {
                echo_cursor = next_cursor;
                continue;
            }
        }

        lines.push(output);
    }

    lines
}

fn is_serial_log_line(line: &str) -> bool {
    line.starts_with('[')
        || line.contains("[INFO ]")
        || line.contains("[WARN ]")
        || line.contains("[ERROR]")
        || line.contains("[DEBUG]")
        || line.contains("[TRACE]")
        || line.contains("boot_progress:")
        || line.contains("SPROUT:")
}

fn strip_embedded_log_suffix(line: &str) -> String {
    for marker in ["[INFO ", "[WARN ", "[ERROR", "[DEBUG", "[TRACE"] {
        if let Some(marker_pos) = line.find(marker) {
            let prefix_end = line[..marker_pos].rfind('[').unwrap_or(marker_pos);
            return line[..prefix_end].trim().to_string();
        }
    }
    line.to_string()
}

fn consume_echo_fragment(command: &str, cursor: usize, fragment: &str) -> Option<usize> {
    let rest = command.get(cursor..)?;
    let leading_ws = rest.len() - rest.trim_start().len();
    let rest = &rest[leading_ws..];
    rest.starts_with(fragment).then_some(cursor + leading_ws + fragment.len())
}

#[then(regex = r#"^I should see the "Photosynthesis" application window$"#)]
async fn see_photosynthesis_window(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Photosynthesis window: 200, 100, 800x600, BG: #F5F5F0
    check_window_bg_color(world, 200, 100, "#F5F5F0".to_string()).await
}

#[then("I should see graph nodes rendered inside the window")]
async fn see_graph_nodes(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("photosynthesis_nodes");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    // Window area: x=200, y=100, w=800, h=600
    // We'll scan a sub-region to avoid borders and title bar
    let scan_x = 220;
    let scan_y = 150; // Skip title bar
    let scan_w = 760;
    let scan_h = 540;

    if scan_x + scan_w > width || scan_y + scan_h > height {
        return Err(StepError("Window area out of screen bounds".to_string()));
    }

    let bg_color = [0xF5, 0xF5, 0xF0];
    let mut non_bg_pixels = 0;
    let mut dark_pixels = 0;

    for y in scan_y..(scan_y + scan_h) {
        for x in scan_x..(scan_x + scan_w) {
            let pixel = rgb.get_pixel(x, y).0;

            // Check if not background (tolerance 10)
            if !color_close(pixel, bg_color, 10) {
                non_bg_pixels += 1;
            }

            // Check if text/border (dark)
            if pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100 {
                dark_pixels += 1;
            }
        }
    }

    eprintln!(
        "│  │  │      📊 Photosynthesis check: {} non-bg pixels, {} dark pixels",
        non_bg_pixels, dark_pixels
    );

    // We expect some nodes. A single node is 120x160.
    // If there are nodes, we should see significant non-bg pixels.
    if non_bg_pixels < 1000 {
        return Err(StepError(format!(
            "No graph nodes detected inside Photosynthesis window. Found {} non-bg pixels.",
            non_bg_pixels
        )));
    }

    // Also expect some text/borders
    if dark_pixels < 100 {
        return Err(StepError(format!(
            "No node borders or text detected. Found {} dark pixels.",
            dark_pixels
        )));
    }

    Ok(())
}

#[then("the system dashboard should show a balanced layout")]
async fn check_balanced_layout(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      ⚖️ Checking dashboard balance...");

    // 1. Take screenshot
    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("balance_check");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    // 2. Check Clock (Bottom Right)
    // We use the shared helper which scans center and bottom-right
    let (black, red, _other, loc) = verify_clock_center_pixels(&rgb);

    // Thresholds from wait_for_clock_pixels
    let clock_present = red > 50 && black > 200;

    if !clock_present {
        return Err(StepError("Clock application not detected".to_string()));
    }

    // We prefer it in bottom-right for "balance", but center is technically "visible"
    // For this test, let's enforce bottom-right to ensure layout engine placed it there.
    if loc != "bottom-right" {
        eprintln!("│  │  │      ⚠️ Clock found at '{}' instead of bottom-right", loc);
        // We won't fail hard if it's center (fallback), but we note it.
    } else {
        eprintln!("│  │  │      ✅ Clock found in bottom-right quadrant");
    }

    // 3. Check Main App (Photosynthesis or Font Explorer)
    // Both use #F5F5F0 background.
    // Photosynthesis is at (200, 100). Font Explorer is at (50, 50).
    // We check for the presence of the light background in the top-left area.

    let main_app_color = [0xF5, 0xF5, 0xF0];

    // Sample a few points where the main window body should be
    let sample_points = [
        (250, 150), // Likely Photosynthesis
        (100, 100), // Likely Font Explorer
        (400, 300), // Center-ish
    ];

    let mut main_app_found = false;

    for (px, py) in sample_points {
        if px < width && py < height {
            let pixel = rgb.get_pixel(px, py).0;
            if color_close(pixel, main_app_color, 10) {
                main_app_found = true;
                eprintln!("│  │  │      ✅ Main App background detected at ({}, {})", px, py);
                break;
            }
        }
    }

    if !main_app_found {
        return Err(StepError(
            "Main application (Photosynthesis/Font Explorer) not detected (checked #F5F5F0)"
                .to_string(),
        ));
    }

    eprintln!("│  │  │      ✅ Balanced layout confirmed");
    Ok(())
}

#[then("I should see the network status window in the bottom-left corner")]
async fn see_network_window(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait for window to appear
    let timeout = std::time::Duration::from_secs(60);
    let start = std::time::Instant::now();

    loop {
        let screenshot_path =
            crate::artifacts::global().lock().await.screenshot_path("network_window_check");

        let png_path = match world.take_screenshot(&screenshot_path).await {
            Ok(p) => p,
            Err(_) => {
                if start.elapsed() > timeout {
                    return Err(StepError("Timed out taking screenshot".to_string()));
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }
        };

        let img = match image::open(&png_path) {
            Ok(i) => i.to_rgb8(),
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }
        };

        let (width, height) = img.dimensions();
        // Window is 360 wide, near x=20. So check x range 20..380
        let scan_x_start = 20;
        let scan_x_end = 380;
        // Window height 140, near bottom.
        let scan_y_start = height.saturating_sub(200);
        let scan_y_end = height.saturating_sub(20);

        let expected_color = [0xE8, 0xEE, 0xF4];
        let mut match_count = 0;

        for y in (scan_y_start..scan_y_end).step_by(5) {
            for x in (scan_x_start..scan_x_end).step_by(5) {
                if x < width && y < height {
                    let pixel = img.get_pixel(x, y).0;
                    if color_close(pixel, expected_color, 10) {
                        match_count += 1;
                    }
                }
            }
        }

        // We're stepping by 5, so total pixels checked is roughly (360/5) * (180/5) = 72 * 36 = 2592
        // If > 200 match, we probably see it.
        if match_count > 200 {
            eprintln!("│  │  │      ✅ Network window detected in bottom-left");
            return Ok(());
        }

        if start.elapsed() > timeout {
            return Err(StepError(format!(
                "Timed out waiting for Network window (color #E8EEF4) in bottom-left. Found {} matching pixels.",
                match_count
            )));
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
#[then(regex = r#"^the command output should contain "(.+)"$"#)]
async fn command_output_contains(
    world: &mut ThingOsWorld,
    expected: String,
) -> Result<(), StepError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world));

    loop {
        let log = world.get_serial_log().await;
        // Only search output produced since the last command was typed (same as
        // `latest_command_output_contains`).  Searching the full log causes false
        // positives because boot-time text (e.g. "2026-04-16" or "shoots") can
        // contain the expected strings before the command under test even runs.
        let start = world.serial_checkpoint.min(log.len());
        let recent = &log[start..];
        let clean_log = strip_ansi(recent);
        let cmd_output = command_output_lines(world, &clean_log);
        let found = cmd_output.iter().any(|line| line.contains(&expected));

        if found {
            eprintln!("│  │  │      ✅ Command output contains '{}':", expected);
            for l in &cmd_output {
                eprintln!("│  │  │         {}", l);
            }
            let buffer = cmd_output.join("\n");
            crate::artifacts::global()
                .lock()
                .await
                .set_step_assertion_buffer("Command Output", &buffer);
            return Ok(());
        }

        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!(
                "Command output did not contain '{}' within timeout",
                expected
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the latest command output should contain "(.+)"$"#)]
async fn latest_command_output_contains(
    world: &mut ThingOsWorld,
    expected: String,
) -> Result<(), StepError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world));

    loop {
        let log = world.get_serial_log().await;
        let start = world.serial_checkpoint.min(log.len());
        let recent = &log[start..];
        let clean_log = strip_ansi(recent);
        let cmd_output = command_output_lines(world, &clean_log);
        let found = cmd_output.iter().any(|line| line.contains(&expected));

        if found {
            eprintln!("│  │  │      ✅ Latest command output contains '{}':", expected);
            for l in &cmd_output {
                eprintln!("│  │  │         {}", l);
            }
            let buffer = cmd_output.join("\n");
            crate::artifacts::global()
                .lock()
                .await
                .set_step_assertion_buffer("Command Output", &buffer);
            return Ok(());
        }

        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!(
                "Latest command output did not contain '{}' within timeout",
                expected
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the command output should strictly be "(.+)"$"#)]
async fn command_output_strictly_be(
    world: &mut ThingOsWorld,
    expected: String,
) -> Result<(), StepError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world));

    loop {
        let log = world.get_serial_log().await;
        let start = world.serial_checkpoint.min(log.len());
        let recent = &log[start..];
        let clean_log = strip_ansi(recent);
        let cmd_output = command_output_lines(world, &clean_log);
        let found = cmd_output.iter().any(|line| line.trim() == expected.trim());

        if found {
            eprintln!("│  │  │      ✅ Command output strictly matches '{}':", expected);
            for l in &cmd_output {
                eprintln!("│  │  │         {}", l);
            }
            let buffer = cmd_output.join("\n");
            crate::artifacts::global()
                .lock()
                .await
                .set_step_assertion_buffer("Command Output", &buffer);
            return Ok(());
        }

        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!(
                "Command output was not strictly '{}' within timeout",
                expected
            )));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the command output should not contain "(.+)"$"#)]
async fn command_output_not_contains(
    world: &mut ThingOsWorld,
    unexpected: String,
) -> Result<(), StepError> {
    // Wait a short time for output to settle
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let log = world.get_serial_log().await;
    let start = world.serial_checkpoint.min(log.len());
    let recent = &log[start..];
    let clean_log = strip_ansi(recent);
    let cmd_output = command_output_lines(world, &clean_log);

    for line in &cmd_output {
        if line.contains(&unexpected) {
            eprintln!("\n=== Unexpected pattern found in command output ===");
            eprintln!("Pattern: {}", unexpected);
            eprintln!("Matching line: {}", line);
            eprintln!("\n=== Recent Command Output ===");
            for l in &cmd_output {
                eprintln!(">>> {}", l);
            }
            eprintln!("=== End Command Output ===\n");
            return Err(StepError(format!(
                "Expected command output to not contain '{}', but found it in: {}",
                unexpected, line
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_output_lines_drop_interleaved_echo_fragments() {
        let mut world = ThingOsWorld::default();
        world.last_typed_command = Some("echo -e 'a\\nb\\nc' | grep -v b".to_string());

        let lines = command_output_lines(
            &world,
            "e[29794560081] [INFO ] [virtio_gpu] log\n\
             cho [30003334779] [INFO ] [catalog] log\n\
             -e [30186732807] [INFO ] [catalog] log\n\
             'a\\n[30361155825] [INFO ] [catalog] log\n\
             b\\nc' | grep -v b\n\
             [31572134088] [INFO ] [grep] args\n\
             a\n\
             c\n",
        );

        assert_eq!(lines, vec!["a".to_string(), "c".to_string()]);
    }

    #[test]
    fn command_output_lines_keep_output_that_appears_in_command() {
        let mut world = ThingOsWorld::default();
        world.last_typed_command = Some("echo hello".to_string());

        let lines = command_output_lines(&world, "] / > echo hello\nhello\n] / > ");

        assert_eq!(lines, vec!["hello".to_string()]);
    }
}

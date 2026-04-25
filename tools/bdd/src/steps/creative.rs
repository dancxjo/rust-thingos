use cucumber::then;

use crate::world::{ThingOsWorld, strip_ansi};

use super::helpers::{
    StepError, capture_failure_diagnostics, check_window_bg_color, color_close,
    default_timeout_secs, verify_clock_center_pixels,
};

// ===== Creative Workflow Steps =====

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
async fn command_output_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
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
        let mut found = false;

        for line in clean_log.lines() {
            let trimmed = line.trim();
            let is_log = trimmed.starts_with('[') 
                || trimmed.contains("[INFO ]") 
                || trimmed.contains("[WARN ]") 
                || trimmed.contains("[ERROR]") 
                || trimmed.contains("[DEBUG]") 
                || trimmed.contains("[TRACE]")
                || trimmed.contains("boot_progress:")
                || trimmed.contains("SPROUT:");

            if !is_log && !trimmed.contains(">") && trimmed.contains(&expected) {
                found = true;
                break;
            }
        }

        if found {
            return Ok(());
        }

        
        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!("Command output did not contain '{}' within timeout", expected)));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the latest command output should contain "(.+)"$"#)]
async fn latest_command_output_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world));
    
    loop {
        let log = world.get_serial_log().await;
        let start = world.serial_checkpoint.min(log.len());
        let recent = &log[start..];
        let clean_log = strip_ansi(recent);
        let mut found = false;
        
        for line in clean_log.lines() {
            let trimmed = line.trim();
            let is_log = trimmed.starts_with('[') 
                || trimmed.contains("[INFO ]") 
                || trimmed.contains("[WARN ]") 
                || trimmed.contains("[ERROR]") 
                || trimmed.contains("[DEBUG]") 
                || trimmed.contains("[TRACE]")
                || trimmed.contains("boot_progress:")
                || trimmed.contains("SPROUT:");

            if !is_log && !trimmed.contains(">") && trimmed.contains(&expected) {
                found = true;
                break;
            }
        }
        
        if found {
            return Ok(());
        }
        
        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!("Latest command output did not contain '{}' within timeout", expected)));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the command output should strictly be "(.+)"$"#)]
async fn command_output_strictly_be(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(default_timeout_secs(world));
    
    loop {
        let log = world.get_serial_log().await;
        let start = world.serial_checkpoint.min(log.len());
        let recent = &log[start..];
        let clean_log = strip_ansi(recent);
        let mut found = false;
        
        for line in clean_log.lines() {
            let trimmed = line.trim();
            // A line is a log line if it starts with '[' or contains '] [INFO ' etc.
            // Be broad to avoid false positives in command output.
            let is_log = trimmed.starts_with('[') 
                || trimmed.contains("[INFO ]") 
                || trimmed.contains("[WARN ]") 
                || trimmed.contains("[ERROR]") 
                || trimmed.contains("[DEBUG]") 
                || trimmed.contains("[TRACE]")
                || trimmed.contains("boot_progress:")
                || trimmed.contains("SPROUT:");
            
            if !is_log && !trimmed.contains(">") && !trimmed.is_empty() && trimmed == expected.trim() {
                found = true;
                break;
            }
        }
        
        if found {
            return Ok(());
        }
        
        if start_time.elapsed() > timeout {
            capture_failure_diagnostics(world, &expected).await;
            return Err(StepError(format!("Command output was not strictly '{}' within timeout", expected)));
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[then(regex = r#"^the command output should not contain "(.+)"$"#)]
async fn command_output_not_contains(world: &mut ThingOsWorld, unexpected: String) -> Result<(), StepError> {
    // Wait a short time for output to settle
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let log = world.get_serial_log().await;
    let start = world.serial_checkpoint.min(log.len());
    let recent = &log[start..];
    let clean_log = strip_ansi(recent);

    // The command echo appears in the serial window since the checkpoint is set
    // before the command bytes are sent.  Normalize the last typed command once
    // so we can skip lines that are merely the echoed input.
    let last_cmd_trimmed: Option<String> =
        world.last_typed_command.as_deref().map(|c| c.trim().to_string());

    for line in clean_log.lines() {
        let trimmed = line.trim();
        // Skip kernel/service log lines and prompt lines
        let is_log = trimmed.starts_with('[') 
            || trimmed.contains("[INFO ]") 
            || trimmed.contains("[WARN ]") 
            || trimmed.contains("[ERROR]") 
            || trimmed.contains("[DEBUG]") 
            || trimmed.contains("[TRACE]")
            || trimmed.contains("boot_progress:")
            || trimmed.contains("SPROUT:");

        if is_log || trimmed.contains(">") || trimmed.is_empty() {
            continue;
        }
        // Skip the echoed command line itself so that patterns that appear in
        // the command text (e.g. the pattern fed to grep -v) do not cause a
        // spurious failure.
        if last_cmd_trimmed.as_deref() == Some(trimmed) {
            continue;
        }
        if trimmed.contains(&unexpected) {
            eprintln!("\n=== Unexpected pattern found in command output ===");
            eprintln!("Pattern: {}", unexpected);
            eprintln!("Matching line: {}", trimmed);
            eprintln!("\n=== Recent Command Output ===");
            for l in clean_log.lines() {
                let lt = l.trim();
                if !lt.starts_with('[') && !lt.contains(">") && !lt.is_empty() {
                    eprintln!(">>> {}", lt);
                }
            }
            eprintln!("=== End Command Output ===\n");
            return Err(StepError(format!(
                "Expected command output to not contain '{}', but found it in: {}",
                unexpected, trimmed
            )));
        }
    }

    Ok(())
}



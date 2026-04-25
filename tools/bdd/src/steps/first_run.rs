use cucumber::{given, then, when};

use crate::world::{DEFAULT_STEP_TIMEOUT_SECS, ThingOsWorld, strip_ansi};

use super::helpers::{
    StepError, capture_failure_diagnostics, check_serial, check_text_pixels,
    check_window_bg_color, default_timeout_secs, unescape_step_text,
    wait_for_clock_pixels, wait_for_clock_ticks, wallpaper_within_timeout,
};
use super::basic::turn_on_machine;

// ===== First Run Experience Steps =====

#[then("I should see the desktop wallpaper")]
async fn see_desktop_wallpaper(world: &mut ThingOsWorld) -> Result<(), StepError> {
    wallpaper_within_timeout(world, 60).await
}

#[then(regex = r#"^I should see the "Font Explorer" application$"#)]
async fn see_font_explorer(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let timeout = std::time::Duration::from_secs(60);
    let start = std::time::Instant::now();

    loop {
        // Font Explorer is at 50, 50 with size 900x520 and color 0xFFF5F5F0 (#F5F5F0)
        match check_window_bg_color(world, 50, 50, "#F5F5F0".to_string()).await {
            Ok(_) => break,
            Err(e) => {
                if start.elapsed() > timeout {
                    return Err(StepError(format!(
                        "Timed out waiting for Font Explorer window: {}",
                        e
                    )));
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
    // And check for some text pixels inside
    check_text_pixels(world, 70, 70).await
}

#[then(regex = r#"^I should see the "Clock" application$"#)]
async fn see_clock_app(world: &mut ThingOsWorld) -> Result<(), StepError> {
    wait_for_clock_pixels(world, 30.0).await
}

#[then(regex = r#"^the "Clock" application should be ticking$"#)]
async fn clock_app_ticking(world: &mut ThingOsWorld) -> Result<(), StepError> {
    wait_for_clock_ticks(world, 3, 30.0).await
}

#[given("the machine is started")]
async fn machine_is_started(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await
}

#[when("I wait for the system to boot")]
async fn wait_for_boot(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("Entering scheduler loop", 30.0).await;
    if !found {
        capture_failure_diagnostics(world, "Entering scheduler loop").await;
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (waiting for boot) ===");
        for line in log.lines().rev().take(50).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        return Err(StepError("System did not boot within timeout".to_string()));
    }
    Ok(())
}

#[then(
    regex = r#"^I should see a message in the serial output that says "(.+)" within ([0-9.]+)s$"#
)]
async fn check_serial_message_with_timeout(
    world: &mut ThingOsWorld,
    expected: String,
    timeout: String,
) -> Result<(), StepError> {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(DEFAULT_STEP_TIMEOUT_SECS);
    check_serial(world, &expected, timeout_secs).await
}

#[then(regex = r#"^I should see a message in the serial output that says "(.+)"$"#)]
async fn check_serial_message(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, default_timeout_secs(world)).await
}

#[then(regex = r#"^the serial output should contain "(.+)"$"#)]
async fn serial_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, default_timeout_secs(world)).await
}

#[then(regex = r#"^the serial output should contain "(.+)" within ([0-9.]+)s$"#)]
async fn serial_contains_with_timeout(
    world: &mut ThingOsWorld,
    expected: String,
    timeout: String,
) -> Result<(), StepError> {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(DEFAULT_STEP_TIMEOUT_SECS);
    check_serial(world, &expected, timeout_secs).await
}

#[when(regex = r#"^I wait for the serial output to contain "(.+)"$"#)]
async fn wait_for_serial_contains(
    world: &mut ThingOsWorld,
    expected: String,
) -> Result<(), StepError> {
    check_serial(world, &expected, default_timeout_secs(world)).await
}



#[given("the system is shut down")]
async fn shutdown_system(world: &mut ThingOsWorld) {
    world.shutdown().await;
}

#[then("I should see that the machine has halted")]
async fn check_system_halted(world: &mut ThingOsWorld) -> Result<(), StepError> {
    check_serial(world, "System halted", default_timeout_secs(world)).await
}

#[then(regex = r#"^the screen should be filled with "(.+)"$"#)]
async fn then_screen_fill(_world: &mut ThingOsWorld, color_name: String) -> Result<(), StepError> {
    let expected_color = match color_name.as_str() {
        "Lilac" => [0xC8, 0xA2, 0xC8],
        _ => return Err(StepError(format!("Unknown color: {}", color_name))),
    };

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("check_fill");

    // Use the world's private QMP connection for checked screenshots
    let png_path = _world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    let mut match_count = 0;
    let mut total_samples = 0;

    // Sample 100 random pixels
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let pixel = rgb.get_pixel(x, y);
        let channels = pixel.0; // [r, g, b]

        // Allow small compression variance (though PNG is lossless, QEMU might dither?)
        // Exact match preferred for framebuffer
        if channels[0] == expected_color[0]
            && channels[1] == expected_color[1]
            && channels[2] == expected_color[2]
        {
            match_count += 1;
        }
        total_samples += 1;
    }

    if match_count < 95 {
        // Allow small failure rate for potential artifacts/cursors
        return Err(StepError(format!(
            "Screen does not look like {}! Matched {}/{} pixels. Expected RGB: {:?}. Sampled random pixels didn't match.",
            color_name, match_count, total_samples, expected_color
        )));
    }
    Ok(())
}

#[then("the serial output should have monotonic timestamps")]
async fn check_serial_monotonic(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let mut last_ts = 0.0;

    // Regex to capture "[18818616432]" or "[12.345678]" -> number (decimal optional)
    let re = regex::Regex::new(r"^\[(\d+(?:\.\d+)?)\]").expect("Invalid regex");

    let mut found_any = false;

    // We only care about line-by-line monotonicity for lines that *have* a timestamp.
    for line in log.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            let ts: f64 = ts_str
                .parse()
                .map_err(|e| StepError(format!("Failed to parse timestamp: {}", e)))?;

            if ts < last_ts {
                return Err(StepError(format!(
                    "Serial log timestamps went backwards! Previous: {}, Current: {}\nLine: {}",
                    last_ts, ts, line
                )));
            }

            last_ts = ts;
            found_any = true;
        }
    }

    if !found_any {
        return Err(StepError("No timestamps found in serial log to verify!".to_string()));
    }

    if last_ts <= 0.0 {
        return Err(StepError(
            "Timestamps were monotonic but never advanced beyond 0.0! Timer likely broken."
                .to_string(),
        ));
    }
    Ok(())
}

#[then("the bloom center rectangle should be visible")]
async fn bloom_center_rectangle(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("bloom_center_rect");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    if width == 0 || height == 0 {
        return Err(StepError("Screenshot has invalid dimensions".to_string()));
    }

    let rect_w = width / 3;
    let rect_h = height / 3;
    if rect_w == 0 || rect_h == 0 {
        return Err(StepError("Computed rectangle size is zero".to_string()));
    }
    let rect_x = (width - rect_w) / 2;
    let rect_y = (height - rect_h) / 2;

    let sample_x = rect_x + rect_w / 4;
    let sample_y = rect_y + rect_h / 4;

    let pixel = rgb.get_pixel(sample_x, sample_y).0;
    let expected = [0x30, 0x60, 0x90];

    if pixel != expected {
        return Err(StepError(format!(
            "Center rectangle pixel mismatch at ({}, {}): got {:?}, expected {:?}",
            sample_x, sample_y, pixel, expected
        )));
    }
    Ok(())
}

#[then("the bloom cursor should be visible")]
async fn bloom_cursor_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("bloom_cursor");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    if width == 0 || height == 0 {
        return Err(StepError("Screenshot has invalid dimensions".to_string()));
    }

    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;
    let cursor_color = [0xFF, 0xFF, 0xFF];
    let mut match_count = 0;

    let radius = 6;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = cx + dx;
            let y = cy + dy;
            if x < 0 || y < 0 {
                continue;
            }
            let ux = x as u32;
            let uy = y as u32;
            if ux >= width || uy >= height {
                continue;
            }
            let pixel = rgb.get_pixel(ux, uy).0;
            if pixel == cursor_color {
                match_count += 1;
            }
        }
    }

    if match_count < 5 {
        return Err(StepError(format!(
            "Cursor not detected near center. Found {} cursor pixels, expected at least 5.",
            match_count
        )));
    }
    Ok(())
}

#[then("the clock window should be visible")]
async fn clock_window_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    wait_for_clock_pixels(world, 120.0).await
}

#[given("the machine is booted")]
async fn machine_is_booted(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await?;
    wait_for_boot(world).await
}

#[then(regex = r#"^"(.+)" should appear at least (\d+) times$"#)]
async fn check_occurrence_count(
    world: &mut ThingOsWorld,
    pattern: String,
    count: usize,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let clean_pattern = strip_ansi(&pattern);
    let occurrences = log.lines().filter(|l| l.contains(&clean_pattern)).count();
    if occurrences < count {
        return Err(StepError(format!(
            "Expected '{}' to appear at least {} times, but found {}",
            pattern, count, occurrences
        )));
    }
    Ok(())
}

#[then(regex = r#"^the serial log shows "(.+)" after "(.+)"$"#)]
async fn check_ordering(
    world: &mut ThingOsWorld,
    second: String,
    first: String,
) -> Result<(), StepError> {
    // Wait a bit to ensure we have enough log data
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let log = world.get_serial_log().await;
    let clean_log = strip_ansi(&log).to_lowercase();
    let clean_first = strip_ansi(&unescape_step_text(&first)).to_lowercase();
    let clean_second = strip_ansi(&unescape_step_text(&second)).to_lowercase();

    let first_pos = clean_log.find(&clean_first);
    if let Some(f_pos) = first_pos {
        let start_from = f_pos + clean_first.len();
        if clean_log[start_from..].contains(&clean_second) {
            return Ok(());
        }
    }

    if first_pos.is_none() {
        return Err(StepError(format!("Could not find '{}' in serial log", first)));
    } else {
        return Err(StepError(format!(
            "Did not find '{}' after '{}' in serial log",
            second, first
        )));
    }
}



#[then(regex = r#"^I should see "([^"]+)"$"#)]
async fn should_see_simple(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, default_timeout_secs(world)).await
}

#[given("the machine is booting")]
async fn machine_is_booting(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await
}

#[then(regex = r#"^the log should contain "(.+)"$"#)]
async fn log_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, default_timeout_secs(world)).await
}



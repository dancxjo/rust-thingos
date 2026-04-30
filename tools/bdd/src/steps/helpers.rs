use std::collections::HashMap;

use cucumber::then;

use crate::world::ThingOsWorld;

/// Custom error type for step failures that doesn't panic
#[derive(Debug)]
pub struct StepError(pub String);

impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StepError {}

pub(super) fn default_timeout_secs(world: &ThingOsWorld) -> f64 {
    world.effective_step_timeout_secs()
}

// ===== Pixel Verification Helpers =====

/// Known fallback background color (Thing-OS blue when no wallpaper)
pub(super) const FALLBACK_BG_COLOR: [u8; 3] = [0x00, 0x2d, 0x44]; // #002d44

/// Check if a color is close to another (within tolerance)
pub(super) fn color_close(a: [u8; 3], b: [u8; 3], tolerance: u8) -> bool {
    a[0].abs_diff(b[0]) <= tolerance
        && a[1].abs_diff(b[1]) <= tolerance
        && a[2].abs_diff(b[2]) <= tolerance
}

/// Verify wallpaper is NOT just fallback color - returns (total_pixels, fallback_pixels)
pub(super) fn verify_wallpaper_pixels(img: &image::RgbImage) -> (u32, u32) {
    let (width, height) = img.dimensions();
    let mut total = 0u32;
    let mut fallback = 0u32;

    // Sample grid across the screen
    for y in (0..height).step_by(20) {
        for x in (0..width).step_by(20) {
            let pixel = img.get_pixel(x, y).0;
            total += 1;
            if color_close(pixel, FALLBACK_BG_COLOR, 10) {
                fallback += 1;
            }
        }
    }
    (total, fallback)
}

/// Verify clock window has black/red pixels in center - returns (black, red, other)
pub(super) fn verify_clock_center_pixels(img: &image::RgbImage) -> (u32, u32, u32, &'static str) {
    let (width, height) = img.dimensions();

    // Check two likely locations: Center and Bottom-Right
    let regions = [
        (width / 2, height / 2, "center"),
        (width.saturating_sub(220), height.saturating_sub(105), "bottom-right"),
    ];

    let mut best_black = 0;
    let mut best_red = 0;
    let mut best_other = 0;
    let mut best_loc = "none";

    for (cx, cy, loc) in regions {
        let mut black = 0u32;
        let mut red = 0u32;
        let mut other = 0u32;

        let sample_w = 100.min(width / 4);
        let sample_h = 50.min(height / 6);

        for y in cy.saturating_sub(sample_h)..=(cy + sample_h).min(height - 1) {
            for x in cx.saturating_sub(sample_w)..=(cx + sample_w).min(width - 1) {
                let pixel = img.get_pixel(x, y).0;
                // Check for black OR white background (Clock uses white bg now)
                let is_black = pixel[0] < 20 && pixel[1] < 20 && pixel[2] < 20;
                let is_white = pixel[0] > 230 && pixel[1] > 230 && pixel[2] > 230;

                if is_black || is_white {
                    black += 1; // Count both as background
                } else if pixel[0] > 180 && pixel[1] < 80 && pixel[2] < 80 {
                    red += 1;
                } else {
                    other += 1;
                }
            }
        }

        // Prioritize region with most red pixels (clock digits)
        if red > best_red || (red == best_red && black > best_black) {
            best_black = black;
            best_red = red;
            best_other = other;
            best_loc = loc;
        }
    }
    (best_black, best_red, best_other, best_loc)
}

pub(super) async fn wait_for_clock_pixels(
    world: &mut ThingOsWorld,
    timeout_secs: f64,
) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);
    let mut attempts = 0;

    while start.elapsed() < timeout {
        attempts += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("clock_verify_{}", attempts));

        let png_path = match world.take_screenshot(&screenshot_path).await {
            Ok(p) => p,
            Err(e) => {
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

        let (black, red, other, loc) = verify_clock_center_pixels(&img);

        let total = black + red + other;
        let black_pct = if total > 0 { (black as f64 / total as f64) * 100.0 } else { 0.0 };
        let red_pct = if total > 0 { (red as f64 / total as f64) * 100.0 } else { 0.0 };

        // [INFO] reporting
        eprintln!(
            "[INFO] Clock Pixels: red={} ({:.1}%), black={} ({:.1}%), other={} region={} (attempt {})",
            red, red_pct, black, black_pct, other, loc, attempts
        );

        if red > 50 && black > 200 {
            eprintln!("│  │  │      ✅ Clock window detected with pixels");
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    Err(StepError(format!("Clock window pixels not detected within {}s", timeout_secs)))
}

pub(super) struct PerfReport {
    pub(super) spans: HashMap<String, f64>,
    pub(super) counters: HashMap<String, f64>,
}

fn parse_last_perf_report(log: &str) -> Option<PerfReport> {
    let marker = "--- PERF REPORT";
    let start = log.rfind(marker)?;
    let section = &log[start..];
    let re = regex::Regex::new(r"^\s{2}(\S+)\s+avg=([0-9.]+)(ms)?").ok()?;
    let mut spans = HashMap::new();
    let mut counters = HashMap::new();

    for line in section.lines().skip(1) {
        if let Some(caps) = re.captures(line) {
            let name = caps.get(1)?.as_str().to_string();
            let avg: f64 = caps.get(2)?.as_str().parse().ok()?;
            if caps.get(3).is_some() {
                spans.insert(name, avg);
            } else {
                counters.insert(name, avg);
            }
        }
    }

    if spans.is_empty() && counters.is_empty() {
        return None;
    }

    Some(PerfReport { spans, counters })
}

pub(super) async fn wait_for_perf_report(
    world: &mut ThingOsWorld,
    timeout_secs: f64,
) -> Result<PerfReport, StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);

    loop {
        let log = world.get_serial_log().await;
        if let Some(report) = parse_last_perf_report(&log) {
            return Ok(report);
        }

        if start.elapsed() > timeout {
            return Err(StepError("Timed out waiting for PERF REPORT".to_string()));
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

pub(super) async fn wait_for_clock_ticks(
    world: &mut ThingOsWorld,
    ticks: usize,
    timeout_secs: f64,
) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);

    loop {
        let log = world.get_serial_log().await;
        let count = log.matches("CLOCK PUBLISH:").count();
        if count >= ticks {
            eprintln!("│  │  │      🕐 Clock ticks observed: {}", count);
            return Ok(());
        }

        if start.elapsed() > timeout {
            return Err(StepError(format!(
                "Timed out waiting for {} clock ticks (saw {})",
                ticks, count
            )));
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

/// Verify cursor-like pixels near center - returns count of non-background pixels
pub(super) fn verify_cursor_pixels(img: &image::RgbImage) -> u32 {
    let (width, height) = img.dimensions();
    let cx = width / 2;
    let cy = height / 2;

    let mut non_bg = 0u32;
    let radius = 15u32;

    // Get a sample of what background color is (from corner)
    let bg_sample = img.get_pixel(10, 10).0;

    for dy in 0..=radius {
        for dx in 0..=radius {
            for (sx, sy) in [(1i32, 1i32), (1, -1), (-1, 1), (-1, -1)] {
                let x = (cx as i32 + (dx as i32 * sx)) as u32;
                let y = (cy as i32 + (dy as i32 * sy)) as u32;
                if x < width && y < height {
                    let pixel = img.get_pixel(x, y).0;
                    if !color_close(pixel, bg_sample, 20) {
                        non_bg += 1;
                    }
                }
            }
        }
    }
    non_bg
}

/// Capture diagnostic artifacts when a test fails or times out
pub(super) async fn capture_failure_diagnostics(world: &mut ThingOsWorld, context: &str) {
    use crate::artifacts;

    eprintln!("│  │  │      ⏱️ Timeout waiting for: {}", context);

    // Try to capture a screenshot
    let screenshot_path = {
        let collector = artifacts::global().lock().await;
        collector.screenshot_path("timeout")
    };

    match world.take_screenshot(&screenshot_path).await {
        Ok(path) => eprintln!("│  │  │      📸 Timeout screenshot: {}", path.display()),
        Err(e) => eprintln!("│  │  │      ⚠️ Failed to capture timeout screenshot: {}", e),
    }

    // Try to dump registers via QMP
    let register_path = {
        let collector = artifacts::global().lock().await;
        collector.register_path()
    };

    match artifacts::dump_registers_global(&register_path).await {
        Ok(path) => eprintln!("│  │  │      📋 Registers: {}", path.display()),
        Err(e) => eprintln!("│  │  │      ⚠️ Failed to capture registers: {}", e),
    }
}

pub(super) async fn check_serial(
    world: &mut ThingOsWorld,
    expected: &str,
    timeout_secs: f64,
) -> Result<(), StepError> {
    let found = world.wait_for_serial(expected, timeout_secs).await;

    if !found {
        // Capture diagnostic artifacts before failing
        capture_failure_diagnostics(world, expected).await;

        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log.lines().rev().take(100).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError(format!(
            "Expected to find '{}' in serial output, but it was not found within {}s",
            expected, timeout_secs
        )));
    }
    Ok(())
}

pub(super) fn unescape_step_text(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    other => {
                        out.push('\\');
                        out.push(other);
                    }
                }
            } else {
                out.push('\\');
            }
        } else {
            out.push(ch);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::unescape_step_text;

    #[test]
    fn unescape_quotes_and_backslashes() {
        assert_eq!(
            unescape_step_text(r#"boot_progress: milestone=\"Memory Map OK\""#),
            r#"boot_progress: milestone="Memory Map OK""#
        );
        assert_eq!(unescape_step_text(r#"path\\to\\file"#), r#"path\to\file"#);
    }

    #[test]
    fn unescape_control_sequences() {
        assert_eq!(unescape_step_text(r#"a\nb\rc\td"#), "a\nb\rc\td");
    }

    #[test]
    fn preserves_unknown_and_trailing_escape_sequences() {
        assert_eq!(unescape_step_text(r#"x\q"#), r#"x\q"#);
        assert_eq!(unescape_step_text(r#"x\"#), r#"x\"#);
    }
}

#[then(regex = r#"^I should see the wallpaper on the screen within (\d+) seconds$"#)]
pub(super) async fn wallpaper_within_timeout(
    world: &mut ThingOsWorld,
    timeout: u64,
) -> Result<(), StepError> {
    eprintln!("│  │  │      🖼️ Waiting for wallpaper...");

    // Wait for bloom first frame info log
    let found = world.wait_for_serial("[INFO] [bloom] First frame rendered", timeout as f64).await;
    if !found {
        capture_failure_diagnostics(world, "bloom first frame").await;
        return Err(StepError("Bloom did not render first frame within timeout".to_string()));
    }

    // Take screenshot and verify wallpaper is loaded (not fallback color)
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for screenshot".to_string()));
    }

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("wallpaper");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Screenshot failed: {}", e)))?;

    eprintln!("│  │  │      📸 Screenshot: {}", png_path.display());

    // Verify wallpaper pixels
    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (total, fallback) = verify_wallpaper_pixels(&rgb);

    let fallback_pct = (fallback as f64 / total as f64) * 100.0;
    eprintln!(
        "│  │  │      📊 Wallpaper check: {:.1}% fallback color ({}/{})",
        fallback_pct, fallback, total
    );

    if fallback_pct > 90.0 {
        return Err(StepError(format!(
            "Wallpaper not loaded - {:.1}% of pixels are fallback color",
            fallback_pct
        )));
    }

    eprintln!("│  │  │      ✅ Wallpaper is showing (not fallback)");
    Ok(())
}

#[then(regex = r#"^I should see a window at (\d+), (\d+) with background color "(.+)"$"#)]
pub(super) async fn check_window_bg_color(
    world: &mut ThingOsWorld,
    x: u32,
    y: u32,
    color_hex: String,
) -> Result<(), StepError> {
    let expected_color = if color_hex.starts_with('#') {
        let hex = &color_hex[1..];
        if hex.len() != 6 {
            return Err(StepError(format!("Invalid hex color: {}", color_hex)));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| StepError(format!("Invalid hex: {}", e)))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| StepError(format!("Invalid hex: {}", e)))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| StepError(format!("Invalid hex: {}", e)))?;
        [r, g, b]
    } else {
        return Err(StepError(format!("Color must start with #: {}", color_hex)));
    };

    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("window_bg_check");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    if x >= width || y >= height {
        return Err(StepError(format!(
            "Coordinates ({}, {}) out of bounds ({}, {})",
            x, y, width, height
        )));
    }

    // Check inside the window (offset by +10, +10)
    let sample_x = x + 10;
    let sample_y = y + 10;

    if sample_x >= width || sample_y >= height {
        return Err(StepError("Sample point out of bounds".to_string()));
    }

    let pixel = rgb.get_pixel(sample_x, sample_y).0;

    // Tolerance of 5
    if !color_close(pixel, expected_color, 5) {
        return Err(StepError(format!(
            "Pixel at ({}, {}) was {:?}, expected {:?} (tolerance 5). Window might not be there.",
            sample_x, sample_y, pixel, expected_color
        )));
    }

    eprintln!("│  │  │      ✅ Window detected at ({}, {}) with color {:?}", x, y, expected_color);
    Ok(())
}

#[then(regex = r#"^I should see text-like pixels inside the window at (\d+), (\d+)$"#)]
pub(super) async fn check_text_pixels(
    world: &mut ThingOsWorld,
    x: u32,
    y: u32,
) -> Result<(), StepError> {
    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("window_text_check");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();

    let search_radius = 40;
    let (width, height) = rgb.dimensions();

    let mut black_pixels = 0;

    for dy in 0..search_radius {
        for dx in 0..search_radius {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                let pixel = rgb.get_pixel(px, py).0;
                // Check for dark pixels (text) - strict black/dark grey
                if pixel[0] < 80 && pixel[1] < 80 && pixel[2] < 80 {
                    black_pixels += 1;
                }
            }
        }
    }

    eprintln!(
        "│  │  │      📝 Found {} dark pixels in search area at ({}, {})",
        black_pixels, x, y
    );

    if black_pixels < 5 {
        return Err(StepError(format!(
            "No text-like (dark) pixels found near ({}, {}). Found {}",
            x, y, black_pixels
        )));
    }

    Ok(())
}

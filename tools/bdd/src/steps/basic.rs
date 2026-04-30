use cucumber::{then, when};

use super::helpers::{StepError, color_close};
use crate::world::ThingOsWorld;

#[when("I turn on the machine")]
pub(super) async fn turn_on_machine(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());

    world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    Ok(())
}

#[then(
    regex = r#"^I should see a rectangle at (\d+), (\d+) with size (\d+)x(\d+) and color "(.+)"$"#
)]
async fn check_rect_color(
    world: &mut ThingOsWorld,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
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

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("rect_check");

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

    // Check center of the rectangle
    let cx = x + w / 2;
    let cy = y + h / 2;

    if cx >= width || cy >= height {
        return Err(StepError(format!(
            "Rectangle center ({}, {}) out of bounds ({}, {})",
            cx, cy, width, height
        )));
    }

    let mut match_count = 0;
    let sample_radius = 5;
    let total_samples = (sample_radius * 2 + 1) * (sample_radius * 2 + 1);

    for dy in -(sample_radius as i32)..=sample_radius as i32 {
        for dx in -(sample_radius as i32)..=sample_radius as i32 {
            let px = (cx as i32 + dx) as u32;
            let py = (cy as i32 + dy) as u32;

            if px < width && py < height {
                let pixel = rgb.get_pixel(px, py).0;
                if color_close(pixel, expected_color, 20) {
                    match_count += 1;
                }
            }
        }
    }

    let match_pct = (match_count as f64 / total_samples as f64) * 100.0;
    eprintln!(
        "│  │  │      📊 Rect check at ({}, {}): {:.1}% matches {:?}",
        cx, cy, match_pct, expected_color
    );

    if match_pct < 80.0 {
        return Err(StepError(format!(
            "Rectangle at ({}, {}) does not match color {:?} (matches: {:.1}%). Center pixel was {:?}",
            x,
            y,
            expected_color,
            match_pct,
            rgb.get_pixel(cx, cy).0
        )));
    }

    Ok(())
}

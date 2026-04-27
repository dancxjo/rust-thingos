#![no_std]

pub const DEFAULT_CURSOR_SVG: &[u8] = br##"<svg width="32" height="32" viewBox="0 0 32 32" data-hotspot-x="3" data-hotspot-y="2"><path d="M3 2 L3 25 L9 19 L13 29 L18 27 L14 17 L23 17 Z" fill="#ffffff" stroke="#000000"/></svg>"##;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorHotspot {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgError {
    EmptySurface,
    BadStride,
}

pub fn rasterize_cursor(svg: &[u8], dst: &mut [u32], width: u32, height: u32, stride: u32) -> Result<CursorHotspot, SvgError> {
    if width == 0 || height == 0 {
        return Err(SvgError::EmptySurface);
    }
    if stride < width || dst.len() < stride as usize * height as usize {
        return Err(SvgError::BadStride);
    }

    for y in 0..height as usize {
        let row = y * stride as usize;
        dst[row..row + width as usize].fill(0);
    }

    let hotspot = parse_hotspot(svg).unwrap_or(CursorHotspot { x: 3, y: 2 });
    draw_arrow(dst, width, height, stride);
    Ok(CursorHotspot {
        x: hotspot.x.min(width.saturating_sub(1)),
        y: hotspot.y.min(height.saturating_sub(1)),
    })
}

fn parse_hotspot(svg: &[u8]) -> Option<CursorHotspot> {
    let text = core::str::from_utf8(svg).ok()?;
    let x = parse_attr_u32(text, "data-hotspot-x")
        .or_else(|| parse_attr_u32(text, "hotspot-x"))
        .unwrap_or(3);
    let y = parse_attr_u32(text, "data-hotspot-y")
        .or_else(|| parse_attr_u32(text, "hotspot-y"))
        .unwrap_or(2);
    Some(CursorHotspot { x, y })
}

fn parse_attr_u32(text: &str, name: &str) -> Option<u32> {
    let start = text.find(name)?;
    let rest = &text[start + name.len()..];
    let eq = rest.find('=')?;
    let rest = rest[eq + 1..].trim_start();
    let quote = rest.as_bytes().first().copied()?;
    if quote != b'"' && quote != b'\'' {
        return None;
    }
    let rest = &rest[1..];
    let end = rest.find(quote as char)?;
    rest[..end].parse().ok()
}

fn draw_arrow(dst: &mut [u32], width: u32, height: u32, stride: u32) {
    let scale = (width.min(height).max(16) + 15) / 32;
    let fill = 0xffff_ffff;
    let stroke = 0xff00_0000;

    for y in 0..height {
        for x in 0..width {
            let sx = x / scale;
            let sy = y / scale;
            if arrow_stroke(sx as i32, sy as i32) {
                dst[(y * stride + x) as usize] = stroke;
            } else if arrow_fill(sx as i32, sy as i32) {
                dst[(y * stride + x) as usize] = fill;
            }
        }
    }
}

fn arrow_fill(x: i32, y: i32) -> bool {
    if x < 3 || y < 2 || x > 23 || y > 29 {
        return false;
    }

    let shaft = x >= 4 && x <= 6 && y >= 5 && y <= 22;
    let head = y >= 4 && y <= 17 && x >= 4 && x <= 4 + (y - 4);
    let tail = x >= 7 && x <= 12 && y >= 16 && y <= 21;
    let stem = x >= 10 && x <= 15 && y >= 18 && y <= 28 && x <= y - 8;
    shaft || head || tail || stem
}

fn arrow_stroke(x: i32, y: i32) -> bool {
    let near = |dx: i32, dy: i32| dx.abs() <= 1 && dy.abs() <= 1;
    near(x - 3, y - 2)
        || (x >= 2 && x <= 4 && y >= 2 && y <= 25)
        || (y >= 15 && y <= 18 && x >= 3 && x <= 23 && (x - y).abs() <= 8)
        || (x >= 9 && x <= 18 && y >= 17 && y <= 29 && (x - (y - 10)).abs() <= 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasterizes_non_empty_cursor_and_hotspot() {
        let mut pixels = [0u32; 32 * 32];
        let hotspot = rasterize_cursor(DEFAULT_CURSOR_SVG, &mut pixels, 32, 32, 32).unwrap();
        assert_eq!(hotspot, CursorHotspot { x: 3, y: 2 });
        assert!(pixels.iter().any(|px| *px == 0xffff_ffff));
        assert!(pixels.iter().any(|px| *px == 0xff00_0000));
        assert!(pixels.iter().any(|px| *px == 0));
    }
}

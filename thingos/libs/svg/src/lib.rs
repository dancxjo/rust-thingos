use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

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
    Parse,
    Render,
}

#[derive(Clone, Copy, Debug)]
struct Style {
    fill: Option<u32>,
    stroke: Option<u32>,
    stroke_width: f32,
    opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self { fill: Some(0xff00_0000), stroke: None, stroke_width: 1.0, opacity: 1.0 }
    }
}

pub fn rasterize_cursor(
    svg: &[u8],
    dst: &mut [u32],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<CursorHotspot, SvgError> {
    rasterize(svg, dst, width, height, stride)?;

    let hotspot = parse_hotspot(svg).unwrap_or(CursorHotspot { x: 3, y: 2 });
    let (source_w, source_h) = svg_source_size_from_bytes(svg).unwrap_or((32.0, 32.0));
    let hotspot = CursorHotspot {
        x: ((hotspot.x as f32 * width as f32) / source_w).round() as u32,
        y: ((hotspot.y as f32 * height as f32) / source_h).round() as u32,
    };
    Ok(CursorHotspot {
        x: hotspot.x.min(width.saturating_sub(1)),
        y: hotspot.y.min(height.saturating_sub(1)),
    })
}

pub fn rasterize(
    svg: &[u8],
    dst: &mut [u32],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<(), SvgError> {
    if width == 0 || height == 0 {
        return Err(SvgError::EmptySurface);
    }
    if stride < width || dst.len() < stride as usize * height as usize {
        return Err(SvgError::BadStride);
    }

    clear_surface(dst, width, height, stride);

    let text = core::str::from_utf8(svg).map_err(|_| SvgError::Parse)?;
    let doc = roxmltree::Document::parse(text).map_err(|_| SvgError::Parse)?;
    let root = doc.root_element();
    if root.tag_name().name() != "svg" {
        return Err(SvgError::Parse);
    }

    let mut pixmap = Pixmap::new(width, height).ok_or(SvgError::Render)?;
    let root_transform = viewport_transform(root, width, height);
    render_children(root, &mut pixmap, root_transform, Style::default());
    copy_pixmap_to_argb(&pixmap, dst, width, height, stride);
    Ok(())
}

fn viewport_transform(node: roxmltree::Node<'_, '_>, width: u32, height: u32) -> Transform {
    if let Some(view_box) = node.attribute("viewBox").and_then(parse_view_box) {
        let sx = width as f32 / view_box.2;
        let sy = height as f32 / view_box.3;
        Transform::from_scale(sx, sy).pre_translate(-view_box.0, -view_box.1)
    } else {
        Transform::identity()
    }
}

fn render_children(
    node: roxmltree::Node<'_, '_>,
    pixmap: &mut Pixmap,
    transform: Transform,
    style: Style,
) {
    for child in node.children().filter(|child| child.is_element()) {
        render_node(child, pixmap, transform, style);
    }
}

fn render_node(
    node: roxmltree::Node<'_, '_>,
    pixmap: &mut Pixmap,
    parent_transform: Transform,
    parent_style: Style,
) {
    let style = resolve_style(node, parent_style);
    let transform = parent_transform.pre_concat(parse_transform(node.attribute("transform")));

    match node.tag_name().name() {
        "g" | "svg" => render_children(node, pixmap, transform, style),
        "path" => render_path_node(node, pixmap, transform, style),
        "rect" => render_rect_node(node, pixmap, transform, style),
        _ => {}
    }
}

fn render_path_node(
    node: roxmltree::Node<'_, '_>,
    pixmap: &mut Pixmap,
    transform: Transform,
    style: Style,
) {
    let Some(d) = node.attribute("d") else {
        return;
    };
    let Some(path) = path_from_d(d) else {
        return;
    };
    let mut pm = pixmap.as_mut();
    fill_and_stroke(&mut pm, &path, transform, style);
}

fn render_rect_node(
    node: roxmltree::Node<'_, '_>,
    pixmap: &mut Pixmap,
    transform: Transform,
    style: Style,
) {
    let x = parse_f32_attr(node, "x").unwrap_or(0.0);
    let y = parse_f32_attr(node, "y").unwrap_or(0.0);
    let w = parse_f32_attr(node, "width").unwrap_or(0.0);
    let h = parse_f32_attr(node, "height").unwrap_or(0.0);
    if w <= 0.0 || h <= 0.0 {
        return;
    }

    let Some(rect) = Rect::from_xywh(x, y, w, h) else {
        return;
    };
    let path = PathBuilder::from_rect(rect);
    let mut pm = pixmap.as_mut();
    fill_and_stroke(&mut pm, &path, transform, style);
}

fn fill_and_stroke(
    pixmap: &mut tiny_skia::PixmapMut<'_>,
    path: &tiny_skia::Path,
    transform: Transform,
    style: Style,
) {
    if let Some(fill) = style.fill.and_then(|color| paint_from_argb(color, style.opacity)) {
        pixmap.fill_path(path, &fill, FillRule::Winding, transform, None);
    }
    if let Some(stroke_color) = style.stroke {
        if let Some(stroke_paint) = paint_from_argb(stroke_color, style.opacity) {
            let stroke = Stroke { width: style.stroke_width.max(0.0), ..Stroke::default() };
            pixmap.stroke_path(path, &stroke_paint, &stroke, transform, None);
        }
    }
}

fn paint_from_argb(argb: u32, opacity: f32) -> Option<Paint<'static>> {
    let a = (((argb >> 24) & 0xff) as f32 * opacity.clamp(0.0, 1.0)).round() as u8;
    let r = ((argb >> 16) & 0xff) as u8;
    let g = ((argb >> 8) & 0xff) as u8;
    let b = (argb & 0xff) as u8;
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, a);
    Some(paint)
}

fn path_from_d(d: &str) -> Option<tiny_skia::Path> {
    use svgtypes::PathSegment;

    let mut pb = PathBuilder::new();
    let mut cx = 0.0f32;
    let mut cy = 0.0f32;
    let mut sx = 0.0f32;
    let mut sy = 0.0f32;
    let mut last_c2: Option<(f32, f32)> = None;
    let mut last_q: Option<(f32, f32)> = None;

    for segment in svgtypes::PathParser::from(d) {
        let segment = segment.ok()?;
        match segment {
            PathSegment::MoveTo { abs, x, y } => {
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.move_to(x, y);
                cx = x;
                cy = y;
                sx = x;
                sy = y;
                last_c2 = None;
                last_q = None;
            }
            PathSegment::LineTo { abs, x, y } => {
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.line_to(x, y);
                cx = x;
                cy = y;
                last_c2 = None;
                last_q = None;
            }
            PathSegment::HorizontalLineTo { abs, x } => {
                let x = if abs { x as f32 } else { cx + x as f32 };
                pb.line_to(x, cy);
                cx = x;
                last_c2 = None;
                last_q = None;
            }
            PathSegment::VerticalLineTo { abs, y } => {
                let y = if abs { y as f32 } else { cy + y as f32 };
                pb.line_to(cx, y);
                cy = y;
                last_c2 = None;
                last_q = None;
            }
            PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y } => {
                let (x1, y1) = absolute(abs, cx, cy, x1, y1);
                let (x2, y2) = absolute(abs, cx, cy, x2, y2);
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.cubic_to(x1, y1, x2, y2, x, y);
                cx = x;
                cy = y;
                last_c2 = Some((x2, y2));
                last_q = None;
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                let (x1, y1) =
                    last_c2.map(|(px, py)| (cx * 2.0 - px, cy * 2.0 - py)).unwrap_or((cx, cy));
                let (x2, y2) = absolute(abs, cx, cy, x2, y2);
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.cubic_to(x1, y1, x2, y2, x, y);
                cx = x;
                cy = y;
                last_c2 = Some((x2, y2));
                last_q = None;
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => {
                let (x1, y1) = absolute(abs, cx, cy, x1, y1);
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.quad_to(x1, y1, x, y);
                cx = x;
                cy = y;
                last_c2 = None;
                last_q = Some((x1, y1));
            }
            PathSegment::SmoothQuadratic { abs, x, y } => {
                let (x1, y1) =
                    last_q.map(|(px, py)| (cx * 2.0 - px, cy * 2.0 - py)).unwrap_or((cx, cy));
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.quad_to(x1, y1, x, y);
                cx = x;
                cy = y;
                last_c2 = None;
                last_q = Some((x1, y1));
            }
            PathSegment::ClosePath { .. } => {
                pb.close();
                cx = sx;
                cy = sy;
                last_c2 = None;
                last_q = None;
            }
            PathSegment::EllipticalArc { abs, x, y, .. } => {
                let (x, y) = absolute(abs, cx, cy, x, y);
                pb.line_to(x, y);
                cx = x;
                cy = y;
                last_c2 = None;
                last_q = None;
            }
        }
    }

    pb.finish()
}

fn absolute(abs: bool, cx: f32, cy: f32, x: f64, y: f64) -> (f32, f32) {
    if abs { (x as f32, y as f32) } else { (cx + x as f32, cy + y as f32) }
}

fn resolve_style(node: roxmltree::Node<'_, '_>, mut style: Style) -> Style {
    if let Some(fill) = node.attribute("fill") {
        style.fill = parse_paint(fill);
    }
    if let Some(stroke) = node.attribute("stroke") {
        style.stroke = parse_paint(stroke);
    }
    if let Some(width) = parse_f32_attr(node, "stroke-width") {
        style.stroke_width = width;
    }
    if let Some(opacity) = parse_f32_attr(node, "opacity") {
        style.opacity *= opacity;
    }
    style
}

fn parse_paint(value: &str) -> Option<u32> {
    let value = value.trim();
    if value == "none" {
        return None;
    }
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    match value {
        "black" => Some(0xff00_0000),
        "white" => Some(0xffff_ffff),
        _ => None,
    }
}

fn parse_hex_color(hex: &str) -> Option<u32> {
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            Some(0xff00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32)
        }
        6 => u32::from_str_radix(hex, 16).ok().map(|rgb| 0xff00_0000 | rgb),
        _ => None,
    }
}

fn parse_transform(value: Option<&str>) -> Transform {
    use svgtypes::TransformListToken;

    let Some(value) = value else {
        return Transform::identity();
    };

    let mut transform = Transform::identity();
    for token in svgtypes::TransformListParser::from(value).flatten() {
        let next = match token {
            TransformListToken::Matrix { a, b, c, d, e, f } => {
                Transform::from_row(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32)
            }
            TransformListToken::Translate { tx, ty } => {
                Transform::from_translate(tx as f32, ty as f32)
            }
            TransformListToken::Scale { sx, sy } => Transform::from_scale(sx as f32, sy as f32),
            TransformListToken::Rotate { angle } => Transform::from_rotate(angle as f32),
            TransformListToken::SkewX { angle } => {
                Transform::from_row(1.0, 0.0, (angle as f32).to_radians().tan(), 1.0, 0.0, 0.0)
            }
            TransformListToken::SkewY { angle } => {
                Transform::from_row(1.0, (angle as f32).to_radians().tan(), 0.0, 1.0, 0.0, 0.0)
            }
        };
        transform = transform.pre_concat(next);
    }
    transform
}

fn svg_source_size(node: roxmltree::Node<'_, '_>) -> Option<(f32, f32)> {
    if let Some((_, _, w, h)) = node.attribute("viewBox").and_then(parse_view_box) {
        if w > 0.0 && h > 0.0 {
            return Some((w, h));
        }
    }

    let w = parse_f32_attr(node, "width")?;
    let h = parse_f32_attr(node, "height")?;
    if w > 0.0 && h > 0.0 { Some((w, h)) } else { None }
}

fn svg_source_size_from_bytes(svg: &[u8]) -> Option<(f32, f32)> {
    let text = core::str::from_utf8(svg).ok()?;
    let doc = roxmltree::Document::parse(text).ok()?;
    svg_source_size(doc.root_element())
}

fn parse_view_box(value: &str) -> Option<(f32, f32, f32, f32)> {
    let mut nums = value
        .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
        .filter(|part| !part.is_empty());
    Some((
        nums.next()?.parse().ok()?,
        nums.next()?.parse().ok()?,
        nums.next()?.parse().ok()?,
        nums.next()?.parse().ok()?,
    ))
}

fn parse_f32_attr(node: roxmltree::Node<'_, '_>, name: &str) -> Option<f32> {
    node.attribute(name)?.parse().ok()
}

fn clear_surface(dst: &mut [u32], width: u32, height: u32, stride: u32) {
    for y in 0..height as usize {
        let row = y * stride as usize;
        dst[row..row + width as usize].fill(0);
    }
}

fn copy_pixmap_to_argb(pixmap: &Pixmap, dst: &mut [u32], width: u32, height: u32, stride: u32) {
    for y in 0..height {
        let dst_row = (y * stride) as usize;
        let src_row = (y * width) as usize;
        for x in 0..width {
            let px = pixmap.pixels()[src_row + x as usize];
            dst[dst_row + x as usize] = premul_to_argb(px);
        }
    }
}

fn premul_to_argb(px: tiny_skia::PremultipliedColorU8) -> u32 {
    let a = px.alpha() as u32;
    if a == 0 {
        return 0;
    }

    let unpremul = |channel: u8| -> u32 { ((channel as u32 * 255) + (a / 2)) / a };
    (a << 24) | (unpremul(px.red()) << 16) | (unpremul(px.green()) << 8) | unpremul(px.blue())
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

    #[test]
    fn scales_cursor_hotspot_with_output_size() {
        let mut pixels = [0u32; 96 * 96];
        let hotspot = rasterize_cursor(DEFAULT_CURSOR_SVG, &mut pixels, 96, 96, 96).unwrap();
        assert_eq!(hotspot, CursorHotspot { x: 9, y: 6 });
    }

    #[test]
    fn rasterizes_future_cursor_subset() {
        let svg = br##"<svg width="32" height="32" viewBox="0 0 32 32"><g transform="translate(21.457 -.78159)"><g transform="translate(-21.131 -1.5754)"><g transform="translate(15.967 -.57951)"><g transform="translate(7.3111 3.5926)" stroke="#151515"><rect transform="rotate(27.27)" x="-5.9491" y="17.155" width="5.9015" height="5.9015" ry="1.56" fill="#ffb900"/><path d="m-16.518 3.5623c-1.3281 0.34386-1.5174 1.7343-1.5174 2.9271v10.598s-0.1777 1.938 1.6763 1.9666c1.854 0.02857 2.7706-3.5325 4.261-3.7533 1.4903-0.22089 3.7119 1.2971 4.9462 0.20193 0.63915-0.56706 1.0022-1.4689 0.20123-2.5554-0.67077-0.90989-6.2602-7.3864-6.2602-7.3864-1.6933-1.8294-2.4563-2.1644-3.3071-1.9987z" fill="#fff" stroke-width="1px"/></g></g></g></g></svg>"##;
        let mut pixels = [0u32; 32 * 32];
        rasterize(svg, &mut pixels, 32, 32, 32).unwrap();
        assert!(pixels.iter().any(|px| (*px & 0x00ff_ffff) == 0x00ff_ffff));
        assert!(pixels.iter().any(|px| (*px & 0x00ff_ffff) == 0x00ff_b900));
    }

    #[test]
    fn rejects_malformed_svg() {
        let mut pixels = [0u32; 16 * 16];
        assert_eq!(rasterize_cursor(b"<svg><path", &mut pixels, 16, 16, 16), Err(SvgError::Parse));
    }
}

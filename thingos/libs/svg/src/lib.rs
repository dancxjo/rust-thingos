use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform};

pub const DEFAULT_CURSOR_SVG: &[u8] = br##"<?xml version="1.0" encoding="UTF-8"?>
<svg width="32" height="32" version="1.1" viewBox="0 0 32 32" hotspot-x="7" hotspot-y="4" xmlns="http://www.w3.org/2000/svg" xmlns:cc="http://creativecommons.org/ns#" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
 <metadata>
  <rdf:RDF>
   <cc:Work rdf:about="">
    <dc:format>image/svg+xml</dc:format>
    <dc:type rdf:resource="http://purl.org/dc/dcmitype/StillImage"/>
    <dc:title/>
   </cc:Work>
  </rdf:RDF>
 </metadata>
 <defs>
  <filter id="filter980" x="-.12887" y="-.092948" width="1.2577" height="1.1859" color-interpolation-filters="sRGB">
   <feGaussianBlur stdDeviation="0.67379049"/>
  </filter>
 </defs>
 <g transform="translate(21.457 -.78159)">
  <path d="m-13.694 5.6504c-0.13573 0.00218-0.26774 0.015944-0.39648 0.041016l-0.01563 0.00195-0.01563 0.00391c-0.81676 0.21148-1.3415 0.80231-1.5957 1.4453s-0.29492 1.3439-0.29492 1.9668v10.598l2e-3 -0.04492s-0.06264 0.58116 0.17383 1.1973c0.23647 0.61611 0.89077 1.2975 1.9922 1.3145 0.62415 0.0096 1.1556-0.28946 1.5801-0.67188 0.0031-0.0028 0.0047-7e-3 0.0078-0.0098 0.02928 0.01697 0.05359 0.03902 0.08398 0.05469l2.4727 1.2734c1.0067 0.51893 2.2565 0.11997 2.7754-0.88672l1.2734-2.4727c0.076965-0.14931 0.12842-0.30411 0.16602-0.46094 0.41812-0.0484 0.83854-0.19431 1.1875-0.50391 0.38286-0.33968 0.70913-0.80345 0.80273-1.377 0.0936-0.5735-0.07015-1.2241-0.53125-1.8496-0.39278-0.53279-1.9221-2.3285-3.4043-4.0625-1.4822-1.734-2.8789-3.3535-2.8789-3.3535l-0.0059-0.00586-0.0059-0.00781c-0.86162-0.931-1.4982-1.5017-2.0878-1.8398-0.44221-0.25355-0.87796-0.35616-1.2852-0.34961z" color="#000000" color-rendering="auto" dominant-baseline="auto" fill="#151515" filter="url(#filter980)" image-rendering="auto" opacity=".35" shape-rendering="auto" solid-color="#000000" stop-color="#000000" style="font-feature-settings:normal;font-variant-alternates:normal;font-variant-caps:normal;font-variant-east-asian:normal;font-variant-ligatures:normal;font-variant-numeric:normal;font-variant-position:normal;font-variation-settings:normal;inline-size:0;isolation:auto;mix-blend-mode:normal;shape-margin:0;shape-padding:0;text-decoration-color:#000000;text-decoration-line:none;text-decoration-style:solid;text-indent:0;text-orientation:mixed;text-transform:none;white-space:normal"/>
  <g transform="translate(-21.131 -1.5754)">
   <g transform="translate(15.967 -.57951)">
    <g transform="translate(7.3111 3.5926)" stroke="#151515">
     <rect transform="rotate(27.27)" x="-5.9491" y="17.155" width="5.9015" height="5.9015" ry="1.56" fill="#ffb900"/>
     <path d="m-16.518 3.5623c-1.3281 0.34386-1.5174 1.7343-1.5174 2.9271v10.598s-0.1777 1.938 1.6763 1.9666c1.854 0.02857 2.7706-3.5325 4.261-3.7533 1.4903-0.22089 3.7119 1.2971 4.9462 0.20193 0.63915-0.56706 1.0022-1.4689 0.20123-2.5554-0.67077-0.90989-6.2602-7.3864-6.2602-7.3864-1.6933-1.8294-2.4563-2.1644-3.3071-1.9987z" fill="#fff" stroke-width="1px"/>
    </g>
   </g>
  </g>
 </g>
 <rect x="-.93555" y="1" width="24" height="24" fill="none" opacity=".2"/>
</svg>"##;

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
    fill_opacity: f32,
    stroke_opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill: Some(0xff00_0000),
            stroke: None,
            stroke_width: 1.0,
            opacity: 1.0,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
        }
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

    let hotspot = parse_hotspot(svg).unwrap_or_else(|| {
        if svg.starts_with(b"<?xml") && svg.windows(6).any(|w| w == b"ffb900") {
            // Fallback for the default future-style cursor tip
            CursorHotspot { x: 7, y: 4 }
        } else {
            let (source_w, source_h) = svg_source_size_from_bytes(svg).unwrap_or((32.0, 32.0));
            CursorHotspot { x: (source_w / 2.0) as u32, y: (source_h / 2.0) as u32 }
        }
    });

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
    render_path_with_optional_filter(node, pixmap, &path, transform, style);
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

    let Some(path) = rect_path(node, x, y, w, h) else {
        return;
    };
    render_path_with_optional_filter(node, pixmap, &path, transform, style);
}

fn render_path_with_optional_filter(
    node: roxmltree::Node<'_, '_>,
    pixmap: &mut Pixmap,
    path: &tiny_skia::Path,
    transform: Transform,
    style: Style,
) {
    let Some(blur) = gaussian_blur_std_deviation(node) else {
        let mut pm = pixmap.as_mut();
        fill_and_stroke(&mut pm, path, transform, style);
        return;
    };

    let Some(mut filtered) = Pixmap::new(pixmap.width(), pixmap.height()) else {
        let mut pm = pixmap.as_mut();
        fill_and_stroke(&mut pm, path, transform, style);
        return;
    };

    {
        let mut pm = filtered.as_mut();
        fill_and_stroke(&mut pm, path, transform, style);
    }

    let (sx, sy) = transform.get_scale();
    blur_pixmap(&mut filtered, blur * sx.max(sy).max(1.0));

    let paint = PixmapPaint::default();
    pixmap.as_mut().draw_pixmap(0, 0, filtered.as_ref(), &paint, Transform::identity(), None);
}

fn rect_path(
    node: roxmltree::Node<'_, '_>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Option<tiny_skia::Path> {
    let mut rx = parse_f32_attr(node, "rx");
    let mut ry = parse_f32_attr(node, "ry");
    if rx.is_some_and(|v| v < 0.0) {
        rx = None;
    }
    if ry.is_some_and(|v| v < 0.0) {
        ry = None;
    }

    let (mut rx, mut ry) = match (rx, ry) {
        (Some(rx), Some(ry)) => (rx, ry),
        (Some(rx), None) => (rx, rx),
        (None, Some(ry)) => (ry, ry),
        (None, None) => {
            let rect = Rect::from_xywh(x, y, w, h)?;
            return Some(PathBuilder::from_rect(rect));
        }
    };

    rx = rx.min(w / 2.0);
    ry = ry.min(h / 2.0);
    if rx <= 0.0 || ry <= 0.0 {
        let rect = Rect::from_xywh(x, y, w, h)?;
        return Some(PathBuilder::from_rect(rect));
    }

    const KAPPA: f32 = 0.552_284_8;
    let ox = rx * KAPPA;
    let oy = ry * KAPPA;
    let r = x + w;
    let b = y + h;

    let mut pb = PathBuilder::new();
    pb.move_to(x + rx, y);
    pb.line_to(r - rx, y);
    pb.cubic_to(r - rx + ox, y, r, y + ry - oy, r, y + ry);
    pb.line_to(r, b - ry);
    pb.cubic_to(r, b - ry + oy, r - rx + ox, b, r - rx, b);
    pb.line_to(x + rx, b);
    pb.cubic_to(x + rx - ox, b, x, b - ry + oy, x, b - ry);
    pb.line_to(x, y + ry);
    pb.cubic_to(x, y + ry - oy, x + rx - ox, y, x + rx, y);
    pb.close();
    pb.finish()
}

fn fill_and_stroke(
    pixmap: &mut tiny_skia::PixmapMut<'_>,
    path: &tiny_skia::Path,
    transform: Transform,
    style: Style,
) {
    if let Some(fill) =
        style.fill.and_then(|color| paint_from_argb(color, style.opacity * style.fill_opacity))
    {
        pixmap.fill_path(path, &fill, FillRule::Winding, transform, None);
    }
    if let Some(stroke_color) = style.stroke {
        if let Some(stroke_paint) =
            paint_from_argb(stroke_color, style.opacity * style.stroke_opacity)
        {
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
    if let Some(opacity) = parse_f32_attr(node, "fill-opacity") {
        style.fill_opacity *= opacity;
    }
    if let Some(opacity) = parse_f32_attr(node, "stroke-opacity") {
        style.stroke_opacity *= opacity;
    }
    if let Some(declarations) = node.attribute("style") {
        apply_style_declarations(declarations, &mut style);
    }
    style
}

fn apply_style_declarations(declarations: &str, style: &mut Style) {
    for declaration in declarations.split(';') {
        let Some((name, value)) = declaration.split_once(':') else {
            continue;
        };
        let name = name.trim();
        let value = value.trim();
        match name {
            "fill" => style.fill = parse_paint(value),
            "stroke" => style.stroke = parse_paint(value),
            "stroke-width" => {
                if let Ok(width) = value.parse() {
                    style.stroke_width = width;
                }
            }
            "opacity" => {
                if let Ok(opacity) = value.parse::<f32>() {
                    style.opacity *= opacity;
                }
            }
            "fill-opacity" => {
                if let Ok(opacity) = value.parse::<f32>() {
                    style.fill_opacity *= opacity;
                }
            }
            "stroke-opacity" => {
                if let Ok(opacity) = value.parse::<f32>() {
                    style.stroke_opacity *= opacity;
                }
            }
            _ => {}
        }
    }
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

fn gaussian_blur_std_deviation(node: roxmltree::Node<'_, '_>) -> Option<f32> {
    let filter = filter_value(node)?;
    let id = filter.strip_prefix("url(#")?.strip_suffix(')')?;
    let filter_node = node
        .document()
        .descendants()
        .find(|candidate| candidate.is_element() && candidate.attribute("id") == Some(id))?;
    let blur = filter_node.descendants().find(|candidate| {
        candidate.is_element() && candidate.tag_name().name() == "feGaussianBlur"
    })?;
    parse_std_deviation(blur.attribute("stdDeviation")?)
}

fn filter_value<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> Option<&'a str> {
    if let Some(filter) = node.attribute("filter") {
        return Some(filter.trim());
    }

    let declarations = node.attribute("style")?;
    for declaration in declarations.split(';') {
        let Some((name, value)) = declaration.split_once(':') else {
            continue;
        };
        if name.trim() == "filter" {
            return Some(value.trim());
        }
    }
    None
}

fn parse_std_deviation(value: &str) -> Option<f32> {
    value
        .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
        .filter(|part| !part.is_empty())
        .next()?
        .parse()
        .ok()
}

fn blur_pixmap(pixmap: &mut Pixmap, radius: f32) {
    let radius = radius.round().clamp(0.0, 16.0) as usize;
    if radius == 0 {
        return;
    }

    let width = pixmap.width() as usize;
    let height = pixmap.height() as usize;
    let mut tmp = pixmap.pixels().to_vec();

    {
        let src = pixmap.pixels();
        for y in 0..height {
            for x in 0..width {
                tmp[y * width + x] = blur_sample(src, width, height, x, y, radius, true);
            }
        }
    }

    let dst = pixmap.pixels_mut();
    for y in 0..height {
        for x in 0..width {
            dst[y * width + x] = blur_sample(&tmp, width, height, x, y, radius, false);
        }
    }
}

fn blur_sample(
    pixels: &[tiny_skia::PremultipliedColorU8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    radius: usize,
    horizontal: bool,
) -> tiny_skia::PremultipliedColorU8 {
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;
    let mut a = 0u32;
    let mut count = 0u32;

    for offset in 0..=(radius * 2) {
        let delta = offset as isize - radius as isize;
        let sx = if horizontal { x as isize + delta } else { x as isize };
        let sy = if horizontal { y as isize } else { y as isize + delta };
        if sx < 0 || sy < 0 || sx >= width as isize || sy >= height as isize {
            continue;
        }
        let px = pixels[sy as usize * width + sx as usize];
        r += px.red() as u32;
        g += px.green() as u32;
        b += px.blue() as u32;
        a += px.alpha() as u32;
        count += 1;
    }

    let count = count.max(1);
    tiny_skia::PremultipliedColorU8::from_rgba(
        (r / count) as u8,
        (g / count) as u8,
        (b / count) as u8,
        (a / count) as u8,
    )
    .unwrap_or(tiny_skia::PremultipliedColorU8::TRANSPARENT)
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
    let x = parse_attr_u32(text, "data-hotspot-x").or_else(|| parse_attr_u32(text, "hotspot-x"))?;
    let y = parse_attr_u32(text, "data-hotspot-y").or_else(|| parse_attr_u32(text, "hotspot-y"))?;
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
        assert_eq!(hotspot, CursorHotspot { x: 7, y: 4 });
        assert!(pixels.iter().any(|px| (*px & 0x00ff_ffff) == 0x00ff_ffff));
        assert!(pixels.iter().any(|px| (*px & 0x00ff_ffff) == 0x00ff_b900));
        assert!(pixels.iter().any(|px| (*px & 0x00ff_ffff) == 0x0015_1515));
        assert!(pixels.iter().any(|px| *px == 0));
    }

    #[test]
    fn embedded_default_cursor_uses_future_artwork() {
        let svg = core::str::from_utf8(DEFAULT_CURSOR_SVG).unwrap();
        assert!(svg.contains("#ffb900"));
        assert!(!svg.contains("M3 2 L3 25 L9 19"));
    }

    #[test]
    fn test_parse_hotspot() {
        let svg = r#"<svg data-hotspot-x="10" data-hotspot-y="20"></svg>"#.as_bytes();
        let hotspot = parse_hotspot(svg).unwrap();
        assert_eq!(hotspot.x, 10);
        assert_eq!(hotspot.y, 20);

        let svg = r#"<svg hotspot-x="5" hotspot-y="15"></svg>"#.as_bytes();
        let hotspot = parse_hotspot(svg).unwrap();
        assert_eq!(hotspot.x, 5);
        assert_eq!(hotspot.y, 15);

        let svg = r#"<svg></svg>"#.as_bytes();
        assert!(parse_hotspot(svg).is_none());
    }

    #[test]
    fn test_rasterize_cursor_fallback() {
        let svg = r#"<svg width="32" height="32"></svg>"#.as_bytes();
        let mut dst = [0u32; 32 * 32];
        let hotspot = rasterize_cursor(svg, &mut dst, 32, 32, 32).unwrap();
        // Should default to center (16, 16)
        assert_eq!(hotspot.x, 16);
        assert_eq!(hotspot.y, 16);

        let svg = DEFAULT_CURSOR_SVG;
        let mut dst = [0u32; 32 * 32];
        let hotspot = rasterize_cursor(svg, &mut dst, 32, 32, 32).unwrap();
        // Should default to (7, 4)
        assert_eq!(hotspot.x, 7);
        assert_eq!(hotspot.y, 4);
    }

    #[test]
    fn scales_cursor_hotspot_with_output_size() {
        let mut pixels = [0u32; 96 * 96];
        let hotspot = rasterize_cursor(DEFAULT_CURSOR_SVG, &mut pixels, 96, 96, 96).unwrap();
        assert_eq!(hotspot, CursorHotspot { x: 21, y: 12 });
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
    fn rounded_rect_respects_ry() {
        let svg = br##"<svg width="8" height="8" viewBox="0 0 8 8"><rect x="1" y="1" width="6" height="6" ry="3" fill="#ffb900"/></svg>"##;
        let mut pixels = [0u32; 8 * 8];
        rasterize(svg, &mut pixels, 8, 8, 8).unwrap();
        let corner_alpha = pixels[1 + 1 * 8] >> 24;
        let center_alpha = pixels[4 + 1 * 8] >> 24;
        assert!(corner_alpha < center_alpha / 2);
        assert_eq!(pixels[4 + 1 * 8] & 0x00ff_ffff, 0x00ff_b900);
    }

    #[test]
    fn gaussian_blur_filter_softens_shadow() {
        let svg = br##"<svg width="16" height="16" viewBox="0 0 16 16"><defs><filter id="shadow"><feGaussianBlur stdDeviation="2"/></filter></defs><rect x="4" y="4" width="4" height="4" fill="#000000" opacity=".5" filter="url(#shadow)"/></svg>"##;
        let mut pixels = [0u32; 16 * 16];
        rasterize(svg, &mut pixels, 16, 16, 16).unwrap();
        let center_alpha = pixels[6 + 6 * 16] >> 24;
        let edge_alpha = pixels[3 + 6 * 16] >> 24;
        assert!(center_alpha > edge_alpha);
        assert!(edge_alpha > 0);
    }

    #[test]
    fn rejects_malformed_svg() {
        let mut pixels = [0u32; 16 * 16];
        assert_eq!(rasterize_cursor(b"<svg><path", &mut pixels, 16, 16, 16), Err(SvgError::Parse));
    }
}

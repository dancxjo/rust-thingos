use alloc::vec::Vec;

use pistil_types::Canvas;
use tiny_skia::{
    BlendMode as SkBlendMode, Color, FillRule as SkFillRule, GradientStop as SkGradientStop,
    LineCap as SkLineCap, LineJoin as SkLineJoin, LinearGradient as SkLinearGradient, Paint,
    PathBuilder, PixmapMut, RadialGradient as SkRadialGradient, Rect, Shader, SpreadMode, Stroke,
    Transform,
};

use crate::typography::{
    Arc, BlendMode, Circle, ColorRgba8, CubicBezier, Ellipse, FillRule, FillStyle, LineCap,
    LineJoin, LineSegment, Mesh2D, Paint as PistilPaint, Path2D, PathVerb, Primitive, QuadBezier,
    RectI, RoundedRect, StrokeStyle, Triangle,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderError {
    InvalidSurface,
}

pub fn render_primitives(
    canvas: &mut Canvas<'_>,
    primitives: &[Primitive],
) -> Result<(), RenderError> {
    if canvas.width == 0 || canvas.height == 0 || canvas.stride_pixels < canvas.width {
        return Err(RenderError::InvalidSurface);
    }

    let needed =
        canvas.stride_pixels.checked_mul(canvas.height).ok_or(RenderError::InvalidSurface)?
            as usize;
    if canvas.buffer.len() < needed {
        return Err(RenderError::InvalidSurface);
    }

    let mut pixels = canvas_to_premul_rgba(canvas)?;
    {
        let mut surface = PixmapMut::from_bytes(&mut pixels, canvas.width, canvas.height)
            .ok_or(RenderError::InvalidSurface)?;
        for primitive in primitives {
            render_primitive(&mut surface, primitive);
        }
    }
    premul_rgba_to_canvas(&pixels, canvas);
    Ok(())
}

fn render_primitive(surface: &mut PixmapMut<'_>, primitive: &Primitive) {
    match primitive {
        Primitive::Pixel { point, color } => {
            if let Some(rect) = Rect::from_xywh(point.x as f32, point.y as f32, 1.0, 1.0) {
                let mut paint = Paint::default();
                paint.set_color(color_to_skia(*color));
                surface.fill_rect(rect, &paint, Transform::identity(), None);
            }
        }
        Primitive::Line { shape, paint } => {
            if let Some(path) = line_path(shape) {
                stroke_path(surface, &path, paint, paint.stroke.unwrap_or_default());
            }
        }
        Primitive::Rect { shape, paint } => {
            if let Some(rect) = rect_i_to_skia(*shape) {
                fill_rect(surface, rect, paint);
                let path = PathBuilder::from_rect(rect);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::RoundedRect { shape, paint } => {
            if let Some(path) = rounded_rect_path(shape) {
                fill_shape(surface, &path, FillRule::NonZero, paint);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::Triangle { shape, paint } => {
            if let Some(path) = triangle_path(shape) {
                fill_shape(surface, &path, FillRule::NonZero, paint);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::Circle { shape, paint } => {
            if let Some(path) = circle_path(shape) {
                fill_shape(surface, &path, FillRule::NonZero, paint);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::Ellipse { shape, paint } => {
            if let Some(path) = ellipse_path(shape) {
                fill_shape(surface, &path, FillRule::NonZero, paint);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::Arc { shape, paint } => render_arc(surface, shape, paint),
        Primitive::Path { shape, fill_rule, paint } => {
            if let Some(path) = path_2d_to_skia(shape) {
                fill_shape(surface, &path, *fill_rule, paint);
                stroke_shape(surface, &path, paint);
            }
        }
        Primitive::QuadBezier { shape, paint } => {
            if let Some(path) = quad_path(shape) {
                stroke_path(surface, &path, paint, paint.stroke.unwrap_or_default());
                if paint.fill.is_some() {
                    fill_shape(surface, &path, FillRule::NonZero, paint);
                }
            }
        }
        Primitive::CubicBezier { shape, paint } => {
            if let Some(path) = cubic_path(shape) {
                stroke_path(surface, &path, paint, paint.stroke.unwrap_or_default());
                if paint.fill.is_some() {
                    fill_shape(surface, &path, FillRule::NonZero, paint);
                }
            }
        }
        Primitive::Mesh { shape, paint } => render_mesh(surface, shape, paint),
        Primitive::GlyphRun { .. } => {
            // GlyphRun placements need an atlas lookup, which is owned by the caller.
        }
    }
}

fn fill_rect(surface: &mut PixmapMut<'_>, rect: Rect, paint: &PistilPaint) {
    if let Some(sk_paint) = fill_paint(paint) {
        surface.fill_rect(rect, &sk_paint, Transform::identity(), None);
    }
}

fn fill_shape(
    surface: &mut PixmapMut<'_>,
    path: &tiny_skia::Path,
    fill_rule: FillRule,
    paint: &PistilPaint,
) {
    if let Some(sk_paint) = fill_paint(paint) {
        surface.fill_path(path, &sk_paint, sk_fill_rule(fill_rule), Transform::identity(), None);
    }
}

fn stroke_shape(surface: &mut PixmapMut<'_>, path: &tiny_skia::Path, paint: &PistilPaint) {
    if let Some(stroke) = paint.stroke {
        stroke_path(surface, path, paint, stroke);
    }
}

fn stroke_path(
    surface: &mut PixmapMut<'_>,
    path: &tiny_skia::Path,
    paint: &PistilPaint,
    style: StrokeStyle,
) {
    if let Some(sk_paint) = stroke_paint(paint) {
        surface.stroke_path(path, &sk_paint, &sk_stroke(style), Transform::identity(), None);
    }
}

fn fill_paint(paint: &PistilPaint) -> Option<Paint<'static>> {
    let fill = paint.fill.as_ref()?;
    let mut sk_paint = Paint::default();
    sk_paint.blend_mode = sk_blend_mode(paint.blend);
    sk_paint.shader = fill_style_to_shader(fill)?;
    Some(sk_paint)
}

fn stroke_paint(paint: &PistilPaint) -> Option<Paint<'static>> {
    let mut sk_paint = Paint::default();
    sk_paint.blend_mode = sk_blend_mode(paint.blend);
    sk_paint.shader = match paint.fill.as_ref() {
        Some(fill) => fill_style_to_shader(fill)?,
        None => Shader::SolidColor(Color::WHITE),
    };
    Some(sk_paint)
}

fn fill_style_to_shader(fill: &FillStyle) -> Option<Shader<'static>> {
    match fill {
        FillStyle::Solid(color) => Some(Shader::SolidColor(color_to_skia(*color))),
        FillStyle::LinearGradient(gradient) => SkLinearGradient::new(
            tiny_skia::Point::from_xy(gradient.start.x, gradient.start.y),
            tiny_skia::Point::from_xy(gradient.end.x, gradient.end.y),
            gradient_stops(&gradient.stops),
            SpreadMode::Pad,
            Transform::identity(),
        ),
        FillStyle::RadialGradient(gradient) => SkRadialGradient::new(
            tiny_skia::Point::from_xy(gradient.center.x, gradient.center.y),
            0.0,
            tiny_skia::Point::from_xy(gradient.center.x, gradient.center.y),
            gradient.radius.max(0.0),
            gradient_stops(&gradient.stops),
            SpreadMode::Pad,
            Transform::identity(),
        ),
    }
}

fn gradient_stops(stops: &[crate::typography::GradientStop]) -> Vec<SkGradientStop> {
    stops.iter().map(|stop| SkGradientStop::new(stop.offset, color_to_skia(stop.color))).collect()
}

fn color_to_skia(color: ColorRgba8) -> Color {
    Color::from_rgba8(color.r, color.g, color.b, color.a)
}

fn sk_blend_mode(mode: BlendMode) -> SkBlendMode {
    match mode {
        BlendMode::Src => SkBlendMode::Source,
        BlendMode::SrcOver => SkBlendMode::SourceOver,
        BlendMode::DstOver => SkBlendMode::DestinationOver,
        BlendMode::Multiply => SkBlendMode::Multiply,
        BlendMode::Screen => SkBlendMode::Screen,
        BlendMode::Overlay => SkBlendMode::Overlay,
        BlendMode::Darken => SkBlendMode::Darken,
        BlendMode::Lighten => SkBlendMode::Lighten,
        BlendMode::Plus => SkBlendMode::Plus,
    }
}

fn sk_fill_rule(rule: FillRule) -> SkFillRule {
    match rule {
        FillRule::NonZero => SkFillRule::Winding,
        FillRule::EvenOdd => SkFillRule::EvenOdd,
    }
}

fn sk_stroke(style: StrokeStyle) -> Stroke {
    Stroke {
        width: style.width.max(0.0),
        miter_limit: style.miter_limit.max(0.0),
        line_cap: match style.line_cap {
            LineCap::Butt => SkLineCap::Butt,
            LineCap::Round => SkLineCap::Round,
            LineCap::Square => SkLineCap::Square,
        },
        line_join: match style.line_join {
            LineJoin::Miter => SkLineJoin::Miter,
            LineJoin::Round => SkLineJoin::Round,
            LineJoin::Bevel => SkLineJoin::Bevel,
        },
        dash: None,
    }
}

fn rect_i_to_skia(rect: RectI) -> Option<Rect> {
    Rect::from_xywh(
        rect.origin.x as f32,
        rect.origin.y as f32,
        rect.size.width.max(0) as f32,
        rect.size.height.max(0) as f32,
    )
}

fn line_path(line: &LineSegment) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(line.start.x, line.start.y);
    pb.line_to(line.end.x, line.end.y);
    pb.finish()
}

fn triangle_path(triangle: &Triangle) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(triangle.p0.x, triangle.p0.y);
    pb.line_to(triangle.p1.x, triangle.p1.y);
    pb.line_to(triangle.p2.x, triangle.p2.y);
    pb.close();
    pb.finish()
}

fn circle_path(circle: &Circle) -> Option<tiny_skia::Path> {
    PathBuilder::from_circle(circle.center.x, circle.center.y, circle.radius.max(0.0))
}

fn ellipse_path(ellipse: &Ellipse) -> Option<tiny_skia::Path> {
    let rect = Rect::from_xywh(
        ellipse.center.x - ellipse.radius_x,
        ellipse.center.y - ellipse.radius_y,
        ellipse.radius_x.max(0.0) * 2.0,
        ellipse.radius_y.max(0.0) * 2.0,
    )?;
    PathBuilder::from_oval(rect)
}

fn rounded_rect_path(shape: &RoundedRect) -> Option<tiny_skia::Path> {
    let rect = rect_i_to_skia(shape.rect)?;
    let rx = shape.radius_x.max(0.0).min(rect.width() * 0.5);
    let ry = shape.radius_y.max(0.0).min(rect.height() * 0.5);
    if rx == 0.0 || ry == 0.0 {
        return Some(PathBuilder::from_rect(rect));
    }

    let k = 0.552_284_8;
    let l = rect.left();
    let t = rect.top();
    let r = rect.right();
    let b = rect.bottom();
    let ox = rx * k;
    let oy = ry * k;

    let mut pb = PathBuilder::new();
    pb.move_to(l + rx, t);
    pb.line_to(r - rx, t);
    pb.cubic_to(r - rx + ox, t, r, t + ry - oy, r, t + ry);
    pb.line_to(r, b - ry);
    pb.cubic_to(r, b - ry + oy, r - rx + ox, b, r - rx, b);
    pb.line_to(l + rx, b);
    pb.cubic_to(l + rx - ox, b, l, b - ry + oy, l, b - ry);
    pb.line_to(l, t + ry);
    pb.cubic_to(l, t + ry - oy, l + rx - ox, t, l + rx, t);
    pb.close();
    pb.finish()
}

fn quad_path(curve: &QuadBezier) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(curve.p0.x, curve.p0.y);
    pb.quad_to(curve.p1.x, curve.p1.y, curve.p2.x, curve.p2.y);
    pb.finish()
}

fn cubic_path(curve: &CubicBezier) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(curve.p0.x, curve.p0.y);
    pb.cubic_to(curve.p1.x, curve.p1.y, curve.p2.x, curve.p2.y, curve.p3.x, curve.p3.y);
    pb.finish()
}

fn path_2d_to_skia(path: &Path2D) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::with_capacity(path.verbs.len(), path.verbs.len() * 2);
    for verb in &path.verbs {
        match *verb {
            PathVerb::MoveTo(p) => pb.move_to(p.x, p.y),
            PathVerb::LineTo(p) => pb.line_to(p.x, p.y),
            PathVerb::QuadTo(cp, p) => pb.quad_to(cp.x, cp.y, p.x, p.y),
            PathVerb::CubicTo(cp1, cp2, p) => pb.cubic_to(cp1.x, cp1.y, cp2.x, cp2.y, p.x, p.y),
            PathVerb::Close => pb.close(),
        }
    }
    pb.finish()
}

fn render_arc(surface: &mut PixmapMut<'_>, arc: &Arc, paint: &PistilPaint) {
    if arc.radius <= 0.0 || arc.sweep_angle_rad == 0.0 {
        return;
    }

    let steps = ((arc.sweep_angle_rad.abs() * arc.radius).ceil() as usize / 8).clamp(8, 96);
    let mut pb = PathBuilder::new();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let a = arc.start_angle_rad + arc.sweep_angle_rad * t;
        let x = arc.center.x + arc.radius * libm::cosf(a);
        let y = arc.center.y + arc.radius * libm::sinf(a);
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }

    if paint.fill.is_some() {
        pb.line_to(arc.center.x, arc.center.y);
        pb.close();
    }

    if let Some(path) = pb.finish() {
        if paint.fill.is_some() {
            fill_shape(surface, &path, FillRule::NonZero, paint);
        }
        stroke_shape(surface, &path, paint);
    }
}

fn render_mesh(surface: &mut PixmapMut<'_>, mesh: &Mesh2D, paint: &PistilPaint) {
    for tri in mesh.indices.chunks_exact(3) {
        let Some(v0) = mesh.vertices.get(tri[0] as usize) else { continue };
        let Some(v1) = mesh.vertices.get(tri[1] as usize) else { continue };
        let Some(v2) = mesh.vertices.get(tri[2] as usize) else { continue };
        let triangle = Triangle { p0: v0.position, p1: v1.position, p2: v2.position };
        let Some(path) = triangle_path(&triangle) else { continue };
        let mut tri_paint = paint.clone();
        if tri_paint.fill.is_none() {
            tri_paint.fill = Some(FillStyle::Solid(average_color(v0.color, v1.color, v2.color)));
        }
        fill_shape(surface, &path, FillRule::NonZero, &tri_paint);
        stroke_shape(surface, &path, &tri_paint);
    }
}

fn average_color(a: ColorRgba8, b: ColorRgba8, c: ColorRgba8) -> ColorRgba8 {
    ColorRgba8 {
        r: ((a.r as u16 + b.r as u16 + c.r as u16) / 3) as u8,
        g: ((a.g as u16 + b.g as u16 + c.g as u16) / 3) as u8,
        b: ((a.b as u16 + b.b as u16 + c.b as u16) / 3) as u8,
        a: ((a.a as u16 + b.a as u16 + c.a as u16) / 3) as u8,
    }
}

fn canvas_to_premul_rgba(canvas: &Canvas<'_>) -> Result<Vec<u8>, RenderError> {
    let width = canvas.width as usize;
    let height = canvas.height as usize;
    let stride = canvas.stride_pixels as usize;
    let mut out = Vec::new();
    out.resize(width * height * 4, 0);

    for y in 0..height {
        for x in 0..width {
            let argb = canvas.buffer[y * stride + x];
            let a = ((argb >> 24) & 0xFF) as u8;
            let r = ((argb >> 16) & 0xFF) as u8;
            let g = ((argb >> 8) & 0xFF) as u8;
            let b = (argb & 0xFF) as u8;
            let dst = (y * width + x) * 4;
            out[dst] = premul(r, a);
            out[dst + 1] = premul(g, a);
            out[dst + 2] = premul(b, a);
            out[dst + 3] = a;
        }
    }

    Ok(out)
}

fn premul_rgba_to_canvas(src: &[u8], canvas: &mut Canvas<'_>) {
    let width = canvas.width as usize;
    let height = canvas.height as usize;
    let stride = canvas.stride_pixels as usize;

    for y in 0..height {
        for x in 0..width {
            let src_idx = (y * width + x) * 4;
            let a = src[src_idx + 3];
            let r = unpremul(src[src_idx], a);
            let g = unpremul(src[src_idx + 1], a);
            let b = unpremul(src[src_idx + 2], a);
            canvas.buffer[y * stride + x] =
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
        }
    }
}

fn premul(c: u8, a: u8) -> u8 {
    ((c as u16 * a as u16 + 127) / 255) as u8
}

fn unpremul(c: u8, a: u8) -> u8 {
    if a == 0 { 0 } else { ((c as u16 * 255 + (a as u16 / 2)) / a as u16).min(255) as u8 }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;
    use crate::typography::{PointF, PointI, SizeI};

    #[test]
    fn renders_solid_rect_to_argb_canvas() {
        let mut pixels = [0u32; 16];
        let mut canvas = Canvas::new(&mut pixels, 4, 4, 4);
        let primitive = Primitive::Rect {
            shape: RectI::new(1, 1, 2, 2),
            paint: PistilPaint {
                fill: Some(FillStyle::Solid(ColorRgba8 { r: 0x12, g: 0x34, b: 0x56, a: 0xFF })),
                stroke: None,
                blend: BlendMode::Src,
            },
        };

        render_primitives(&mut canvas, &[primitive]).unwrap();

        assert_eq!(canvas.buffer[5], 0xFF12_3456);
        assert_eq!(canvas.buffer[10], 0xFF12_3456);
        assert_eq!(canvas.buffer[0], 0);
    }

    #[test]
    fn renders_gradient_path_and_stroke() {
        let mut pixels = [0u32; 64];
        let mut canvas = Canvas::new(&mut pixels, 8, 8, 8);
        let path = Path2D {
            verbs: vec![
                PathVerb::MoveTo(PointF { x: 1.0, y: 1.0 }),
                PathVerb::LineTo(PointF { x: 6.0, y: 1.0 }),
                PathVerb::LineTo(PointF { x: 6.0, y: 6.0 }),
                PathVerb::Close,
            ],
        };
        let primitive = Primitive::Path {
            shape: path,
            fill_rule: FillRule::NonZero,
            paint: PistilPaint {
                fill: Some(FillStyle::LinearGradient(crate::typography::LinearGradient {
                    start: PointF { x: 1.0, y: 1.0 },
                    end: PointF { x: 6.0, y: 6.0 },
                    stops: vec![
                        crate::typography::GradientStop {
                            offset: 0.0,
                            color: ColorRgba8 { r: 255, g: 0, b: 0, a: 255 },
                        },
                        crate::typography::GradientStop {
                            offset: 1.0,
                            color: ColorRgba8 { r: 0, g: 0, b: 255, a: 255 },
                        },
                    ],
                })),
                stroke: Some(StrokeStyle {
                    width: 1.0,
                    line_cap: LineCap::Round,
                    line_join: LineJoin::Round,
                    miter_limit: 4.0,
                }),
                blend: BlendMode::SrcOver,
            },
        };

        render_primitives(&mut canvas, &[primitive]).unwrap();

        assert!(canvas.buffer.iter().any(|&px| px != 0));
    }

    #[test]
    fn rejects_short_canvas_buffer() {
        let mut pixels = [0u32; 2];
        let mut canvas = Canvas::new(&mut pixels, 4, 4, 4);
        let primitive = Primitive::Pixel {
            point: PointI { x: 0, y: 0 },
            color: ColorRgba8 { r: 255, g: 255, b: 255, a: 255 },
        };

        assert_eq!(render_primitives(&mut canvas, &[primitive]), Err(RenderError::InvalidSurface));
    }

    #[test]
    fn rect_i_constructor_remains_available() {
        let rect = RectI { origin: PointI { x: 1, y: 2 }, size: SizeI { width: 3, height: 4 } };
        assert_eq!(rect_i_to_skia(rect).unwrap().width(), 3.0);
    }
}

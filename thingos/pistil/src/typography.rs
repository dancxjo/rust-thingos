use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct SizeI {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct RectI {
    pub origin: PointI,
    pub size: SizeI,
}

impl RectI {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            origin: PointI { x, y },
            size: SizeI { width, height },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ColorRgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba8 {
    /// Pack as `0xAARRGGBB`.
    pub const fn to_u32(self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment {
    pub start: PointF,
    pub end: PointF,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub p0: PointF,
    pub p1: PointF,
    pub p2: PointF,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub center: PointF,
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse {
    pub center: PointF,
    pub radius_x: f32,
    pub radius_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRect {
    pub rect: RectI,
    pub radius_x: f32,
    pub radius_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arc {
    pub center: PointF,
    pub radius: f32,
    pub start_angle_rad: f32,
    pub sweep_angle_rad: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadBezier {
    pub p0: PointF,
    pub p1: PointF,
    pub p2: PointF,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBezier {
    pub p0: PointF,
    pub p1: PointF,
    pub p2: PointF,
    pub p3: PointF,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendMode {
    Src,
    SrcOver,
    DstOver,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Plus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeStyle {
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    pub offset: f32,
    pub color: ColorRgba8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinearGradient {
    pub start: PointF,
    pub end: PointF,
    pub stops: Vec<GradientStop>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadialGradient {
    pub center: PointF,
    pub radius: f32,
    pub stops: Vec<GradientStop>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FillStyle {
    Solid(ColorRgba8),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    pub fill: Option<FillStyle>,
    pub stroke: Option<StrokeStyle>,
    pub blend: BlendMode,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            fill: Some(FillStyle::Solid(ColorRgba8 {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            })),
            stroke: None,
            blend: BlendMode::SrcOver,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex2D {
    pub position: PointF,
    pub uv: PointF,
    pub color: ColorRgba8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh2D {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathVerb {
    MoveTo(PointF),
    LineTo(PointF),
    QuadTo(PointF, PointF),
    CubicTo(PointF, PointF, PointF),
    Close,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Path2D {
    pub verbs: Vec<PathVerb>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphBitmap {
    pub glyph_id: u32,
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance: i16,
    pub pixels_a8: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphPlacement {
    pub glyph_id: u32,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphRun {
    pub face_id: u64,
    pub px_size: u16,
    pub glyphs: Vec<GlyphPlacement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Pixel {
        point: PointI,
        color: ColorRgba8,
    },
    Line {
        shape: LineSegment,
        paint: Paint,
    },
    Rect {
        shape: RectI,
        paint: Paint,
    },
    RoundedRect {
        shape: RoundedRect,
        paint: Paint,
    },
    Triangle {
        shape: Triangle,
        paint: Paint,
    },
    Circle {
        shape: Circle,
        paint: Paint,
    },
    Ellipse {
        shape: Ellipse,
        paint: Paint,
    },
    Arc {
        shape: Arc,
        paint: Paint,
    },
    Path {
        shape: Path2D,
        fill_rule: FillRule,
        paint: Paint,
    },
    QuadBezier {
        shape: QuadBezier,
        paint: Paint,
    },
    CubicBezier {
        shape: CubicBezier,
        paint: Paint,
    },
    Mesh {
        shape: Mesh2D,
        paint: Paint,
    },
    GlyphRun {
        run: GlyphRun,
        paint: Paint,
    },
}

pub const PISTIL_TYPOGRAPHY_ABI_VERSION: u32 = 1;

#[unsafe(no_mangle)]
pub extern "C" fn pistil_typography_abi_version() -> u32 {
    PISTIL_TYPOGRAPHY_ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_pack_rgba8(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ColorRgba8 { r, g, b, a }.to_u32()
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_rect_area(width: u32, height: u32) -> u64 {
    (width as u64) * (height as u64)
}

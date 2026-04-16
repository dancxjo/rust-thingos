use abi::font_protocol::{
    decode_request_tag, EnsureGlyphs, EnsureGlyphsResp, FaceMetrics, FontRequestTag,
    FontResponseTag, GetFaceMetrics, GlyphPlacement,
};
use abi::wire::ThingId;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use ipc_helpers::rpc::RpcServer;
use petals::font::TextRenderer;
use petals::Atlas;
use stem::syscall::socket::sendmsg;
use stem::syscall::vfs::vfs_thing_from_channel;
use stem::{error, info};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SizeI {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct ColorRgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
pub struct GlyphRun {
    pub face_id: ThingId,
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

pub struct FontService {
    renderer: TextRenderer,
    // (face_id, px_size) -> Atlas
    atlases: BTreeMap<(u64, u16), Atlas>,
    // (face_id, px_size, glyph_id) -> GlyphPlacement
    cache: BTreeMap<(u64, u16, u32), GlyphPlacement>,
}

/// Unpack the paired channel handles from arg0.
///
/// sprout passes `(write_h << 16) | read_h` as the spawn argument so that
/// pistil has both ends of the request/reply channel pair.
fn unpack_handles(arg: usize) -> (u32, u32) {
    let write_h = ((arg >> 16) & 0xFFFF) as u32;
    let read_h = (arg & 0xFFFF) as u32;
    (write_h, read_h)
}

pub fn run_service(arg0: usize) -> ! {
    info!("pistil: starting up...");

    if arg0 == 0 {
        error!("pistil: No channel handles provided!");
        loop {
            stem::yield_now();
        }
    }

    let (write_h, read_h) = unpack_handles(arg0);
    info!("pistil: Entering RPC loop (read={}, write={})", read_h, write_h);

    let mut service = FontService {
        renderer: TextRenderer::load_from_boot("/share/fonts/NotoSans-Regular.ttf")
            .expect("Failed to load default font"),
        atlases: BTreeMap::new(),
        cache: BTreeMap::new(),
    };

    let mut server = RpcServer::new(read_h);

    loop {
        let req = match server.next() {
            Ok(r) => r,
            Err(_) => {
                stem::yield_now();
                continue;
            }
        };

        let tag = match decode_request_tag(&req.payload) {
            Some(tag) => tag,
            None => {
                error!("pistil: Received invalid request tag");
                let _ =
                    server.reply_err(req.request_id, write_h, abi::errors::Errno::EINVAL);
                continue;
            }
        };

        match tag {
            FontRequestTag::Ping => {
                let resp = [FontResponseTag::Pong as u8];
                let _ = server.reply(req.request_id, write_h, &resp);
            }
            FontRequestTag::GetFaceMetrics => {
                if let Some(font_req) = GetFaceMetrics::decode(&req.payload[1..]) {
                    let font = &service.renderer.font;
                    let metrics = font
                        .horizontal_line_metrics(font_req.px_size as f32)
                        .unwrap_or_else(|| font.horizontal_line_metrics(16.0).unwrap());

                    let resp = FaceMetrics {
                        ascent: metrics.ascent as i16,
                        descent: metrics.descent as i16,
                        line_gap: metrics.line_gap as i16,
                        units_per_em: font.units_per_em() as u16,
                    };
                    let mut out_buf = [0u8; 10];
                    if let Some(len) = resp.encode(&mut out_buf) {
                        let _ = server.reply(req.request_id, write_h, &out_buf[..len]);
                    }
                } else {
                    let _ =
                        server.reply_err(req.request_id, write_h, abi::errors::Errno::EINVAL);
                }
            }
            FontRequestTag::EnsureGlyphs => {
                if let Some(font_req) = EnsureGlyphs::decode(&req.payload[1..]) {
                    handle_ensure_glyphs(
                        &mut service,
                        &server,
                        write_h,
                        req.request_id,
                        font_req,
                    );
                } else {
                    let _ =
                        server.reply_err(req.request_id, write_h, abi::errors::Errno::EINVAL);
                }
            }
        }
    }
}

fn handle_ensure_glyphs(
    service: &mut FontService,
    server: &RpcServer,
    write_h: u32,
    request_id: u64,
    req: EnsureGlyphs,
) {
    let face_id_u64 = req.face_id.to_u64_lossy();
    let px_size = req.px_size;

    // Get or create atlas
    if !service.atlases.contains_key(&(face_id_u64, px_size)) {
        let atlas = Atlas::new("font_atlas", 1024, 1024, 1).expect("Failed to create atlas");
        service.atlases.insert((face_id_u64, px_size), atlas);
    }
    let atlas = service.atlases.get_mut(&(face_id_u64, px_size)).unwrap();

    let mut placements = Vec::new();
    let mut missing = Vec::new();

    for &gid in &req.glyph_ids {
        if let Some(p) = service.cache.get(&(face_id_u64, px_size, gid)) {
            placements.push(*p);
            continue;
        }

        // Rasterize
        let (metrics, bitmap) = service
            .renderer
            .font
            .rasterize(char::from_u32(gid).unwrap_or(' '), px_size as f32);

        if metrics.width == 0 || metrics.height == 0 {
            let p = GlyphPlacement {
                glyph_id: gid,
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                bearing_x: metrics.xmin as i16,
                bearing_y: metrics.ymin as i16,
                advance: metrics.advance_width as i16,
            };
            service.cache.insert((face_id_u64, px_size, gid), p);
            placements.push(p);
            continue;
        }

        // Pack
        if let Some((x, y)) = atlas.pack(metrics.width as u32, metrics.height as u32, &bitmap) {
            let p = GlyphPlacement {
                glyph_id: gid,
                x: x as u16,
                y: y as u16,
                w: metrics.width as u16,
                h: metrics.height as u16,
                bearing_x: metrics.xmin as i16,
                bearing_y: metrics.ymin as i16,
                advance: metrics.advance_width as i16,
            };
            service.cache.insert((face_id_u64, px_size, gid), p);
            placements.push(p);
        } else {
            missing.push(gid);
        }
    }

    let resp = EnsureGlyphsResp {
        req_face_id: req.face_id,
        req_px_size: px_size,
        atlas_fd: atlas.texture.fd,
        atlas_width: atlas.texture.width,
        atlas_height: atlas.texture.height,
        atlas_format: abi::font_protocol::AtlasFormat::A8,
        atlas_version: 1, // Need to increment this if we invalidate
        placements,
        missing,
    };

    let mut resp_buf = vec![0u8; 4096 * 4];
    if let Some(len) = resp.encode(&mut resp_buf) {
        // Send atlas fd alongside the encoded response framed with RpcHeader.
        let write_h_fd = vfs_thing_from_channel(write_h).unwrap_or(write_h);
        let _ = sendmsg(write_h_fd, &[], &[atlas.texture.fd]);
        let _ = server.reply(request_id, write_h, &resp_buf[..len]);
    }
}

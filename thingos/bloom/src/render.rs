use alloc::string::String;
use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use pistil_types::Texture;
use stem::syscall::vfs::{vfs_close, vfs_lstat, vfs_open, vfs_readdir};

use crate::display::DisplayBackend;

const PISTIL_PATH: &str = "/lib/libpistil.so";
const PREPARE_BACKGROUND_SYMBOL: &[u8] = b"pistil_prepare_background";
const PREPARE_CURSOR_SYMBOL: &[u8] = b"pistil_prepare_cursor";
const DEFAULT_CURSOR_PATH: &str = "/share/cursors/default.svg";
const CURSOR_SIZE: u32 = 32;
const POINTER_OVERLAY_MAX_W: u32 = 460;
const POINTER_OVERLAY_MAX_H: u32 = 96;
const POINTER_OVERLAY_MARGIN: u32 = 12;
const SVG_DEBUG_MAX_W: u32 = 720;
const SVG_DEBUG_MAX_H: u32 = 360;
const SVG_DEBUG_MARGIN: u32 = 12;
const SVG_DEBUG_TILE: u32 = 72;
const SVG_DEBUG_ICON: u32 = 48;
const SVG_DEBUG_MAX_ITEMS: usize = 32;
const SVG_DEBUG_MAX_DEPTH: u32 = 6;
const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;
const O_RDONLY: u32 = 0;
const SVG_DEBUG_ROOTS: &[&str] = &["/share", "/assets", "/boot", "/session"];
const SVG_DEBUG_KNOWN_PATHS: &[&str] = &["/share/cursors/default.svg", "/boot.svg"];

type PrepareBackgroundFn = extern "C" fn(
    path: *const u8,
    dst: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32;

type PrepareCursorFn = extern "C" fn(
    path: *const u8,
    dst: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    hotspot_out: *mut u32,
) -> i32;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    cursor: Option<CursorBuffer>,
    pointer_overlay: Option<PointerOverlayBuffer>,
    svg_debug: Option<SvgDebugBuffer>,
    pistil: Option<PistilLib>,
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
    prepare_cursor: Option<PrepareCursorFn>,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

#[derive(Clone, Copy)]
pub struct CursorPlane {
    pub buffer_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct OverlayPlane {
    pub buffer_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

struct CursorBuffer {
    _texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
    hotspot_x: u32,
    hotspot_y: u32,
}

struct PointerOverlayBuffer {
    texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

struct SvgDebugBuffer {
    _texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        let pistil = load_pistil();

        Self { background: None, cursor: None, pointer_overlay: None, svg_debug: None, pistil }
    }

    /// Install a solid-colour background that can be presented immediately.
    pub fn prepare_solid_background(&mut self, display: &DisplayBackend, argb: u32) {
        let (width, height) = display.output_size();

        let mut texture = match Texture::new("bloom.compositor.background", width, height, 4) {
            Some(t) => t,
            None => return,
        };
        texture.as_slice_mut().fill(argb);

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        ) else {
            return;
        };

        if let Some(old) = self.background.take() {
            display.release_buffer(old.buffer_id);
        }

        self.background = Some(ServerBuffer { _texture: texture, buffer_id });
    }

    /// Synchronously decode and import a wallpaper.
    pub fn prepare_background(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        let (width, height) = display.output_size();

        let mut texture: Texture =
            match Texture::new("bloom.compositor.background", width, height, 4) {
                Some(t) => t,
                None => return,
            };

        let success = if let Some(ref lib) = self.pistil {
            let res = call_prepare_bg(
                lib.prepare_bg,
                wallpaper_path,
                texture.as_slice_mut().as_mut_ptr(),
                width,
                height,
            );
            if res != 0 {
                stem::error!(
                    "bloom: wallpaper decode failed in pistil (code {}); keeping previous wallpaper",
                    res
                );
            }
            res == 0
        } else {
            false
        };

        if !success {
            if self.background.is_some() {
                return;
            }
            stem::info!("bloom: using periwinkle fallback background");
            texture.as_slice_mut().fill(0xFFCCCCFF);
        }

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        ) else {
            return;
        };

        if let Some(old) = self.background.take() {
            display.release_buffer(old.buffer_id);
        }

        self.background = Some(ServerBuffer { _texture: texture, buffer_id });
    }

    /// Decode and import a wallpaper after the service loop is live.
    ///
    /// This intentionally runs on the compositor thread for now. The current
    /// user allocator is fragile under concurrent wallpaper decoding, and the
    /// bundled BMP assets are small enough that an inline decode is safer than
    /// risking heap corruption during startup.
    pub fn start_background_load(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        self.prepare_background(display, wallpaper_path);
    }

    /// Background loads are currently completed inline on the compositor
    /// thread, so there is no pending worker result to poll.
    pub fn poll_ready_background(&mut self, _display: &DisplayBackend) -> bool {
        false
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }

    pub fn prepare_cursor(&mut self, display: &DisplayBackend) {
        let mut texture = match Texture::new("bloom.compositor.cursor", CURSOR_SIZE, CURSOR_SIZE, 4)
        {
            Some(t) => t,
            None => {
                stem::warn!("bloom: failed to allocate cursor texture");
                return;
            }
        };

        let mut hotspot = [3u32, 2u32];
        let success = if let Some(ref lib) = self.pistil {
            if let Some(prepare_cursor) = lib.prepare_cursor {
                call_prepare_cursor(
                    prepare_cursor,
                    DEFAULT_CURSOR_PATH,
                    texture.as_slice_mut().as_mut_ptr(),
                    CURSOR_SIZE,
                    CURSOR_SIZE,
                    &mut hotspot,
                ) == 0
            } else {
                false
            }
        } else {
            false
        };

        if !success {
            draw_fallback_cursor(texture.as_slice_mut(), CURSOR_SIZE, CURSOR_SIZE, CURSOR_SIZE);
        }

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            texture.width,
            texture.height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        ) else {
            stem::warn!("bloom: failed to import cursor texture");
            return;
        };

        if let Some(old) = self.cursor.take() {
            display.release_buffer(old.buffer_id);
        }

        self.cursor = Some(CursorBuffer {
            _texture: texture,
            buffer_id,
            width: CURSOR_SIZE,
            height: CURSOR_SIZE,
            hotspot_x: hotspot[0],
            hotspot_y: hotspot[1],
        });
        stem::info!(
            "bloom: cursor ready buffer={} size={}x{} hotspot={},{}",
            buffer_id,
            CURSOR_SIZE,
            CURSOR_SIZE,
            hotspot[0],
            hotspot[1]
        );
    }

    pub fn prepare_svg_debug_surface(&mut self, display: &DisplayBackend) {
        let (output_w, output_h) = display.output_size();
        if output_w <= SVG_DEBUG_MARGIN * 2 || output_h <= SVG_DEBUG_MARGIN * 2 {
            return;
        }

        let width = SVG_DEBUG_MAX_W.min(output_w.saturating_sub(SVG_DEBUG_MARGIN * 2));
        let height = SVG_DEBUG_MAX_H.min(output_h.saturating_sub(POINTER_OVERLAY_MAX_H + 36));
        if width < SVG_DEBUG_TILE || height < SVG_DEBUG_TILE {
            stem::warn!("bloom: SVG debug surface skipped; output too small");
            return;
        }

        let mut texture = match Texture::new("bloom.compositor.svg_debug", width, height, 4) {
            Some(t) => t,
            None => {
                stem::warn!("bloom: failed to allocate SVG debug surface");
                return;
            }
        };

        let paths = discover_svg_paths();
        draw_svg_debug_surface(
            texture.as_slice_mut(),
            width,
            height,
            &paths,
            self.pistil.as_ref().and_then(|lib| lib.prepare_cursor),
        );

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        ) else {
            stem::warn!("bloom: failed to import SVG debug surface");
            return;
        };

        if let Some(old) = self.svg_debug.take() {
            display.release_buffer(old.buffer_id);
        }

        self.svg_debug = Some(SvgDebugBuffer { _texture: texture, buffer_id, width, height });
        stem::info!(
            "bloom: SVG debug surface ready buffer={} size={}x{} file_svgs={}",
            buffer_id,
            width,
            height,
            paths.len()
        );
    }

    pub fn svg_debug_plane(&self) -> Option<OverlayPlane> {
        let debug = self.svg_debug.as_ref()?;
        Some(OverlayPlane {
            buffer_id: debug.buffer_id,
            x: SVG_DEBUG_MARGIN as i32,
            y: (POINTER_OVERLAY_MARGIN + POINTER_OVERLAY_MAX_H + SVG_DEBUG_MARGIN) as i32,
            width: debug.width,
            height: debug.height,
        })
    }

    pub fn cursor_plane(&self, pointer_x: i32, pointer_y: i32) -> Option<CursorPlane> {
        let cursor = self.cursor.as_ref()?;
        Some(CursorPlane {
            buffer_id: cursor.buffer_id,
            x: pointer_x.saturating_sub(cursor.hotspot_x as i32),
            y: pointer_y.saturating_sub(cursor.hotspot_y as i32),
            width: cursor.width,
            height: cursor.height,
        })
    }

    pub fn pointer_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        pointer_x: i32,
        pointer_y: i32,
    ) -> Option<OverlayPlane> {
        self.ensure_pointer_overlay(display)?;

        let overlay = self.pointer_overlay.as_mut()?;
        draw_pointer_overlay(
            overlay.texture.as_slice_mut(),
            overlay.width,
            overlay.height,
            pointer_x,
            pointer_y,
        );
        Some(OverlayPlane {
            buffer_id: overlay.buffer_id,
            x: POINTER_OVERLAY_MARGIN as i32,
            y: POINTER_OVERLAY_MARGIN as i32,
            width: overlay.width,
            height: overlay.height,
        })
    }

    fn ensure_pointer_overlay(&mut self, display: &DisplayBackend) -> Option<()> {
        let (output_w, output_h) = display.output_size();
        if output_w <= POINTER_OVERLAY_MARGIN * 2 || output_h <= POINTER_OVERLAY_MARGIN * 2 {
            return None;
        }

        let width = POINTER_OVERLAY_MAX_W.min(output_w.saturating_sub(POINTER_OVERLAY_MARGIN * 2));
        let height = POINTER_OVERLAY_MAX_H.min(output_h.saturating_sub(POINTER_OVERLAY_MARGIN * 2));
        if width == 0 || height == 0 {
            return None;
        }

        if matches!(
            self.pointer_overlay.as_ref(),
            Some(overlay) if overlay.width == width && overlay.height == height
        ) {
            return Some(());
        }

        let texture = Texture::new("bloom.compositor.pointer_overlay", width, height, 4)?;
        let buffer_id = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        )?;

        if let Some(old) = self.pointer_overlay.take() {
            display.release_buffer(old.buffer_id);
        }

        self.pointer_overlay = Some(PointerOverlayBuffer { texture, buffer_id, width, height });
        stem::info!(
            "bloom: pointer debug overlay ready buffer={} size={}x{}",
            buffer_id,
            width,
            height
        );
        Some(())
    }
}

fn discover_svg_paths() -> Vec<String> {
    let mut found = Vec::new();
    for path in SVG_DEBUG_KNOWN_PATHS {
        push_svg_path_if_present(&mut found, path);
    }
    for root in SVG_DEBUG_ROOTS {
        collect_svg_paths(root, 0, &mut found);
        if found.len() >= SVG_DEBUG_MAX_ITEMS {
            break;
        }
    }
    found.sort();
    found.dedup();
    found.truncate(SVG_DEBUG_MAX_ITEMS);
    found
}

fn push_svg_path_if_present(found: &mut Vec<String>, path: &str) {
    if found.len() >= SVG_DEBUG_MAX_ITEMS || !path_has_svg_suffix(path) {
        return;
    }
    if vfs_lstat(path).is_ok() && !found.iter().any(|p| p == path) {
        found.push(String::from(path));
    }
}

fn collect_svg_paths(path: &str, depth: u32, found: &mut Vec<String>) {
    if found.len() >= SVG_DEBUG_MAX_ITEMS || depth > SVG_DEBUG_MAX_DEPTH {
        return;
    }

    let Ok(stat) = vfs_lstat(path) else {
        return;
    };
    if (stat.mode & S_IFMT) != S_IFDIR {
        push_svg_path_if_present(found, path);
        return;
    }

    let Ok(fd) = vfs_open(path, O_RDONLY) else {
        return;
    };
    let mut children = Vec::new();
    let mut buf = [0u8; 1024];
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0usize;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }
                    if end > offset {
                        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                            if name != "." && name != ".." && !name.is_empty() {
                                children.push(path_join(path, name));
                            }
                        }
                    }
                    offset = end.saturating_add(1);
                }
            }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);

    children.sort();
    for child in children {
        collect_svg_paths(&child, depth.saturating_add(1), found);
        if found.len() >= SVG_DEBUG_MAX_ITEMS {
            break;
        }
    }
}

fn path_join(parent: &str, name: &str) -> String {
    if parent == "/" {
        let mut path = String::from("/");
        path.push_str(name);
        return path;
    }
    let mut path = String::from(parent);
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str(name);
    path
}

fn path_has_svg_suffix(path: &str) -> bool {
    let bytes = path.as_bytes();
    if bytes.len() < 4 {
        return false;
    }
    let tail = &bytes[bytes.len() - 4..];
    (tail[0] == b'.') && lower(tail[1]) == b's' && lower(tail[2]) == b'v' && lower(tail[3]) == b'g'
}

fn lower(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' { byte + 32 } else { byte }
}

fn draw_svg_debug_surface(
    dst: &mut [u32],
    width: u32,
    height: u32,
    paths: &[String],
    prepare_cursor: Option<PrepareCursorFn>,
) {
    dst.fill(0);
    fill_rect(dst, width, 0, 0, width, height, 0xEE182026);
    fill_rect(dst, width, 0, 0, width, 2, 0xFFFFD166);
    draw_text(dst, width, height, 16, 14, 2, "BLOOM SVG DEBUG", 0xFFFFF4B0);

    let mut tile_index = 0usize;
    draw_svg_tile(dst, width, height, tile_index, "EMBEDDED DEFAULT", None, prepare_cursor);
    tile_index += 1;

    for path in paths {
        draw_svg_tile(dst, width, height, tile_index, path, Some(path.as_str()), prepare_cursor);
        stem::info!("bloom: SVG debug rendered {}", path);
        tile_index += 1;
        if tile_index >= max_svg_tiles(width, height) {
            break;
        }
    }
    stem::info!(
        "bloom: SVG debug rendered embedded fallback plus {} discovered files",
        paths.len()
    );
}

fn max_svg_tiles(width: u32, height: u32) -> usize {
    let cols = (width.saturating_sub(24) / SVG_DEBUG_TILE).max(1);
    let rows = (height.saturating_sub(64) / SVG_DEBUG_TILE).max(1);
    (cols * rows) as usize
}

fn draw_svg_tile(
    dst: &mut [u32],
    width: u32,
    height: u32,
    index: usize,
    label: &str,
    path: Option<&str>,
    prepare_cursor: Option<PrepareCursorFn>,
) {
    let cols = (width.saturating_sub(24) / SVG_DEBUG_TILE).max(1) as usize;
    let x = 16 + ((index % cols) as u32 * SVG_DEBUG_TILE) as i32;
    let y = 56 + ((index / cols) as u32 * SVG_DEBUG_TILE) as i32;
    if y >= height as i32 {
        return;
    }

    fill_rect(dst, width, x, y, SVG_DEBUG_TILE - 8, SVG_DEBUG_TILE - 8, 0xFF26313A);
    fill_rect(dst, width, x, y, SVG_DEBUG_TILE - 8, 1, 0xFF6EE7B7);

    let mut icon = alloc::vec![0u32; (SVG_DEBUG_ICON * SVG_DEBUG_ICON) as usize];
    let rendered = match (path, prepare_cursor) {
        (Some(svg_path), Some(func)) => {
            let mut hotspot = [0u32; 2];
            call_prepare_cursor(
                func,
                svg_path,
                icon.as_mut_ptr(),
                SVG_DEBUG_ICON,
                SVG_DEBUG_ICON,
                &mut hotspot,
            ) == 0
        }
        _ => svg::rasterize_cursor(
            svg::DEFAULT_CURSOR_SVG,
            &mut icon,
            SVG_DEBUG_ICON,
            SVG_DEBUG_ICON,
            SVG_DEBUG_ICON,
        )
        .is_ok(),
    };

    if rendered {
        blit_icon(dst, width, height, &icon, x + 8, y + 7);
    } else {
        draw_text(dst, width, height, x + 14, y + 20, 2, "ERR", 0xFFFF7A7A);
        if let Some(svg_path) = path {
            stem::warn!("bloom: SVG debug failed to render {}", svg_path);
        }
    }
    draw_label(dst, width, height, x + 4, y + 58, label);
}

fn blit_icon(dst: &mut [u32], stride: u32, height: u32, src: &[u32], x: i32, y: i32) {
    for sy in 0..SVG_DEBUG_ICON {
        let dy = y + sy as i32;
        if dy < 0 || dy >= height as i32 {
            continue;
        }
        for sx in 0..SVG_DEBUG_ICON {
            let dx = x + sx as i32;
            if dx < 0 || dx >= stride as i32 {
                continue;
            }
            let px = src[(sy * SVG_DEBUG_ICON + sx) as usize];
            if px >> 24 != 0 {
                dst[(dy as u32 * stride + dx as u32) as usize] = px;
            }
        }
    }
}

fn draw_label(dst: &mut [u32], stride: u32, height: u32, x: i32, y: i32, label: &str) {
    let mut chars = 0usize;
    let mut text = [b' '; 10];
    for byte in label.as_bytes().iter().rev() {
        if chars >= text.len() {
            break;
        }
        let b = if *byte == b'/' || *byte == b'.' || *byte == b':' {
            b' '
        } else if *byte >= b'a' && *byte <= b'z' {
            *byte - 32
        } else {
            *byte
        };
        text[text.len() - 1 - chars] = b;
        chars += 1;
    }
    let start = text.len().saturating_sub(chars);
    if let Ok(s) = core::str::from_utf8(&text[start..]) {
        draw_text(dst, stride, height, x, y, 1, s, 0xFFE6F7FF);
    }
}

fn load_pistil() -> Option<PistilLib> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        log_dlerror("bloom: failed to load /lib/libpistil.so");
        return None;
    }

    let sym = dlsym_bytes(handle, PREPARE_BACKGROUND_SYMBOL);
    if sym.is_null() {
        log_dlerror("bloom: failed to resolve pistil_prepare_background");
        return None;
    }

    let prepare_bg: PrepareBackgroundFn = unsafe { core::mem::transmute(sym) };
    let cursor_sym = dlsym_bytes(handle, PREPARE_CURSOR_SYMBOL);
    let prepare_cursor =
        if cursor_sym.is_null() { None } else { Some(unsafe { core::mem::transmute(cursor_sym) }) };
    stem::info!("bloom: pistil background renderer loaded from {}", PISTIL_PATH);
    Some(PistilLib { _handle: handle, prepare_bg, prepare_cursor })
}

fn log_dlerror(prefix: &str) {
    let err = dlerror();
    if err.is_null() {
        stem::warn!("{}", prefix);
        return;
    }

    let mut len = 0usize;
    unsafe {
        while *err.add(len) != 0 && len < 256 {
            len += 1;
        }
        let bytes = core::slice::from_raw_parts(err, len);
        match core::str::from_utf8(bytes) {
            Ok(msg) => stem::warn!("{}: {}", prefix, msg),
            Err(_) => stem::warn!("{}", prefix),
        }
    }
}

fn call_prepare_bg(
    prepare_bg: PrepareBackgroundFn,
    wallpaper_path: &str,
    dst: *mut u32,
    width: u32,
    height: u32,
) -> i32 {
    let bytes = wallpaper_path.as_bytes();
    if bytes.len() >= 256 {
        return -4;
    }

    let mut path_c = [0u8; 256];
    path_c[..bytes.len()].copy_from_slice(bytes);
    prepare_bg(path_c.as_ptr(), dst, width, height, width)
}

fn call_prepare_cursor(
    prepare_cursor: PrepareCursorFn,
    cursor_path: &str,
    dst: *mut u32,
    width: u32,
    height: u32,
    hotspot: &mut [u32; 2],
) -> i32 {
    let bytes = cursor_path.as_bytes();
    if bytes.len() >= 256 {
        return -4;
    }

    let mut path_c = [0u8; 256];
    path_c[..bytes.len()].copy_from_slice(bytes);
    prepare_cursor(path_c.as_ptr(), dst, width, height, width, hotspot.as_mut_ptr())
}

fn draw_fallback_cursor(dst: &mut [u32], width: u32, height: u32, stride: u32) {
    let _ = svg::rasterize_cursor(svg::DEFAULT_CURSOR_SVG, dst, width, height, stride);
}

fn draw_pointer_overlay(dst: &mut [u32], width: u32, height: u32, pointer_x: i32, pointer_y: i32) {
    dst.fill(0);
    fill_rect(dst, width, 0, 0, width, height, 0xAA101820);
    fill_rect(dst, width, 0, 0, width, 2, 0xFF7DD3FC);
    fill_rect(dst, width, 0, height.saturating_sub(2) as i32, width, 2, 0x6659C3C3);

    draw_text(dst, width, height, 16, 14, 2, "BLOOM POINTER DEBUG", 0xFFFFF4B0);
    draw_text(dst, width, height, 18, 48, 2, "POINTER 0: X=", 0xFFE6F7FF);
    let next_x = draw_signed_number(dst, width, height, 18 + 13 * 12, 48, 2, pointer_x, 0xFFFFFFFF);
    let next_x = next_x + 18;
    draw_text(dst, width, height, next_x, 48, 2, "Y=", 0xFFE6F7FF);
    draw_signed_number(dst, width, height, next_x + 24, 48, 2, pointer_y, 0xFFFFFFFF);
}

fn fill_rect(dst: &mut [u32], stride: u32, x: i32, y: i32, w: u32, h: u32, color: u32) {
    if stride == 0 {
        return;
    }
    let height = (dst.len() as u32) / stride;
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x.saturating_add(w as i32)).max(0) as u32;
    let y1 = (y.saturating_add(h as i32)).max(0) as u32;
    let x1 = x1.min(stride);
    let y1 = y1.min(height);
    for yy in y0..y1 {
        let row = (yy * stride) as usize;
        for xx in x0..x1 {
            dst[row + xx as usize] = color;
        }
    }
}

fn draw_text(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    mut x: i32,
    y: i32,
    scale: u32,
    text: &str,
    color: u32,
) -> i32 {
    for ch in text.chars() {
        if ch == ' ' {
            x += (6 * scale) as i32;
            continue;
        }
        draw_glyph(dst, stride, height, x, y, scale, ch, color);
        x += (6 * scale) as i32;
    }
    x
}

fn draw_signed_number(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    scale: u32,
    value: i32,
    color: u32,
) -> i32 {
    let mut buf = [0u8; 12];
    let mut len = 0usize;
    let mut n = value as i64;
    if n < 0 {
        buf[len] = b'-';
        len += 1;
        n = -n;
    }
    let start = len;
    loop {
        buf[len] = b'0' + (n % 10) as u8;
        len += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    buf[start..len].reverse();

    let mut cursor = x;
    for byte in &buf[..len] {
        draw_glyph(dst, stride, height, cursor, y, scale, *byte as char, color);
        cursor += (6 * scale) as i32;
    }
    cursor
}

fn draw_glyph(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    scale: u32,
    ch: char,
    color: u32,
) {
    if y >= height as i32 {
        return;
    }

    let glyph = glyph_rows(ch);
    for (row, bits) in glyph.iter().enumerate() {
        for col in 0..5u32 {
            if bits & (0b10000 >> col) != 0 {
                fill_rect(
                    dst,
                    stride,
                    x + (col * scale) as i32,
                    y + (row as u32 * scale) as i32,
                    scale,
                    scale,
                    color,
                );
            }
        }
    }
}

fn glyph_rows(ch: char) -> [u8; 7] {
    match ch {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
        '6' => [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        ':' => [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
        '=' => [0b00000, 0b11111, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        _ => [0b11111, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100],
    }
}

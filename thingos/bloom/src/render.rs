use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use pistil_types::Texture;

use crate::display::DisplayBackend;
use crate::scene::{
    ChromeButton, CompositionEntry, CursorKind, SurfaceChrome, chrome_button_rects,
};
use crate::theme::{UiTheme, default_theme, theme_by_name};

const PISTIL_PATH: &str = "/lib/libpistil.so";
const PREPARE_BACKGROUND_SYMBOL: &[u8] = b"pistil_prepare_background";
const PREPARE_CURSOR_SYMBOL: &[u8] = b"pistil_prepare_cursor";
const DRAW_SVG_ICON_SYMBOL: &[u8] = b"pistil_draw_svg_icon";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const DRAW_SYMBOL_TEXT_SYMBOL: &[u8] = b"pistil_draw_symbol_text";
const DEFAULT_CURSOR_PATH: &str = "/share/cursors/future/default.svg";
const MOVE_CURSOR_PATH: &str = "/share/cursors/future/fleur.svg";
const RESIZE_N_CURSOR_PATH: &str = "/share/cursors/future/top_side.svg";
const RESIZE_S_CURSOR_PATH: &str = "/share/cursors/future/bottom_side.svg";
const RESIZE_E_CURSOR_PATH: &str = "/share/cursors/future/right_side.svg";
const RESIZE_W_CURSOR_PATH: &str = "/share/cursors/future/left_side.svg";
const RESIZE_NE_CURSOR_PATH: &str = "/share/cursors/future/top_right_corner.svg";
const RESIZE_NW_CURSOR_PATH: &str = "/share/cursors/future/top_left_corner.svg";
const RESIZE_SE_CURSOR_PATH: &str = "/share/cursors/future/bottom_right_corner.svg";
const RESIZE_SW_CURSOR_PATH: &str = "/share/cursors/future/bottom_left_corner.svg";
const WINDOW_ICON_MINIMIZE_PATH: &str = "/share/icons/lucide/minus.svg";
const WINDOW_ICON_SHADE_PATH: &str = "/share/icons/lucide/chevron-up.svg";
const WINDOW_ICON_UNSHADE_PATH: &str = "/share/icons/lucide/chevron-down.svg";
const WINDOW_ICON_MAXIMIZE_PATH: &str = "/share/icons/lucide/square.svg";
const WINDOW_ICON_FULLSCREEN_PATH: &str = "/share/icons/lucide/fullscreen.svg";
const WINDOW_ICON_RESTORE_PATH: &str = "/share/icons/lucide/minimize-2.svg";
const WINDOW_ICON_CLOSE_PATH: &str = "/share/icons/lucide/x.svg";
const DEFAULT_FONT_PATH: &str = "/share/fonts/Inter-Regular.ttf";
const SYMBOL_FONT_PATH: &str = "/share/fonts/NotoSansSymbol2-Regular.ttf";
const CURSOR_SIZE: u32 = 96;
const CURSOR_PIXELS: usize = (CURSOR_SIZE * CURSOR_SIZE) as usize;
const POINTER_OVERLAY_MAX_W: u32 = 460;
const POINTER_OVERLAY_MAX_H: u32 = 144;
const POINTER_OVERLAY_MARGIN: u32 = 12;
const POINTER_OVERLAY_CURSOR_INSET: i32 = 24;
const CHROME_ICON_X_BIAS: i32 = 0;
const CHROME_ICON_Y_BIAS: i32 = 0;

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

type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;
type DrawSvgIconFn =
    extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, u32, u32, u32) -> i32;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    cursor: Option<CursorBuffer>,
    cursor_variants: Vec<(CursorKind, CursorBuffer)>,
    pointer_overlay: Option<PointerOverlayBuffer>,
    body_overlay: Option<ChromeOverlayBuffer>,
    chrome_overlay: Option<ChromeOverlayBuffer>,
    pistil: Option<PistilLib>,
    theme: UiTheme,
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
    prepare_cursor: Option<PrepareCursorFn>,
    draw_svg_icon: Option<DrawSvgIconFn>,
    draw_text: Option<DrawTextFn>,
    draw_symbol_text: Option<DrawTextFn>,
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

struct ChromeOverlayBuffer {
    texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        let pistil = load_pistil();

        Self {
            background: None,
            cursor: None,
            cursor_variants: Vec::new(),
            pointer_overlay: None,
            body_overlay: None,
            chrome_overlay: None,
            pistil,
            theme: default_theme(),
        }
    }

    pub fn set_theme_by_name(&mut self, name: &str) -> &'static str {
        self.theme = theme_by_name(name);
        self.theme.name
    }

    /// Returns the current theme's window corner radius for display clipping.
    pub fn corner_radius(&self) -> u8 {
        self.theme.corner_radius.min(255) as u8
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
            stem::warn!("bloom: failed to prepare cursor via pistil");
            return;
        }

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            texture.width,
            texture.height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
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

    pub fn cursor_plane(
        &mut self,
        display: &DisplayBackend,
        kind: CursorKind,
        pointer_x: i32,
        pointer_y: i32,
    ) -> Option<CursorPlane> {
        let cursor = self.cursor_buffer(display, kind)?;
        Some(CursorPlane {
            buffer_id: cursor.buffer_id,
            x: pointer_x.saturating_sub(cursor.hotspot_x as i32),
            y: pointer_y.saturating_sub(cursor.hotspot_y as i32),
            width: cursor.width,
            height: cursor.height,
        })
    }

    pub fn chrome_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        composition: &[CompositionEntry],
    ) -> (Option<OverlayPlane>, Option<OverlayPlane>) {
        if !composition.iter().any(|entry| !entry.chrome.is_empty()) {
            return (None, None);
        }
        self.ensure_overlay_buffers(display);

        let draw_svg_icon = self.pistil.as_ref().and_then(|lib| lib.draw_svg_icon);
        let draw_text = self.pistil.as_ref().and_then(|lib| lib.draw_text);
        let draw_symbol_text = self.pistil.as_ref().and_then(|lib| lib.draw_symbol_text);

        if let Some(body) = self.body_overlay.as_mut() {
            draw_window_body_overlay(
                body.texture.as_slice_mut(),
                body.width,
                body.height,
                composition,
                self.theme,
            );
        }

        if let Some(chrome) = self.chrome_overlay.as_mut() {
            draw_chrome_overlay(
                chrome.texture.as_slice_mut(),
                chrome.width,
                chrome.height,
                composition,
                draw_svg_icon,
                draw_text,
                draw_symbol_text,
                self.theme,
            );
        }

        (
            self.body_overlay.as_ref().map(|o| OverlayPlane {
                buffer_id: o.buffer_id,
                x: 0,
                y: 0,
                width: o.width,
                height: o.height,
            }),
            self.chrome_overlay.as_ref().map(|o| OverlayPlane {
                buffer_id: o.buffer_id,
                x: 0,
                y: 0,
                width: o.width,
                height: o.height,
            }),
        )
    }

    fn cursor_buffer(
        &mut self,
        display: &DisplayBackend,
        kind: CursorKind,
    ) -> Option<&CursorBuffer> {
        if kind == CursorKind::Default {
            return self.cursor.as_ref();
        }
        if let Some(idx) = self.cursor_variants.iter().position(|(k, _)| *k == kind) {
            return Some(&self.cursor_variants[idx].1);
        }
        self.prepare_cursor_variant(display, kind)?;
        self.cursor_variants
            .iter()
            .find_map(|(k, cursor)| if *k == kind { Some(cursor) } else { None })
    }

    fn prepare_cursor_variant(&mut self, display: &DisplayBackend, kind: CursorKind) -> Option<()> {
        let mut texture =
            Texture::new("bloom.compositor.cursor.variant", CURSOR_SIZE, CURSOR_SIZE, 4)?;
        let cursor_path = cursor_path(kind)?;
        let mut hotspot = [CURSOR_SIZE / 2, CURSOR_SIZE / 2];
        let success = if let Some(ref lib) = self.pistil {
            if let Some(prepare_cursor) = lib.prepare_cursor {
                call_prepare_cursor(
                    prepare_cursor,
                    cursor_path,
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
            stem::warn!("bloom: failed to prepare {} via pistil", cursor_path);
            return None;
        }
        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            texture.width,
            texture.height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
            0,
        ) else {
            stem::warn!("bloom: failed to import cursor variant texture");
            return None;
        };
        self.cursor_variants.push((
            kind,
            CursorBuffer {
                _texture: texture,
                buffer_id,
                width: CURSOR_SIZE,
                height: CURSOR_SIZE,
                hotspot_x: hotspot[0],
                hotspot_y: hotspot[1],
            },
        ));
        stem::info!("bloom: cursor {:?} ready from {} buffer={}", kind, cursor_path, buffer_id);
        Some(())
    }

    fn ensure_overlay_buffers(&mut self, display: &DisplayBackend) {
        let (width, height) = display.output_size();
        if !self.body_overlay.as_ref().map_or(false, |o| o.width == width && o.height == height) {
            if let Some(buffer) =
                make_overlay_buffer(display, "bloom.compositor.window_body", width, height)
            {
                release_overlay_buffer(display, self.body_overlay.take());
                self.body_overlay = Some(buffer);
            }
        }

        if !self.chrome_overlay.as_ref().map_or(false, |o| o.width == width && o.height == height) {
            if let Some(buffer) =
                make_overlay_buffer(display, "bloom.compositor.window_chrome", width, height)
            {
                release_overlay_buffer(display, self.chrome_overlay.take());
                self.chrome_overlay = Some(buffer);
                stem::info!("bloom: flat window overlays ready size={}x{}", width, height);
            }
        }
    }

    pub fn pointer_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        pointer_x: i32,
        pointer_y: i32,
    ) -> Option<OverlayPlane> {
        self.ensure_pointer_overlay(display)?;

        let mut cursor_sample = [0u32; CURSOR_PIXELS];
        let cursor_sample = self.cursor.as_ref().and_then(|cursor| {
            copy_cursor_sample(cursor, &mut cursor_sample)?;
            Some(&cursor_sample[..])
        });

        let overlay = self.pointer_overlay.as_mut()?;
        draw_pointer_overlay(
            overlay.texture.as_slice_mut(),
            overlay.width,
            overlay.height,
            pointer_x,
            pointer_y,
            cursor_sample,
            self.pistil.as_ref().and_then(|lib| lib.draw_text),
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
    let svg_icon_sym = dlsym_bytes(handle, DRAW_SVG_ICON_SYMBOL);
    let draw_svg_icon = if svg_icon_sym.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute(svg_icon_sym) })
    };
    let text_sym = dlsym_bytes(handle, DRAW_TEXT_SYMBOL);
    let draw_text =
        if text_sym.is_null() { None } else { Some(unsafe { core::mem::transmute(text_sym) }) };
    let symbol_text_sym = dlsym_bytes(handle, DRAW_SYMBOL_TEXT_SYMBOL);
    let draw_symbol_text = if symbol_text_sym.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute(symbol_text_sym) })
    };
    stem::info!("bloom: pistil background renderer loaded from {}", PISTIL_PATH);
    if draw_text.is_some() {
        stem::info!("bloom: pistil font text renderer loaded with default {}", DEFAULT_FONT_PATH);
    }
    if draw_symbol_text.is_some() {
        stem::info!("bloom: pistil symbol renderer loaded with {}", SYMBOL_FONT_PATH);
    }
    Some(PistilLib {
        _handle: handle,
        prepare_bg,
        prepare_cursor,
        draw_svg_icon,
        draw_text,
        draw_symbol_text,
    })
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

fn call_draw_svg_icon(
    draw_svg_icon: DrawSvgIconFn,
    icon_path: &str,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    icon_w: u32,
    icon_h: u32,
    color: u32,
) -> i32 {
    let bytes = icon_path.as_bytes();
    if bytes.len() >= 256 {
        return -4;
    }

    let mut path_c = [0u8; 256];
    path_c[..bytes.len()].copy_from_slice(bytes);
    draw_svg_icon(
        path_c.as_ptr(),
        dst.as_mut_ptr(),
        stride,
        height,
        stride,
        x,
        y,
        icon_w,
        icon_h,
        color,
    )
}

fn copy_cursor_sample(cursor: &CursorBuffer, dst: &mut [u32; CURSOR_PIXELS]) -> Option<()> {
    if cursor.width != CURSOR_SIZE || cursor.height != CURSOR_SIZE {
        return None;
    }

    let src_stride = cursor._texture.stride / 4;
    if src_stride < CURSOR_SIZE {
        return None;
    }

    let src_len = (cursor._texture.height * src_stride) as usize;
    let src = unsafe { core::slice::from_raw_parts(cursor._texture.ptr as *const u32, src_len) };
    for y in 0..CURSOR_SIZE {
        let src_row = (y * src_stride) as usize;
        let dst_row = (y * CURSOR_SIZE) as usize;
        dst[dst_row..dst_row + CURSOR_SIZE as usize]
            .copy_from_slice(&src[src_row..src_row + CURSOR_SIZE as usize]);
    }
    Some(())
}

fn make_overlay_buffer(
    display: &DisplayBackend,
    name: &'static str,
    width: u32,
    height: u32,
) -> Option<ChromeOverlayBuffer> {
    let texture = Texture::new(name, width, height, 4)?;
    let buffer_id = display.import_buffer(
        texture.fd,
        width,
        height,
        texture.stride,
        PixelFormat::Bgra8888,
        0,
        0,
    )?;
    Some(ChromeOverlayBuffer { texture, buffer_id, width, height })
}

fn release_overlay_buffer(display: &DisplayBackend, buffer: Option<ChromeOverlayBuffer>) {
    if let Some(buffer) = buffer {
        display.release_buffer(buffer.buffer_id);
    }
}

fn cursor_path(kind: CursorKind) -> Option<&'static str> {
    match kind {
        CursorKind::Default => Some(DEFAULT_CURSOR_PATH),
        CursorKind::Move => Some(MOVE_CURSOR_PATH),
        CursorKind::ResizeNorth => Some(RESIZE_N_CURSOR_PATH),
        CursorKind::ResizeSouth => Some(RESIZE_S_CURSOR_PATH),
        CursorKind::ResizeEast => Some(RESIZE_E_CURSOR_PATH),
        CursorKind::ResizeWest => Some(RESIZE_W_CURSOR_PATH),
        CursorKind::ResizeNorthEast => Some(RESIZE_NE_CURSOR_PATH),
        CursorKind::ResizeNorthWest => Some(RESIZE_NW_CURSOR_PATH),
        CursorKind::ResizeSouthEast => Some(RESIZE_SE_CURSOR_PATH),
        CursorKind::ResizeSouthWest => Some(RESIZE_SW_CURSOR_PATH),
    }
}

fn draw_chrome_overlay(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    composition: &[CompositionEntry],
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    pistil_draw_text: Option<DrawTextFn>,
    pistil_draw_symbol_text: Option<DrawTextFn>,
    theme: UiTheme,
) {
    dst.fill(0);
    for entry in composition {
        if entry.is_fullscreen {
            continue;
        }
        let chrome = entry.chrome;
        if chrome.is_empty() {
            continue;
        }
        let rect = entry.dest_rect;
        let frame = chrome.frame_thickness.min(rect.w / 2).min(rect.h / 2);
        if frame == 0 {
            continue;
        }
        let x = rect.x as i32;
        let y = rect.y as i32;
        let w = rect.w;
        let h = rect.h;
        let text_color = if entry.active { theme.chrome_text } else { theme.chrome_text_inactive };
        let icon_color =
            if entry.active { theme.control_icon } else { theme.control_icon_inactive };
        let titlebar_height = chrome.titlebar_height.min(theme.titlebar_height).min(h);
        let border_color = if entry.active { theme.active.border } else { theme.inactive.border };

        if titlebar_height > 0 {
            fill_rect(dst, stride, x, y, w, titlebar_height, theme.active.title_top);
            draw_chrome_buttons(
                dst,
                stride,
                height,
                rect,
                chrome,
                entry.active,
                entry.is_shaded,
                entry.is_fullscreen,
                pistil_draw_svg_icon,
                pistil_draw_symbol_text.or(pistil_draw_text),
                icon_color,
                theme.close_icon,
                theme,
            );
            if let Some(title) = entry.title.as_deref() {
                let text_x = x.saturating_add(frame as i32).saturating_add(8);
                let text_y = y.saturating_add((titlebar_height as i32 + 16) / 2);
                let buttons_w = chrome_button_rects(rect, chrome)
                    .map(|rects| {
                        let first = rects[0].1;
                        rect.x.saturating_add(rect.w).saturating_sub(first.x)
                    })
                    .unwrap_or(0);
                let max_chars = w
                    .saturating_sub(frame.saturating_mul(2))
                    .saturating_sub(24)
                    .saturating_sub(buttons_w)
                    .saturating_div(10)
                    .max(1) as usize;
                draw_overlay_text_bold(
                    pistil_draw_text,
                    dst,
                    stride,
                    height,
                    text_x,
                    text_y,
                    16.0,
                    title_prefix(title, max_chars),
                    text_color,
                );
            }
        }
        draw_flat_window_border(dst, stride, height, rect, frame, border_color);
        if titlebar_height > 0 && titlebar_height < h.saturating_sub(frame) {
            let sep_y = y.saturating_add(titlebar_height as i32);
            let rule =
                if entry.active { theme.title_rule_active } else { theme.title_rule_inactive };
            fill_rect(dst, stride, x, sep_y, w, frame.min(2), rule);
        }
    }

    let len = (stride.saturating_mul(height)) as usize;
    if dst.len() > len {
        for px in &mut dst[len..] {
            *px = 0;
        }
    }
}

fn draw_window_body_overlay(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    composition: &[CompositionEntry],
    theme: UiTheme,
) {
    dst.fill(0);
    for entry in composition {
        if entry.is_fullscreen {
            continue;
        }
        let chrome = entry.chrome;
        if chrome.is_empty() {
            continue;
        }

        let rect = entry.dest_rect;
        let frame = chrome.frame_thickness.min(rect.w / 2).min(rect.h / 2);
        let titlebar_height = chrome.titlebar_height.min(theme.titlebar_height).min(rect.h);
        if frame == 0 || titlebar_height >= rect.h {
            continue;
        }

        let body_x = rect.x as i32 + frame as i32;
        let body_y = rect.y as i32 + titlebar_height as i32;
        let body_w = rect.w.saturating_sub(frame.saturating_mul(2));
        let body_h = rect.h.saturating_sub(titlebar_height).saturating_sub(frame);
        if body_w == 0 || body_h == 0 {
            continue;
        }

        fill_rect(dst, stride, body_x, body_y, body_w, body_h, theme.body_top);
    }

    let len = (stride.saturating_mul(height)) as usize;
    if dst.len() > len {
        for px in &mut dst[len..] {
            *px = 0;
        }
    }
}

fn title_prefix(title: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }
    match title.char_indices().nth(max_chars) {
        Some((idx, _)) => &title[..idx],
        None => title,
    }
}

fn draw_flat_window_border(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    frame: u32,
    color: u32,
) {
    let x = rect.x as i32;
    let y = rect.y as i32;
    let w = rect.w;
    let h = rect.h;
    if w == 0 || h == 0 || frame == 0 {
        return;
    }

    let frame = frame.min(w / 2).min(h / 2);
    fill_rect(dst, stride, x, y, w, frame, color);
    fill_rect(dst, stride, x, y + h.saturating_sub(frame) as i32, w, frame, color);
    fill_rect(dst, stride, x, y, frame, h, color);
    fill_rect(dst, stride, x + w.saturating_sub(frame) as i32, y, frame, h, color);
}

fn draw_chrome_buttons(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    surface_rect: abi::display_protocol::Rect,
    chrome: SurfaceChrome,
    active: bool,
    is_shaded: bool,
    is_fullscreen: bool,
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    pistil_draw_symbol_text: Option<DrawTextFn>,
    icon_color: u32,
    close_icon: u32,
    theme: UiTheme,
) {
    let Some(buttons) = chrome_button_rects(surface_rect, chrome) else {
        return;
    };

    for (button, rect) in buttons {
        draw_chrome_button(
            dst,
            stride,
            height,
            rect,
            button,
            active,
            is_shaded,
            is_fullscreen,
            pistil_draw_svg_icon,
            pistil_draw_symbol_text,
            icon_color,
            close_icon,
            theme,
        );
    }
}

fn draw_chrome_button(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
    _active: bool,
    is_shaded: bool,
    is_fullscreen: bool,
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    _pistil_draw_symbol_text: Option<DrawTextFn>,
    icon_color: u32,
    close_icon: u32,
    theme: UiTheme,
) {
    if rect.w < 8 || rect.h < 8 {
        return;
    }

    draw_chrome_button_well(dst, stride, height, rect, theme);

    let color = if matches!(button, ChromeButton::Close) { close_icon } else { icon_color };
    if draw_chrome_button_lucide(
        pistil_draw_svg_icon,
        dst,
        stride,
        height,
        rect,
        button,
        is_shaded,
        is_fullscreen,
        color,
    ) {
        return;
    }

    draw_chrome_button_fallback_glyph(
        dst,
        stride,
        height,
        rect,
        button,
        is_shaded,
        is_fullscreen,
        color,
    );
}

fn draw_chrome_button_well(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    theme: UiTheme,
) {
    let x = rect.x as i32 + 1;
    let y = rect.y as i32;
    let w = rect.w.saturating_sub(2);
    let h = rect.h;
    if w < 8 || h < 8 {
        return;
    }

    fill_rect(dst, stride, x, y, w, h, theme.button_top);
}

fn draw_chrome_button_lucide(
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
    is_shaded: bool,
    is_fullscreen: bool,
    color: u32,
) -> bool {
    let Some(draw_svg_icon) = pistil_draw_svg_icon else {
        return false;
    };
    let Some(path) = lucide_icon_path(button, is_shaded, is_fullscreen) else {
        return false;
    };

    let icon_size = rect.w.min(rect.h).saturating_sub(10).clamp(12, 14);
    let dst_x = rect.x as i32 + ((rect.w.saturating_sub(icon_size)) / 2) as i32;
    let dst_y = rect.y as i32 + ((rect.h.saturating_sub(icon_size)) / 2) as i32;
    call_draw_svg_icon(
        draw_svg_icon,
        path,
        dst,
        stride,
        height,
        dst_x,
        dst_y,
        icon_size,
        icon_size,
        color,
    ) == 0
}

fn lucide_icon_path(
    button: ChromeButton,
    is_shaded: bool,
    is_fullscreen: bool,
) -> Option<&'static str> {
    match button {
        ChromeButton::Minimize => Some(WINDOW_ICON_MINIMIZE_PATH),
        ChromeButton::Shade => {
            if is_shaded {
                Some(WINDOW_ICON_UNSHADE_PATH)
            } else {
                Some(WINDOW_ICON_SHADE_PATH)
            }
        }
        ChromeButton::Maximize => Some(WINDOW_ICON_MAXIMIZE_PATH),
        ChromeButton::Fullscreen => {
            if is_fullscreen {
                Some(WINDOW_ICON_RESTORE_PATH)
            } else {
                Some(WINDOW_ICON_FULLSCREEN_PATH)
            }
        }
        ChromeButton::Close => Some(WINDOW_ICON_CLOSE_PATH),
    }
}

fn draw_chrome_button_fallback_glyph(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
    is_shaded: bool,
    is_fullscreen: bool,
    color: u32,
) {
    let x = rect.x as i32;
    let y = rect.y as i32;
    let w = rect.w as i32;
    let h = rect.h as i32;
    let cx = x + w / 2 + CHROME_ICON_X_BIAS;
    let cy = y + h / 2 + CHROME_ICON_Y_BIAS;

    match button {
        ChromeButton::Minimize => {
            let glyph_w = (w - 10).max(6);
            let glyph_h = 2;
            fill_rect_i32(
                dst,
                stride,
                height,
                cx - glyph_w / 2,
                cy - glyph_h / 2,
                glyph_w,
                glyph_h,
                color,
            );
        }
        ChromeButton::Shade => {
            // Draw a chevron: up if unshaded, down if shaded
            let arm = ((w - 10).max(6)) / 2;
            if is_shaded {
                // Chevron-down (V shape)
                for d in 0..=arm {
                    plot_thick_pixel(dst, stride, height, cx - arm + d, cy - arm / 2 + d, color, 1);
                    plot_thick_pixel(dst, stride, height, cx + arm - d, cy - arm / 2 + d, color, 1);
                }
            } else {
                // Chevron-up (^ shape)
                for d in 0..=arm {
                    plot_thick_pixel(dst, stride, height, cx - arm + d, cy + arm / 2 - d, color, 1);
                    plot_thick_pixel(dst, stride, height, cx + arm - d, cy + arm / 2 - d, color, 1);
                }
            }
        }
        ChromeButton::Maximize => {
            let box_w = (w - 10).max(7);
            let box_h = (h - 10).max(7);
            let bx = cx - box_w / 2;
            let by = cy - box_h / 2;
            fill_rect_i32(dst, stride, height, bx, by, box_w, 1, color);
            fill_rect_i32(dst, stride, height, bx, by, 1, box_h, color);
            fill_rect_i32(dst, stride, height, bx, by + box_h - 1, box_w, 1, color);
            fill_rect_i32(dst, stride, height, bx + box_w - 1, by, 1, box_h, color);
            if box_w > 4 && box_h > 4 {
                fill_rect_i32(dst, stride, height, bx + 2, by + 2, box_w - 4, 1, color);
            }
        }
        ChromeButton::Fullscreen => {
            let box_w = (w - 10).max(7);
            let box_h = (h - 10).max(7);
            let bx = cx - box_w / 2;
            let by = cy - box_h / 2;
            let clen = 3;
            if is_fullscreen {
                // Restore glyph: inward-pointing corners (minimize-2 style)
                fill_rect_i32(dst, stride, height, bx, by, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx, by + 1, 1, clen - 1, color);

                fill_rect_i32(dst, stride, height, bx + box_w - clen, by, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx + box_w - 1, by + 1, 1, clen - 1, color);

                fill_rect_i32(dst, stride, height, bx, by + box_h - 1, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx, by + box_h - clen, 1, clen - 1, color);

                fill_rect_i32(
                    dst,
                    stride,
                    height,
                    bx + box_w - clen,
                    by + box_h - 1,
                    clen,
                    1,
                    color,
                );
                fill_rect_i32(
                    dst,
                    stride,
                    height,
                    bx + box_w - 1,
                    by + box_h - clen,
                    1,
                    clen - 1,
                    color,
                );
            } else {
                // Expand glyph: outward-pointing corners
                fill_rect_i32(dst, stride, height, bx, by, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx, by, 1, clen, color);

                fill_rect_i32(dst, stride, height, bx + box_w - clen, by, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx + box_w - 1, by, 1, clen, color);

                fill_rect_i32(dst, stride, height, bx, by + box_h - 1, clen, 1, color);
                fill_rect_i32(dst, stride, height, bx, by + box_h - clen, 1, clen, color);

                fill_rect_i32(
                    dst,
                    stride,
                    height,
                    bx + box_w - clen,
                    by + box_h - 1,
                    clen,
                    1,
                    color,
                );
                fill_rect_i32(
                    dst,
                    stride,
                    height,
                    bx + box_w - 1,
                    by + box_h - clen,
                    1,
                    clen,
                    color,
                );
            }
        }
        ChromeButton::Close => {
            let radius = ((w.min(h) - 8) / 2).max(3);
            for d in -radius..=radius {
                plot_thick_pixel(dst, stride, height, cx + d, cy + d, color, 1);
                plot_thick_pixel(dst, stride, height, cx + d, cy - d, color, 1);
            }
        }
    }
}

fn plot_thick_pixel(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    color: u32,
    thickness: i32,
) {
    let half = thickness / 2;
    fill_rect_i32(
        dst,
        stride,
        height,
        x - half,
        y - half,
        thickness.max(1),
        thickness.max(1),
        color,
    );
}

fn blend_argb(base: u32, overlay: u32) -> u32 {
    let sa = (overlay >> 24) & 0xFF;
    if sa == 0 {
        return base;
    }
    if sa == 255 {
        return overlay;
    }

    let da = (base >> 24) & 0xFF;
    let inv_sa = 255 - sa;
    let out_a = sa + (da * inv_sa + 127) / 255;
    if out_a == 0 {
        return 0;
    }

    let sr = (overlay >> 16) & 0xFF;
    let sg = (overlay >> 8) & 0xFF;
    let sb = overlay & 0xFF;
    let dr = (base >> 16) & 0xFF;
    let dg = (base >> 8) & 0xFF;
    let db = base & 0xFF;

    let r = (sr * sa + (dr * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let g = (sg * sa + (dg * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let b = (sb * sa + (db * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    (out_a << 24) | (r.min(255) << 16) | (g.min(255) << 8) | b.min(255)
}

fn draw_pointer_overlay(
    dst: &mut [u32],
    width: u32,
    height: u32,
    pointer_x: i32,
    pointer_y: i32,
    cursor_sample: Option<&[u32]>,
    pistil_draw_text: Option<DrawTextFn>,
) {
    dst.fill(0);
    fill_rect(dst, width, 0, 0, width, height, 0xAA101820);
    fill_rect(dst, width, 0, 0, width, 2, 0xFF7DD3FC);
    fill_rect(dst, width, 0, height.saturating_sub(2) as i32, width, 2, 0x6659C3C3);

    draw_overlay_text(
        pistil_draw_text,
        dst,
        width,
        height,
        16,
        32,
        28.0,
        "BLOOM POINTER DEBUG",
        0xFFFFF4B0,
    );
    draw_overlay_text(
        pistil_draw_text,
        dst,
        width,
        height,
        18,
        68,
        22.0,
        "POINTER 0: X=",
        0xFFE6F7FF,
    );
    let next_x = draw_signed_number(
        pistil_draw_text,
        dst,
        width,
        height,
        18 + 13 * 11,
        68,
        pointer_x,
        0xFFFFFFFF,
    );
    let next_x = next_x + 18;
    draw_overlay_text(pistil_draw_text, dst, width, height, next_x, 68, 22.0, "Y=", 0xFFE6F7FF);
    draw_signed_number(
        pistil_draw_text,
        dst,
        width,
        height,
        next_x + 24,
        68,
        pointer_y,
        0xFFFFFFFF,
    );

    if let Some(cursor_sample) = cursor_sample {
        draw_cursor_sample(dst, width, height, cursor_sample);
    }
}

fn draw_cursor_sample(dst: &mut [u32], stride: u32, height: u32, cursor_sample: &[u32]) {
    if stride < CURSOR_SIZE || height < CURSOR_SIZE || cursor_sample.len() < CURSOR_PIXELS {
        return;
    }

    let x = stride as i32 - CURSOR_SIZE as i32 - POINTER_OVERLAY_CURSOR_INSET;
    let y = POINTER_OVERLAY_CURSOR_INSET;
    blit_nontransparent(
        dst,
        stride,
        height,
        cursor_sample,
        CURSOR_SIZE,
        CURSOR_SIZE,
        CURSOR_SIZE,
        x,
        y,
    );
}

fn blit_nontransparent(
    dst: &mut [u32],
    dst_stride: u32,
    dst_height: u32,
    src: &[u32],
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    dst_x: i32,
    dst_y: i32,
) {
    if dst_stride == 0 || src_stride < src_width {
        return;
    }

    for sy in 0..src_height {
        let dy = dst_y + sy as i32;
        if dy < 0 || dy >= dst_height as i32 {
            continue;
        }
        for sx in 0..src_width {
            let dx = dst_x + sx as i32;
            if dx < 0 || dx >= dst_stride as i32 {
                continue;
            }

            let src_px = src[(sy * src_stride + sx) as usize];
            if src_px >> 24 == 0 {
                continue;
            }
            dst[(dy as u32 * dst_stride + dx as u32) as usize] = src_px;
        }
    }
}

fn blit_argb_over(
    dst: &mut [u32],
    dst_stride: u32,
    dst_height: u32,
    src: &[u32],
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    dst_x: i32,
    dst_y: i32,
) {
    if dst_stride == 0 || src_stride < src_width {
        return;
    }

    for sy in 0..src_height {
        let dy = dst_y + sy as i32;
        if dy < 0 || dy >= dst_height as i32 {
            continue;
        }
        for sx in 0..src_width {
            let dx = dst_x + sx as i32;
            if dx < 0 || dx >= dst_stride as i32 {
                continue;
            }

            let src_px = src[(sy * src_stride + sx) as usize];
            if src_px >> 24 == 0 {
                continue;
            }
            let dst_idx = (dy as u32 * dst_stride + dx as u32) as usize;
            dst[dst_idx] = blend_argb(dst[dst_idx], src_px);
        }
    }
}

fn fill_rect_i32(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    if stride == 0 || w <= 0 || h <= 0 {
        return;
    }
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = x.saturating_add(w).max(0) as u32;
    let y1 = y.saturating_add(h).max(0) as u32;
    let x1 = x1.min(stride);
    let y1 = y1.min(height);
    for yy in y0..y1 {
        let row = (yy * stride) as usize;
        for xx in x0..x1 {
            dst[row + xx as usize] = color;
        }
    }
}

fn fill_rect(dst: &mut [u32], stride: u32, x: i32, y: i32, w: u32, h: u32, color: u32) {
    let height = if stride == 0 { 0 } else { (dst.len() as u32) / stride };
    fill_rect_i32(dst, stride, height, x, y, w as i32, h as i32, color);
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

fn draw_overlay_text(
    pistil_draw_text: Option<DrawTextFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    px_size: f32,
    text: &str,
    color: u32,
) -> i32 {
    if let Some(draw_text_fn) = pistil_draw_text {
        let mut text_c = [0u8; 128];
        let bytes = text.as_bytes();
        if bytes.len() < text_c.len() {
            text_c[..bytes.len()].copy_from_slice(bytes);
            let rc = draw_text_fn(
                text_c.as_ptr(),
                dst.as_mut_ptr(),
                stride,
                height,
                stride,
                x,
                y,
                px_size,
                color,
            );
            if rc == 0 {
                return x + (text.chars().count() as i32 * (px_size * 0.62) as i32);
            }
        }
    }

    let scale = ((px_size / 9.0) as u32).max(1);
    draw_text(dst, stride, height, x, y - (7 * scale) as i32, scale, text, color)
}

fn draw_overlay_text_bold(
    pistil_draw_text: Option<DrawTextFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    px_size: f32,
    text: &str,
    color: u32,
) -> i32 {
    draw_overlay_text(pistil_draw_text, dst, stride, height, x, y, px_size, text, color);
    draw_overlay_text(pistil_draw_text, dst, stride, height, x + 1, y, px_size, text, color)
}

fn draw_signed_number(
    pistil_draw_text: Option<DrawTextFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
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

    let text = core::str::from_utf8(&buf[..len]).unwrap_or("?");
    draw_overlay_text(pistil_draw_text, dst, stride, height, x, y, 22.0, text, color)
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

use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use pistil_types::Texture;

use crate::display::DisplayBackend;
use crate::scene::{
    ChromeButton, CompositionEntry, CursorKind, SurfaceChrome, chrome_button_rects,
};

const PISTIL_PATH: &str = "/lib/libpistil.so";
const PREPARE_BACKGROUND_SYMBOL: &[u8] = b"pistil_prepare_background";
const PREPARE_CURSOR_SYMBOL: &[u8] = b"pistil_prepare_cursor";
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
const DEFAULT_FONT_PATH: &str = "/share/fonts/NotoSans-Regular.ttf";
const SYMBOL_FONT_PATH: &str = "/share/fonts/NotoSansSymbol2-Regular.ttf";
const CURSOR_SIZE: u32 = 96;
const CURSOR_PIXELS: usize = (CURSOR_SIZE * CURSOR_SIZE) as usize;
const POINTER_OVERLAY_MAX_W: u32 = 460;
const POINTER_OVERLAY_MAX_H: u32 = 144;
const POINTER_OVERLAY_MARGIN: u32 = 12;
const POINTER_OVERLAY_CURSOR_INSET: i32 = 24;
const ACTIVE_CHROME: u32 = 0xFFFFB900;
const INACTIVE_CHROME: u32 = 0xFFA6984A;
const CHROME_TEXT: u32 = 0xFF32331F;
const CHROME_OUTLINE_DARK: u32 = 0xAA32331F;
const CHROME_OUTLINE_LIGHT: u32 = 0x66FFE07A;
const CHROME_ICON_MINIMIZE: &str = "\u{1F5D5}";
const CHROME_ICON_MAXIMIZE: &str = "\u{1F5D6}";
const CHROME_ICON_CLOSE: &str = "\u{1F5D9}";

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

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    cursor: Option<CursorBuffer>,
    cursor_variants: Vec<(CursorKind, CursorBuffer)>,
    pointer_overlay: Option<PointerOverlayBuffer>,
    chrome_overlay: Option<ChromeOverlayBuffer>,
    pistil: Option<PistilLib>,
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
    prepare_cursor: Option<PrepareCursorFn>,
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
            chrome_overlay: None,
            pistil,
        }
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
    ) -> Option<OverlayPlane> {
        if !composition.iter().any(|entry| !entry.chrome.is_empty()) {
            return None;
        }
        self.ensure_chrome_overlay(display)?;
        let overlay = self.chrome_overlay.as_mut()?;
        let draw_text = self.pistil.as_ref().and_then(|lib| lib.draw_text);
        let draw_symbol_text = self.pistil.as_ref().and_then(|lib| lib.draw_symbol_text);
        draw_chrome_overlay(
            overlay.texture.as_slice_mut(),
            overlay.width,
            overlay.height,
            composition,
            draw_text,
            draw_symbol_text,
        );
        Some(OverlayPlane {
            buffer_id: overlay.buffer_id,
            x: 0,
            y: 0,
            width: overlay.width,
            height: overlay.height,
        })
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

    fn ensure_chrome_overlay(&mut self, display: &DisplayBackend) -> Option<()> {
        let (width, height) = display.output_size();
        if matches!(
            self.chrome_overlay.as_ref(),
            Some(overlay) if overlay.width == width && overlay.height == height
        ) {
            return Some(());
        }
        let texture = Texture::new("bloom.compositor.chrome_overlay", width, height, 4)?;
        let buffer_id = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        )?;
        if let Some(old) = self.chrome_overlay.take() {
            display.release_buffer(old.buffer_id);
        }
        self.chrome_overlay = Some(ChromeOverlayBuffer { texture, buffer_id, width, height });
        stem::info!("bloom: chrome overlay ready buffer={} size={}x{}", buffer_id, width, height);
        Some(())
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
    Some(PistilLib { _handle: handle, prepare_bg, prepare_cursor, draw_text, draw_symbol_text })
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
    pistil_draw_text: Option<DrawTextFn>,
    pistil_draw_symbol_text: Option<DrawTextFn>,
) {
    dst.fill(0);
    for entry in composition {
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
        let chrome_color = if entry.active { ACTIVE_CHROME } else { INACTIVE_CHROME };

        let titlebar_height = chrome.titlebar_height.min(h);
        if titlebar_height > 0 {
            fill_rect(dst, stride, x, y, w, titlebar_height, chrome_color);
            draw_chrome_buttons(
                dst,
                stride,
                height,
                rect,
                chrome,
                entry.active,
                pistil_draw_symbol_text.or(pistil_draw_text),
            );
            if let Some(title) = entry.title.as_deref() {
                let text_x = x.saturating_add(frame as i32).saturating_add(12);
                let text_y = y.saturating_add(28);
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
                    .saturating_div(11)
                    .max(1) as usize;
                draw_overlay_text_bold(
                    pistil_draw_text,
                    dst,
                    stride,
                    height,
                    text_x,
                    text_y,
                    18.0,
                    title_prefix(title, max_chars),
                    CHROME_TEXT,
                );
            }
        } else {
            fill_rect(dst, stride, x, y, w, frame, chrome_color);
        }
        fill_rect(dst, stride, x, y, frame, h, chrome_color);
        fill_rect(
            dst,
            stride,
            x,
            y.saturating_add(h.saturating_sub(frame) as i32),
            w,
            frame,
            chrome_color,
        );
        fill_rect(
            dst,
            stride,
            x.saturating_add(w.saturating_sub(frame) as i32),
            y,
            frame,
            h,
            chrome_color,
        );
        draw_frame_outline(dst, stride, rect, frame);
        if chrome.titlebar_height > frame.saturating_mul(2)
            && chrome.titlebar_height < h.saturating_sub(frame)
        {
            let sep_y = y.saturating_add(chrome.titlebar_height as i32);
            fill_rect(dst, stride, x, sep_y, w, 1, CHROME_TEXT);
        }
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

fn draw_frame_outline(dst: &mut [u32], stride: u32, rect: abi::display_protocol::Rect, frame: u32) {
    let x = rect.x as i32;
    let y = rect.y as i32;
    let w = rect.w;
    let h = rect.h;
    if w == 0 || h == 0 {
        return;
    }

    fill_rect(dst, stride, x, y, w, 1, CHROME_OUTLINE_DARK);
    fill_rect(dst, stride, x, y, 1, h, CHROME_OUTLINE_DARK);
    fill_rect(
        dst,
        stride,
        x,
        y.saturating_add(h.saturating_sub(1) as i32),
        w,
        1,
        CHROME_OUTLINE_DARK,
    );
    fill_rect(
        dst,
        stride,
        x.saturating_add(w.saturating_sub(1) as i32),
        y,
        1,
        h,
        CHROME_OUTLINE_DARK,
    );

    if frame <= 1 || w <= frame.saturating_mul(2) || h <= frame.saturating_mul(2) {
        return;
    }

    let ix = x.saturating_add(frame as i32);
    let iy = y.saturating_add(frame as i32);
    let iw = w.saturating_sub(frame.saturating_mul(2));
    let ih = h.saturating_sub(frame.saturating_mul(2));
    fill_rect(dst, stride, ix, iy, iw, 1, CHROME_OUTLINE_LIGHT);
    fill_rect(dst, stride, ix, iy, 1, ih, CHROME_OUTLINE_LIGHT);
    fill_rect(
        dst,
        stride,
        ix,
        iy.saturating_add(ih.saturating_sub(1) as i32),
        iw,
        1,
        CHROME_OUTLINE_DARK,
    );
    fill_rect(
        dst,
        stride,
        ix.saturating_add(iw.saturating_sub(1) as i32),
        iy,
        1,
        ih,
        CHROME_OUTLINE_DARK,
    );
}

fn draw_chrome_buttons(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    surface_rect: abi::display_protocol::Rect,
    chrome: SurfaceChrome,
    active: bool,
    pistil_draw_symbol_text: Option<DrawTextFn>,
) {
    let Some(buttons) = chrome_button_rects(surface_rect, chrome) else {
        return;
    };

    for (button, rect) in buttons {
        draw_chrome_button(dst, stride, height, rect, button, active, pistil_draw_symbol_text);
    }
}

fn draw_chrome_button(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
    _active: bool,
    pistil_draw_symbol_text: Option<DrawTextFn>,
) {
    if rect.w < 8 || rect.h < 8 {
        return;
    }

    if !draw_chrome_button_symbol(
        pistil_draw_symbol_text,
        dst,
        stride,
        height,
        rect,
        button,
        CHROME_TEXT,
    ) {
        draw_chrome_button_fallback_glyph(dst, stride, height, rect, button);
    }
}

fn draw_chrome_button_symbol(
    pistil_draw_symbol_text: Option<DrawTextFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
    color: u32,
) -> bool {
    let Some(draw_text_fn) = pistil_draw_symbol_text else {
        return false;
    };

    let symbol = match button {
        ChromeButton::Minimize => CHROME_ICON_MINIMIZE,
        ChromeButton::Maximize => CHROME_ICON_MAXIMIZE,
        ChromeButton::Close => CHROME_ICON_CLOSE,
    };

    let mut text_c = [0u8; 16];
    let bytes = symbol.as_bytes();
    if bytes.len() >= text_c.len() {
        return false;
    }
    text_c[..bytes.len()].copy_from_slice(bytes);

    let scratch_w = rect.w.max(32);
    let scratch_h = rect.h.max(32);
    let scratch_len = scratch_w.saturating_mul(scratch_h) as usize;
    let mut scratch = Vec::new();
    if scratch.try_reserve_exact(scratch_len).is_err() {
        return false;
    }
    scratch.resize(scratch_len, 0);

    let px_size = ((rect.h.min(rect.w) as f32) * 0.62).clamp(14.0, 28.0);
    let draw_x = scratch_w as i32 / 2 - (px_size * 0.36) as i32;
    let draw_y = scratch_h as i32 / 2 + (px_size * 0.38) as i32;
    if draw_text_fn(
        text_c.as_ptr(),
        scratch.as_mut_ptr(),
        scratch_w,
        scratch_h,
        scratch_w,
        draw_x,
        draw_y,
        px_size,
        color,
    ) != 0
    {
        return false;
    }

    let Some(bounds) = nontransparent_bounds(&scratch, scratch_w, scratch_h, scratch_w) else {
        return false;
    };
    let (min_x, min_y, max_x, max_y) = bounds;
    let glyph_w = max_x.saturating_sub(min_x).saturating_add(1);
    let glyph_h = max_y.saturating_sub(min_y).saturating_add(1);
    let dst_x = rect.x as i32 + ((rect.w.saturating_sub(glyph_w)) / 2) as i32 - min_x as i32;
    let dst_y = rect.y as i32 + ((rect.h.saturating_sub(glyph_h)) / 2) as i32 - min_y as i32;
    blit_argb_over(dst, stride, height, &scratch, scratch_w, scratch_h, scratch_w, dst_x, dst_y);
    true
}

fn draw_chrome_button_fallback_glyph(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    button: ChromeButton,
) {
    let x = rect.x as i32;
    let y = rect.y as i32;
    let w = rect.w as i32;
    let h = rect.h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;

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
                CHROME_TEXT,
            );
        }
        ChromeButton::Maximize => {
            let box_w = (w - 10).max(7);
            let box_h = (h - 10).max(7);
            let bx = cx - box_w / 2;
            let by = cy - box_h / 2;
            fill_rect_i32(dst, stride, height, bx, by, box_w, 1, CHROME_TEXT);
            fill_rect_i32(dst, stride, height, bx, by, 1, box_h, CHROME_TEXT);
            fill_rect_i32(dst, stride, height, bx, by + box_h - 1, box_w, 1, CHROME_TEXT);
            fill_rect_i32(dst, stride, height, bx + box_w - 1, by, 1, box_h, CHROME_TEXT);
            if box_w > 4 && box_h > 4 {
                fill_rect_i32(dst, stride, height, bx + 2, by + 2, box_w - 4, 1, CHROME_TEXT);
            }
        }
        ChromeButton::Close => {
            let radius = ((w.min(h) - 8) / 2).max(3);
            for d in -radius..=radius {
                plot_thick_pixel(dst, stride, height, cx + d, cy + d, CHROME_TEXT, 1);
                plot_thick_pixel(dst, stride, height, cx + d, cy - d, CHROME_TEXT, 1);
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
    let oa = (overlay >> 24) & 0xFF;
    if oa == 0 {
        return base;
    }

    let br = (base >> 16) & 0xFF;
    let bg = (base >> 8) & 0xFF;
    let bb = base & 0xFF;

    let or = (overlay >> 16) & 0xFF;
    let og = (overlay >> 8) & 0xFF;
    let ob = overlay & 0xFF;

    let r = (br * (255 - oa) + or * oa) / 255;
    let g = (bg * (255 - oa) + og * oa) / 255;
    let b = (bb * (255 - oa) + ob * oa) / 255;
    (0xFF << 24) | (r << 16) | (g << 8) | b
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
        22.0,
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
        18.0,
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
    draw_overlay_text(pistil_draw_text, dst, width, height, next_x, 68, 18.0, "Y=", 0xFFE6F7FF);
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

fn nontransparent_bounds(
    pixels: &[u32],
    width: u32,
    height: u32,
    stride: u32,
) -> Option<(u32, u32, u32, u32)> {
    if width == 0 || height == 0 || stride < width {
        return None;
    }

    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..height {
        for x in 0..width {
            let idx = (y * stride + x) as usize;
            if pixels.get(idx).copied().unwrap_or(0) >> 24 == 0 {
                continue;
            }
            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    if found { Some((min_x, min_y, max_x, max_y)) } else { None }
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
    let end = draw_overlay_text(pistil_draw_text, dst, stride, height, x, y, px_size, text, color);
    draw_overlay_text(
        pistil_draw_text,
        dst,
        stride,
        height,
        x.saturating_add(1),
        y,
        px_size,
        text,
        color,
    );
    end
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
    draw_overlay_text(pistil_draw_text, dst, stride, height, x, y, 18.0, text, color)
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

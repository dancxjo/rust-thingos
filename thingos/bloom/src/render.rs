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
const DEFAULT_FONT_READY_SYMBOL: &[u8] = b"pistil_default_font_ready";
const SYMBOL_FONT_READY_SYMBOL: &[u8] = b"pistil_symbol_font_ready";
const DEFAULT_CURSOR_PATH: &str = "/public/cursors/future/default.svg";
const MOVE_CURSOR_PATH: &str = "/public/cursors/future/fleur.svg";
const RESIZE_N_CURSOR_PATH: &str = "/public/cursors/future/top_side.svg";
const RESIZE_S_CURSOR_PATH: &str = "/public/cursors/future/bottom_side.svg";
const RESIZE_E_CURSOR_PATH: &str = "/public/cursors/future/right_side.svg";
const RESIZE_W_CURSOR_PATH: &str = "/public/cursors/future/left_side.svg";
const RESIZE_NE_CURSOR_PATH: &str = "/public/cursors/future/top_right_corner.svg";
const RESIZE_NW_CURSOR_PATH: &str = "/public/cursors/future/top_left_corner.svg";
const RESIZE_SE_CURSOR_PATH: &str = "/public/cursors/future/bottom_right_corner.svg";
const RESIZE_SW_CURSOR_PATH: &str = "/public/cursors/future/bottom_left_corner.svg";
const WINDOW_ICON_MINIMIZE_PATH: &str = "/public/icons/lucide/minus.svg";
const WINDOW_ICON_SHADE_PATH: &str = "/public/icons/lucide/chevron-up.svg";
const WINDOW_ICON_UNSHADE_PATH: &str = "/public/icons/lucide/chevron-down.svg";
const WINDOW_ICON_MAXIMIZE_PATH: &str = "/public/icons/lucide/square.svg";
const WINDOW_ICON_FULLSCREEN_PATH: &str = "/public/icons/lucide/fullscreen.svg";
const WINDOW_ICON_RESTORE_PATH: &str = "/public/icons/lucide/minimize-2.svg";
const WINDOW_ICON_CLOSE_PATH: &str = "/public/icons/lucide/x.svg";
const DEFAULT_FONT_PATH: &str = "/public/fonts/Inter-Regular.ttf";
const SYMBOL_FONT_PATH: &str = "/public/fonts/NotoSansSymbol2-Regular.ttf";
const CURSOR_SIZE: u32 = 48;
const CURSOR_PIXELS: usize = (CURSOR_SIZE * CURSOR_SIZE) as usize;
const BUSY_CURSOR_MASK_SIZE: u32 = 24;
const BUSY_CURSOR_MASK_STRIDE: usize = 3;
const BUSY_CURSOR_SCALE: u32 = CURSOR_SIZE / BUSY_CURSOR_MASK_SIZE;
const BUSY_CURSOR_HOTSPOT_X: u32 = CURSOR_SIZE / 2;
const BUSY_CURSOR_HOTSPOT_Y: u32 = CURSOR_SIZE / 2;
const BUSY_CURSOR_FRAME: u32 = 0xFF15171F;
const BUSY_CURSOR_FRAME_LIGHT: u32 = 0xFFFFFFFF;
const BUSY_CURSOR_SAND: u32 = 0xFFD99A2B;
const BUSY_CURSOR_SAND_DARK: u32 = 0xFF7A4C15;
const BUSY_CURSOR_SHADOW: u32 = 0x88000000;
const BUSY_CURSOR_MASK: [u8; (BUSY_CURSOR_MASK_SIZE as usize) * BUSY_CURSOR_MASK_STRIDE] = [
    0b11111111, 0b11111111, 0b11111111, 0b10000000, 0b00000000, 0b00000001, 0b01000000, 0b00000000,
    0b00000010, 0b00111111, 0b11111111, 0b11111100, 0b00010000, 0b00000000, 0b00001000, 0b00001000,
    0b00000000, 0b00010000, 0b00000100, 0b00000000, 0b00100000, 0b00000010, 0b00000000, 0b01000000,
    0b00000001, 0b00000000, 0b10000000, 0b00000000, 0b10000001, 0b00000000, 0b00000000, 0b01000010,
    0b00000000, 0b00000000, 0b00100100, 0b00000000, 0b00000000, 0b00100100, 0b00000000, 0b00000000,
    0b01000010, 0b00000000, 0b00000000, 0b10000001, 0b00000000, 0b00000001, 0b00000000, 0b10000000,
    0b00000010, 0b00000000, 0b01000000, 0b00000100, 0b00000000, 0b00100000, 0b00001000, 0b00000000,
    0b00010000, 0b00010000, 0b00000000, 0b00001000, 0b00111111, 0b11111111, 0b11111100, 0b01000000,
    0b00000000, 0b00000010, 0b10000000, 0b00000000, 0b00000001, 0b11111111, 0b11111111, 0b11111111,
];
const POINTER_OVERLAY_MAX_W: u32 = 460;
const POINTER_OVERLAY_MAX_H: u32 = 144;
const POINTER_OVERLAY_MARGIN: u32 = 12;
const POINTER_OVERLAY_CURSOR_INSET: i32 = 12;
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
type FontReadyFn = extern "C" fn() -> i32;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    /// Path of the wallpaper that is currently loaded as `background`.
    ///
    /// `None` means no wallpaper has been successfully loaded yet (or the
    /// compositor has only a solid-colour fallback).  Used to skip redundant
    /// `prepare_background` calls when the requested path has not changed.
    wallpaper_path: Option<alloc::string::String>,
    cursor: Option<CursorBuffer>,
    cursor_variants: Vec<(CursorKind, CursorBuffer)>,
    pointer_overlay: Option<PointerOverlayBuffer>,
    body_overlays: Vec<ChromeOverlayBuffer>,
    chrome_overlays: Vec<ChromeOverlayBuffer>,
    pistil: Option<PistilLib>,
    default_font_ready: bool,
    symbol_font_ready: bool,
    theme: UiTheme,
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
    prepare_cursor: Option<PrepareCursorFn>,
    draw_svg_icon: Option<DrawSvgIconFn>,
    draw_text: Option<DrawTextFn>,
    draw_symbol_text: Option<DrawTextFn>,
    default_font_ready: Option<FontReadyFn>,
    symbol_font_ready: Option<FontReadyFn>,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

#[derive(Clone, Copy)]
pub struct CursorPlane {
    pub buffer_id: u32,
    /// Top-left X of the cursor image in screen coordinates (= pointer_x - hotspot_x).
    pub x: i32,
    /// Top-left Y of the cursor image in screen coordinates (= pointer_y - hotspot_y).
    pub y: i32,
    pub width: u32,
    pub height: u32,
    /// Horizontal hotspot offset within the cursor image.
    pub hotspot_x: u32,
    /// Vertical hotspot offset within the cursor image.
    pub hotspot_y: u32,
}

#[derive(Clone, Copy)]
pub struct OverlayPlane {
    pub buffer_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct WindowOverlayPlane {
    pub surface_id: u32,
    pub buffer_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct ResourceRetryStatus {
    pub pending: bool,
    pub improved: bool,
}

struct CursorBuffer {
    _texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
    hotspot_x: u32,
    hotspot_y: u32,
    fallback: bool,
}

struct PointerOverlayBuffer {
    texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

struct ChromeOverlayBuffer {
    surface_id: u32,
    texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        Self {
            background: None,
            wallpaper_path: None,
            cursor: None,
            cursor_variants: Vec::new(),
            pointer_overlay: None,
            body_overlays: Vec::new(),
            chrome_overlays: Vec::new(),
            pistil: None,
            default_font_ready: false,
            symbol_font_ready: false,
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
        // Clear the wallpaper path because this is a solid-colour background,
        // not a file-backed wallpaper.  `prepare_background` sets
        // `wallpaper_path` after a successful file-based load.
        self.wallpaper_path = None;
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
            stem::info!("bloom: using facet-frame fallback background");
            texture.as_slice_mut().fill(0xFF0B0A10);
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
        if success {
            // Record the path only when the file was decoded successfully so
            // a re-request for the same path after a decode failure is retried.
            self.wallpaper_path = Some(alloc::string::String::from(wallpaper_path));
        }
    }

    /// Decode and import a wallpaper after the service loop is live.
    ///
    /// This intentionally runs on the compositor thread for now. The current
    /// user allocator is fragile under concurrent wallpaper decoding, and the
    /// bundled BMP assets are small enough that an inline decode is safer than
    /// risking heap corruption during startup.
    ///
    /// The call is a no-op when the requested path matches the path of the
    /// wallpaper that is already loaded, avoiding redundant decode+import work
    /// when the watch service fires but the path has not changed.
    ///
    /// `wallpaper_path` is only set after a successful `prepare_background`
    /// call, so a `Some` value here implies the background buffer is loaded.
    pub fn start_background_load(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        if self.wallpaper_path.as_deref() == Some(wallpaper_path) {
            stem::debug!("bloom: wallpaper '{}' already loaded, skipping reload", wallpaper_path);
            return;
        }
        self.prepare_background(display, wallpaper_path);
    }

    /// Background loads are currently completed inline on the compositor
    /// thread, so there is no pending worker result to poll.
    pub fn poll_ready_background(&mut self, _display: &DisplayBackend) -> bool {
        false
    }

    pub fn retry_deferred_resources(
        &mut self,
        display: &DisplayBackend,
        wallpaper_path: Option<&str>,
        load_cursor: bool,
    ) -> ResourceRetryStatus {
        let mut improved = false;

        if self.pistil.is_none() {
            self.pistil = load_pistil(false);
            if self.pistil.is_some() {
                stem::info!("bloom: deferred pistil renderer became available");
                improved = true;
            }
        }

        let mut font_pending = false;
        if let Some(ref lib) = self.pistil {
            if lib.draw_text.is_some() {
                match lib.default_font_ready {
                    Some(ready) if ready() != 0 => {
                        if !self.default_font_ready {
                            stem::info!("bloom: deferred default font became available");
                            improved = true;
                        }
                        self.default_font_ready = true;
                    }
                    Some(_) => font_pending = true,
                    None => self.default_font_ready = true,
                }
            }
            if lib.draw_symbol_text.is_some() {
                match lib.symbol_font_ready {
                    Some(ready) if ready() != 0 => {
                        if !self.symbol_font_ready {
                            stem::info!("bloom: deferred symbol font became available");
                            improved = true;
                        }
                        self.symbol_font_ready = true;
                    }
                    Some(_) => font_pending = true,
                    None => self.symbol_font_ready = true,
                }
            }
        }

        if let Some(path) = wallpaper_path {
            if self.pistil.is_some() && self.wallpaper_path.as_deref() != Some(path) {
                let before = self.wallpaper_path.clone();
                self.start_background_load(display, path);
                if self.wallpaper_path.as_deref() == Some(path) && before.as_deref() != Some(path) {
                    stem::info!("bloom: deferred wallpaper became available: {}", path);
                    improved = true;
                }
            }
        }

        if load_cursor && self.pistil.is_some() && self.needs_asset_cursor() {
            self.prepare_cursor(display);
            if !self.needs_asset_cursor() {
                improved = true;
            }
        }

        let wallpaper_pending = wallpaper_path
            .map(|path| self.wallpaper_path.as_deref() != Some(path))
            .unwrap_or(false);
        let cursor_pending = load_cursor && self.needs_asset_cursor();
        let pending = self.pistil.is_none() || font_pending || wallpaper_pending || cursor_pending;

        ResourceRetryStatus { pending, improved }
    }

    pub fn reconfigure_for_output(&mut self, display: &DisplayBackend) {
        if let Some(path) = self.wallpaper_path.clone() {
            self.prepare_background(display, &path);
        } else {
            self.prepare_solid_background(display, 0xFF0B0A10);
        }
        release_overlay_buffers(display, &mut self.body_overlays);
        release_overlay_buffers(display, &mut self.chrome_overlays);
        if let Some(old) = self.pointer_overlay.take() {
            display.release_buffer(old.buffer_id);
        }
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }

    pub fn prepare_busy_cursor(&mut self, display: &DisplayBackend) {
        if self.cursor.is_some() {
            return;
        }

        let mut texture =
            match Texture::new("bloom.compositor.cursor.busy", CURSOR_SIZE, CURSOR_SIZE, 4) {
                Some(t) => t,
                None => {
                    stem::warn!("bloom: failed to allocate built-in busy cursor texture");
                    return;
                }
            };
        let stride_pixels = texture.stride / 4;
        let height = texture.height;
        draw_builtin_busy_cursor(texture.as_slice_mut(), stride_pixels, height);

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            texture.width,
            texture.height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
            0,
        ) else {
            stem::warn!("bloom: failed to import built-in busy cursor texture");
            return;
        };

        self.cursor = Some(CursorBuffer {
            _texture: texture,
            buffer_id,
            width: CURSOR_SIZE,
            height: CURSOR_SIZE,
            hotspot_x: BUSY_CURSOR_HOTSPOT_X,
            hotspot_y: BUSY_CURSOR_HOTSPOT_Y,
            fallback: true,
        });
        stem::info!(
            "bloom: built-in busy cursor ready buffer={} size={}x{} hotspot={},{}",
            buffer_id,
            CURSOR_SIZE,
            CURSOR_SIZE,
            BUSY_CURSOR_HOTSPOT_X,
            BUSY_CURSOR_HOTSPOT_Y
        );
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
            fallback: false,
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

    fn needs_asset_cursor(&self) -> bool {
        self.cursor.as_ref().map(|cursor| cursor.fallback).unwrap_or(true)
    }

    pub fn cursor_is_fallback(&self) -> bool {
        self.cursor.as_ref().map(|cursor| cursor.fallback).unwrap_or(false)
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
            hotspot_x: cursor.hotspot_x,
            hotspot_y: cursor.hotspot_y,
        })
    }

    pub fn chrome_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        composition: &[CompositionEntry],
        pointer_x: i32,
        pointer_y: i32,
        primary_button_down: bool,
    ) -> (Vec<WindowOverlayPlane>, Vec<WindowOverlayPlane>) {
        if !composition.iter().any(needs_window_overlay) {
            release_overlay_buffers(display, &mut self.body_overlays);
            release_overlay_buffers(display, &mut self.chrome_overlays);
            return (Vec::new(), Vec::new());
        }
        self.ensure_overlay_buffers(display, composition);

        let draw_svg_icon = self.pistil.as_ref().and_then(|lib| lib.draw_svg_icon);
        let draw_text = self.pistil.as_ref().and_then(|lib| lib.draw_text);
        let draw_symbol_text = self.pistil.as_ref().and_then(|lib| lib.draw_symbol_text);

        let mut body_planes = Vec::new();
        let mut chrome_planes = Vec::new();

        for entry in composition.iter().filter(|entry| needs_window_overlay(entry)) {
            let visual_rect = crate::scene::surface_visual_rect(entry.dest_rect, entry.chrome);
            let local_entry = local_overlay_entry(entry, visual_rect);
            if let Some(body) =
                self.body_overlays.iter_mut().find(|buffer| buffer.surface_id == entry.surface_id)
            {
                draw_window_body_overlay(
                    body.texture.as_slice_mut(),
                    body.width,
                    body.height,
                    core::slice::from_ref(&local_entry),
                    self.theme,
                );
                body_planes.push(WindowOverlayPlane {
                    surface_id: entry.surface_id,
                    buffer_id: body.buffer_id,
                    x: visual_rect.x as i32,
                    y: visual_rect.y as i32,
                    width: body.width,
                    height: body.height,
                });
            }

            let Some(chrome) = self
                .chrome_overlays
                .iter_mut()
                .find(|buffer| buffer.surface_id == entry.surface_id)
            else {
                continue;
            };
            let local_pointer_x = pointer_x.saturating_sub(visual_rect.x as i32);
            let local_pointer_y = pointer_y.saturating_sub(visual_rect.y as i32);
            draw_chrome_overlay(
                chrome.texture.as_slice_mut(),
                chrome.width,
                chrome.height,
                core::slice::from_ref(&local_entry),
                draw_svg_icon,
                draw_text,
                draw_symbol_text,
                self.theme,
                local_pointer_x,
                local_pointer_y,
                primary_button_down,
            );
            chrome_planes.push(WindowOverlayPlane {
                surface_id: entry.surface_id,
                buffer_id: chrome.buffer_id,
                x: visual_rect.x as i32,
                y: visual_rect.y as i32,
                width: chrome.width,
                height: chrome.height,
            });
        }

        (body_planes, chrome_planes)
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
        if self.prepare_cursor_variant(display, kind).is_none() {
            return self.cursor.as_ref();
        }
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
                fallback: false,
            },
        ));
        stem::info!("bloom: cursor {:?} ready from {} buffer={}", kind, cursor_path, buffer_id);
        Some(())
    }

    fn ensure_overlay_buffers(
        &mut self,
        display: &DisplayBackend,
        composition: &[CompositionEntry],
    ) {
        sync_overlay_set(
            display,
            &mut self.body_overlays,
            composition,
            "bloom.compositor.window_body",
        );
        sync_overlay_set(
            display,
            &mut self.chrome_overlays,
            composition,
            "bloom.compositor.window_chrome",
        );
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

fn load_pistil(log_failures: bool) -> Option<PistilLib> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        if log_failures {
            log_dlerror("bloom: failed to load /lib/libpistil.so");
        }
        return None;
    }

    let sym = dlsym_bytes(handle, PREPARE_BACKGROUND_SYMBOL);
    if sym.is_null() {
        if log_failures {
            log_dlerror("bloom: failed to resolve pistil_prepare_background");
        }
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
    let default_font_ready_sym = dlsym_bytes(handle, DEFAULT_FONT_READY_SYMBOL);
    let default_font_ready = if default_font_ready_sym.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute(default_font_ready_sym) })
    };
    let symbol_font_ready_sym = dlsym_bytes(handle, SYMBOL_FONT_READY_SYMBOL);
    let symbol_font_ready = if symbol_font_ready_sym.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute(symbol_font_ready_sym) })
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
        default_font_ready,
        symbol_font_ready,
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

fn draw_builtin_busy_cursor(dst: &mut [u32], stride: u32, height: u32) {
    dst.fill(0);
    if stride < CURSOR_SIZE || height < CURSOR_SIZE {
        return;
    }

    for y in 0..BUSY_CURSOR_MASK_SIZE {
        for x in 0..BUSY_CURSOR_MASK_SIZE {
            if busy_cursor_mask_bit(x, y) {
                draw_scaled_cursor_pixel(dst, stride, x + 1, y + 1, BUSY_CURSOR_SHADOW);
            }
        }
    }

    for y in 5..11 {
        let radius = 11u32.saturating_sub(y);
        for x in BUSY_CURSOR_MASK_SIZE / 2 - radius..=BUSY_CURSOR_MASK_SIZE / 2 + radius {
            draw_scaled_cursor_pixel(dst, stride, x, y, BUSY_CURSOR_SAND);
        }
    }
    for y in 14..21 {
        let radius = (y - 14).min(7);
        for x in BUSY_CURSOR_MASK_SIZE / 2 - radius..=BUSY_CURSOR_MASK_SIZE / 2 + radius {
            let color = if y >= 19 { BUSY_CURSOR_SAND_DARK } else { BUSY_CURSOR_SAND };
            draw_scaled_cursor_pixel(dst, stride, x, y, color);
        }
    }
    for y in 10..15 {
        draw_scaled_cursor_pixel(dst, stride, BUSY_CURSOR_MASK_SIZE / 2, y, BUSY_CURSOR_SAND);
    }

    for y in 0..BUSY_CURSOR_MASK_SIZE {
        for x in 0..BUSY_CURSOR_MASK_SIZE {
            if busy_cursor_mask_bit(x, y) {
                let color = if y <= 2 || (x <= 2 && y < BUSY_CURSOR_MASK_SIZE - 2) {
                    BUSY_CURSOR_FRAME_LIGHT
                } else {
                    BUSY_CURSOR_FRAME
                };
                draw_scaled_cursor_pixel(dst, stride, x, y, color);
            }
        }
    }
}

fn busy_cursor_mask_bit(x: u32, y: u32) -> bool {
    if x >= BUSY_CURSOR_MASK_SIZE || y >= BUSY_CURSOR_MASK_SIZE {
        return false;
    }
    let row = y as usize * BUSY_CURSOR_MASK_STRIDE;
    let byte = BUSY_CURSOR_MASK[row + (x as usize / 8)];
    let bit = 7 - (x % 8);
    (byte & (1 << bit)) != 0
}

fn draw_scaled_cursor_pixel(dst: &mut [u32], stride: u32, x: u32, y: u32, color: u32) {
    let dst_x = x * BUSY_CURSOR_SCALE;
    let dst_y = y * BUSY_CURSOR_SCALE;
    if dst_x >= CURSOR_SIZE || dst_y >= CURSOR_SIZE {
        return;
    }
    for yy in 0..BUSY_CURSOR_SCALE {
        let row = ((dst_y + yy) * stride) as usize;
        for xx in 0..BUSY_CURSOR_SCALE {
            dst[row + (dst_x + xx) as usize] = color;
        }
    }
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
    surface_id: u32,
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
    Some(ChromeOverlayBuffer { surface_id, texture, buffer_id, width, height })
}

fn release_overlay_buffers(display: &DisplayBackend, buffers: &mut Vec<ChromeOverlayBuffer>) {
    for buffer in buffers.drain(..) {
        display.release_buffer(buffer.buffer_id);
    }
}

fn sync_overlay_set(
    display: &DisplayBackend,
    buffers: &mut Vec<ChromeOverlayBuffer>,
    composition: &[CompositionEntry],
    name: &'static str,
) {
    let mut idx = 0;
    while idx < buffers.len() {
        let keep = overlay_size_for(composition, buffers[idx].surface_id)
            .map(|(w, h)| buffers[idx].width == w && buffers[idx].height == h)
            .unwrap_or(false);
        if keep {
            idx += 1;
        } else {
            let old = buffers.swap_remove(idx);
            display.release_buffer(old.buffer_id);
        }
    }

    for entry in composition.iter().filter(|entry| needs_window_overlay(entry)) {
        let Some((width, height)) = overlay_size_for(composition, entry.surface_id) else {
            continue;
        };
        if buffers.iter().any(|buffer| buffer.surface_id == entry.surface_id) {
            continue;
        }
        if let Some(buffer) = make_overlay_buffer(display, name, entry.surface_id, width, height) {
            stem::info!(
                "bloom: flat window overlays ready surface={} size={}x{}",
                entry.surface_id,
                width,
                height
            );
            buffers.push(buffer);
        }
    }
}

fn overlay_size_for(composition: &[CompositionEntry], surface_id: u32) -> Option<(u32, u32)> {
    let entry = composition
        .iter()
        .find(|entry| entry.surface_id == surface_id && needs_window_overlay(entry))?;
    let rect = crate::scene::surface_visual_rect(entry.dest_rect, entry.chrome);
    if rect.w == 0 || rect.h == 0 { None } else { Some((rect.w, rect.h)) }
}

fn needs_window_overlay(entry: &CompositionEntry) -> bool {
    !entry.is_fullscreen && !entry.chrome.is_empty()
}

fn local_overlay_entry(
    entry: &CompositionEntry,
    visual_rect: abi::display_protocol::Rect,
) -> CompositionEntry {
    let mut local = entry.clone();
    local.dest_rect = abi::display_protocol::Rect {
        x: entry.dest_rect.x.saturating_sub(visual_rect.x),
        y: entry.dest_rect.y.saturating_sub(visual_rect.y),
        w: entry.dest_rect.w,
        h: entry.dest_rect.h,
    };
    local
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
    pointer_x: i32,
    pointer_y: i32,
    primary_button_down: bool,
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

        draw_facet_window_frame(
            dst,
            stride,
            height,
            rect,
            frame,
            titlebar_height,
            entry.active,
            theme,
        );

        if titlebar_height > 0 {
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
                pointer_x,
                pointer_y,
                primary_button_down,
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

        draw_contact_shadow(dst, stride, height, rect, theme);
        fill_rect(dst, stride, body_x, body_y, body_w, body_h, theme.body_top);
        draw_content_field(dst, stride, height, body_x, body_y, body_w, body_h, theme);
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

fn draw_facet_window_frame(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    frame: u32,
    titlebar_height: u32,
    active: bool,
    theme: UiTheme,
) {
    let x = rect.x as i32;
    let y = rect.y as i32;
    let w = rect.w;
    let h = rect.h;
    if w == 0 || h == 0 || frame == 0 {
        return;
    }

    let frame = frame.min(w / 2).min(h / 2);
    let fill_top = if active { theme.frame_fill } else { theme.frame_fill_inactive };
    let fill_bottom =
        if active { theme.frame_fill_bottom } else { theme.frame_fill_bottom_inactive };
    let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
    let facet = if active { theme.facet } else { theme.facet_inactive };
    let state = if active { theme.active } else { theme.inactive };

    if titlebar_height > 0 {
        fill_vertical_gradient(
            dst,
            stride,
            height,
            x,
            y,
            w,
            titlebar_height,
            state.title_top,
            state.title_bottom,
        );
        if active && titlebar_height > 6 && w > frame.saturating_mul(2) {
            fill_horizontal_gradient(
                dst,
                stride,
                height,
                x + frame as i32,
                y + 2,
                w.saturating_sub(frame.saturating_mul(2)),
                2,
                state.title_top,
                theme.title_sheen,
                state.title_top,
            );
        }
    } else {
        fill_vertical_gradient(dst, stride, height, x, y, w, frame, fill_top, fill_bottom);
    }
    fill_vertical_gradient(dst, stride, height, x, y, frame, h, fill_top, fill_bottom);
    fill_vertical_gradient(
        dst,
        stride,
        height,
        x + w.saturating_sub(frame) as i32,
        y,
        frame,
        h,
        fill_top,
        fill_bottom,
    );
    fill_horizontal_gradient(
        dst,
        stride,
        height,
        x,
        y + h.saturating_sub(frame) as i32,
        w,
        frame,
        fill_bottom,
        fill_top,
        fill_bottom,
    );

    draw_rect_stroke(dst, stride, height, x, y, w, h, 2, theme.outer_stroke);
    draw_frame_bevel(dst, stride, height, x, y, w, h, frame, active, theme);

    if titlebar_height > 0 && titlebar_height < h {
        let sep_y = y.saturating_add(titlebar_height as i32).saturating_sub(1);
        let rule = if active { theme.title_rule_active } else { theme.title_rule_inactive };
        fill_rect_i32(
            dst,
            stride,
            height,
            x + frame as i32,
            sep_y.saturating_sub(1),
            w.saturating_sub(frame * 2) as i32,
            1,
            if active { theme.frame_bevel_light } else { theme.frame_bevel_light_inactive },
        );
        fill_rect_i32(
            dst,
            stride,
            height,
            x + frame as i32,
            sep_y,
            w.saturating_sub(frame * 2) as i32,
            1,
            rule,
        );
    }

    let body_y = y.saturating_add(titlebar_height as i32);
    let body_h = h.saturating_sub(titlebar_height).saturating_sub(frame);
    if body_h > 0 && w > frame.saturating_mul(2) {
        let inner_x = x + frame as i32 - 1;
        let inner_right = x + w.saturating_sub(frame) as i32;
        let inner_bottom = y + h.saturating_sub(frame) as i32;
        fill_rect_i32(dst, stride, height, inner_x, body_y, 1, body_h as i32, inner);
        fill_rect_i32(dst, stride, height, inner_right, body_y, 1, body_h as i32, inner);
        fill_rect_i32(
            dst,
            stride,
            height,
            x + frame as i32,
            inner_bottom,
            w.saturating_sub(frame * 2) as i32,
            1,
            inner,
        );
    }

    draw_corner_facets(dst, stride, height, x, y, w, h, facet);

    if active {
        draw_top_glow_strip(dst, stride, height, x + 2, y, w.saturating_sub(4), theme);
        fill_rect_i32(
            dst,
            stride,
            height,
            x + w.saturating_sub(4) as i32,
            y + 2,
            2,
            2,
            theme.focus_accent,
        );
        fill_rect_i32(
            dst,
            stride,
            height,
            x + frame as i32,
            y + h.saturating_sub(frame).saturating_sub(2) as i32,
            2,
            2,
            theme.focus_accent,
        );
    }
}

fn draw_frame_bevel(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    frame: u32,
    active: bool,
    theme: UiTheme,
) {
    if w < 6 || h < 6 || frame == 0 {
        return;
    }

    let light = if active { theme.frame_bevel_light } else { theme.frame_bevel_light_inactive };
    let shadow = theme.frame_bevel_shadow;
    let mid = if active { theme.edge_light } else { theme.inner_stroke_inactive };

    let inner_x = x + frame.min(w / 2) as i32;
    let inner_y = y + frame.min(h / 2) as i32;
    let inner_w = w.saturating_sub(frame.saturating_mul(2));
    let inner_h = h.saturating_sub(frame.saturating_mul(2));

    fill_rect_i32(dst, stride, height, x + 2, y + 2, w.saturating_sub(4) as i32, 1, light);
    fill_rect_i32(dst, stride, height, x + 2, y + 2, 1, h.saturating_sub(4) as i32, mid);
    fill_rect_i32(
        dst,
        stride,
        height,
        x + 2,
        y + h.saturating_sub(3) as i32,
        w.saturating_sub(4) as i32,
        1,
        shadow,
    );
    fill_rect_i32(
        dst,
        stride,
        height,
        x + w.saturating_sub(3) as i32,
        y + 2,
        1,
        h.saturating_sub(4) as i32,
        shadow,
    );

    if inner_w > 4 && inner_h > 4 {
        fill_rect_i32(dst, stride, height, inner_x, inner_y, inner_w as i32, 1, shadow);
        fill_rect_i32(dst, stride, height, inner_x, inner_y, 1, inner_h as i32, shadow);
        fill_rect_i32(
            dst,
            stride,
            height,
            inner_x,
            inner_y + inner_h.saturating_sub(1) as i32,
            inner_w as i32,
            1,
            light,
        );
        fill_rect_i32(
            dst,
            stride,
            height,
            inner_x + inner_w.saturating_sub(1) as i32,
            inner_y,
            1,
            inner_h as i32,
            light,
        );
    }
}

fn draw_rect_stroke(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    thickness: u32,
    color: u32,
) {
    if w == 0 || h == 0 || thickness == 0 {
        return;
    }
    let t = thickness.min(w / 2).min(h / 2);
    fill_rect_i32(dst, stride, height, x, y, w as i32, t as i32, color);
    fill_rect_i32(
        dst,
        stride,
        height,
        x,
        y + h.saturating_sub(t) as i32,
        w as i32,
        t as i32,
        color,
    );
    fill_rect_i32(dst, stride, height, x, y, t as i32, h as i32, color);
    fill_rect_i32(
        dst,
        stride,
        height,
        x + w.saturating_sub(t) as i32,
        y,
        t as i32,
        h as i32,
        color,
    );
}

fn draw_corner_facets(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: u32,
) {
    if w < 16 || h < 16 {
        return;
    }

    let size = 8i32;
    let inset = 2i32;
    for row in 0..size {
        fill_rect_i32(dst, stride, height, x + inset, y + inset + row, size - row, 1, color);
        fill_rect_i32(
            dst,
            stride,
            height,
            x + w as i32 - inset - size + row,
            y + h as i32 - inset - size + row,
            size - row,
            1,
            color,
        );
    }
}

fn draw_top_glow_strip(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    theme: UiTheme,
) {
    if w == 0 {
        return;
    }

    for sx in 0..w {
        let t = if w <= 1 { 0 } else { sx.saturating_mul(255) / (w - 1) };
        let color = if t < 85 {
            lerp_argb(0x007C5CFF, theme.edge_light, t.saturating_mul(3))
        } else if t < 170 {
            lerp_argb(theme.edge_light, theme.control_icon, (t - 85).saturating_mul(3))
        } else {
            lerp_argb(theme.control_icon, 0x00B8A8FF, (t - 170).saturating_mul(3))
        };
        fill_rect_i32(dst, stride, height, x + sx as i32, y, 1, 2, color);
    }
}

fn fill_vertical_gradient(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    top: u32,
    bottom: u32,
) {
    if h == 0 {
        return;
    }
    for row in 0..h {
        let t = if h <= 1 { 0 } else { row.saturating_mul(255) / (h - 1) };
        fill_rect_i32(
            dst,
            stride,
            height,
            x,
            y + row as i32,
            w as i32,
            1,
            lerp_argb(top, bottom, t),
        );
    }
}

fn fill_horizontal_gradient(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    left: u32,
    center: u32,
    right: u32,
) {
    if w == 0 || h == 0 {
        return;
    }
    for col in 0..w {
        let t = if w <= 1 { 0 } else { col.saturating_mul(255) / (w - 1) };
        let color = if t < 128 {
            lerp_argb(left, center, t.saturating_mul(2))
        } else {
            lerp_argb(center, right, (t - 128).saturating_mul(2))
        };
        fill_rect_i32(dst, stride, height, x + col as i32, y, 1, h as i32, color);
    }
}

fn draw_contact_shadow(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: abi::display_protocol::Rect,
    theme: UiTheme,
) {
    fill_rect_i32(
        dst,
        stride,
        height,
        rect.x as i32 + 2,
        rect.y as i32 + 2,
        rect.w as i32,
        rect.h as i32,
        theme.contact_shadow,
    );
}

fn draw_content_field(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    theme: UiTheme,
) {
    if w == 0 || h == 0 {
        return;
    }

    for gx in (8..w).step_by(8) {
        fill_rect_i32(dst, stride, height, x + gx as i32, y, 1, h as i32, theme.grid_line);
    }
    for gy in (8..h).step_by(8) {
        fill_rect_i32(dst, stride, height, x, y + gy as i32, w as i32, 1, theme.grid_line);
    }
    draw_rect_stroke(dst, stride, height, x, y, w, h, 1, theme.content_edge);
}

fn lerp_argb(a: u32, b: u32, t: u32) -> u32 {
    let t = t.min(255);
    let inv = 255 - t;
    let aa = (a >> 24) & 0xFF;
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let ba = (b >> 24) & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    let ca = (aa * inv + ba * t) / 255;
    let cr = (ar * inv + br * t) / 255;
    let cg = (ag * inv + bg * t) / 255;
    let cb = (ab * inv + bb * t) / 255;
    (ca << 24) | (cr << 16) | (cg << 8) | cb
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
    pointer_x: i32,
    pointer_y: i32,
    primary_button_down: bool,
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
            pointer_x,
            pointer_y,
            primary_button_down,
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
    pointer_x: i32,
    pointer_y: i32,
    primary_button_down: bool,
) {
    if rect.w < 8 || rect.h < 8 {
        return;
    }

    let color = if matches!(button, ChromeButton::Close) { close_icon } else { icon_color };
    let hovered = rect_contains(rect, pointer_x, pointer_y);
    let pressed = hovered && primary_button_down;
    let color = if pressed { 0xFF0B0A10 } else { color };
    if hovered {
        let bg = if pressed { theme.edge_light } else { theme.button_top };
        draw_chrome_button_well(dst, stride, rect, bg);
    }
    let _ = pistil_draw_svg_icon;
    draw_facet_control_glyph(dst, stride, height, rect, button, is_shaded, is_fullscreen, color);
}

fn rect_contains(rect: abi::display_protocol::Rect, x: i32, y: i32) -> bool {
    let x0 = rect.x as i32;
    let y0 = rect.y as i32;
    let x1 = rect.x.saturating_add(rect.w) as i32;
    let y1 = rect.y.saturating_add(rect.h) as i32;
    x >= x0 && x < x1 && y >= y0 && y < y1
}

fn draw_chrome_button_well(
    dst: &mut [u32],
    stride: u32,
    rect: abi::display_protocol::Rect,
    color: u32,
) {
    let size = 16u32.min(rect.w).min(rect.h);
    if size < 8 {
        return;
    }

    let x = rect.x as i32 + rect.w.saturating_sub(size) as i32 / 2;
    let y = rect.y as i32 + rect.h.saturating_sub(size) as i32 / 2;
    fill_rect(dst, stride, x, y, size, size, color);
}

#[allow(dead_code)]
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

fn draw_facet_control_glyph(
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
    let grid = 12i32;
    let stroke = 2i32;

    match button {
        ChromeButton::Minimize | ChromeButton::Shade => {
            fill_rect_i32(dst, stride, height, cx - grid / 2, cy - stroke / 2, grid, stroke, color);
        }
        ChromeButton::Maximize | ChromeButton::Fullscreen => {
            let bx = cx - grid / 2;
            let by = cy - grid / 2;
            fill_rect_i32(dst, stride, height, bx, by, grid, stroke, color);
            fill_rect_i32(dst, stride, height, bx, by, stroke, grid, color);
            fill_rect_i32(dst, stride, height, bx, by + grid - stroke, grid, stroke, color);
            fill_rect_i32(dst, stride, height, bx + grid - stroke, by, stroke, grid, color);
        }
        ChromeButton::Close => {
            let radius = grid / 2;
            for d in -radius..=radius {
                plot_thick_pixel(dst, stride, height, cx + d, cy + d, color, stroke);
                plot_thick_pixel(dst, stride, height, cx + d, cy - d, color, stroke);
            }
        }
    }
    let _ = (is_shaded, is_fullscreen);
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

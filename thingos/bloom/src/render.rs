use alloc::string::String;
use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use pistil_types::Texture;
use stile::{PaintCommand, ThemeIcon, ThemeRect};

use crate::display::DisplayBackend;
use crate::scene::{ChromeButton, CompositionEntry, CursorKind, SurfaceChrome};
use crate::theme::{Theme, default_theme, theme_by_name};

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
const WINDOW_ICON_SHADE_PATH: &str = "/public/icons/lucide/chevron-up.svg";
const WINDOW_ICON_UNSHADE_PATH: &str = "/public/icons/lucide/chevron-down.svg";
const WINDOW_ICON_FULLSCREEN_PATH: &str = "/public/icons/lucide/fullscreen.svg";
const WINDOW_ICON_RESTORE_PATH: &str = "/public/icons/lucide/minimize-2.svg";
const WINDOW_ICON_CLOSE_PATH: &str = "/public/icons/lucide/x.svg";
const DEFAULT_FONT_PATH: &str = "/public/fonts/Inter-Regular.ttf";
const SYMBOL_FONT_PATH: &str = "/public/fonts/NotoSansSymbol2-Regular.ttf";
const CURSOR_SIZE: u32 = 48;
const CURSOR_PIXELS: usize = (CURSOR_SIZE * CURSOR_SIZE) as usize;
const BUSY_CURSOR_HOTSPOT_X: u32 = CURSOR_SIZE / 2;
const BUSY_CURSOR_HOTSPOT_Y: u32 = CURSOR_SIZE / 2;
const BUSY_SPINNER_RGB: u32 = 0x00EAF2FF;
const BUSY_SPINNER_DOT_OFFSETS: [(i32, i32); 12] = [
    (0, -14),
    (7, -12),
    (12, -7),
    (14, 0),
    (12, 7),
    (7, 12),
    (0, 14),
    (-7, 12),
    (-12, 7),
    (-14, 0),
    (-12, -7),
    (-7, -12),
];
const BUSY_SPINNER_ALPHA: [u8; 12] = [238, 208, 174, 140, 112, 88, 68, 52, 40, 32, 26, 22];
const BUSY_SPINNER_SAMPLE_OFFSETS: [i32; 4] = [-3, -1, 1, 3];
const BUSY_SPINNER_BAYER_4X4: [u8; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];
const POINTER_OVERLAY_MAX_W: u32 = 460;
const POINTER_OVERLAY_MAX_H: u32 = 144;
const POINTER_OVERLAY_MARGIN: u32 = 12;
const POINTER_OVERLAY_CURSOR_INSET: i32 = 12;
const RUNBOX_CARET_W: u32 = 2;
const CHROME_ICON_X_BIAS: i32 = 0;
const CHROME_ICON_Y_BIAS: i32 = 0;
const WALLPAPER_FADE_FRAMES: u8 = 18;
const WALLPAPER_FADE_BLACK: u32 = 0xFF000000;

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
    background_fade: Option<BackgroundFade>,
    cursor: Option<CursorBuffer>,
    pending_cursor: Option<CursorBuffer>,
    cursor_variants: Vec<(CursorKind, CursorBuffer)>,
    pointer_overlay: Option<PointerOverlayBuffer>,
    runbox_overlay: Option<RunBoxOverlayBuffer>,
    launcher_overlay: Option<LauncherOverlayBuffer>,
    body_overlays: Vec<ChromeOverlayBuffer>,
    chrome_overlays: Vec<ChromeOverlayBuffer>,
    pistil: Option<PistilLib>,
    default_font_ready: bool,
    symbol_font_ready: bool,
    theme: Theme,
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

struct BackgroundFade {
    pixels: Vec<u32>,
    frame: u8,
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
    pub wallpaper_pending: bool,
    pub cursor_pending: bool,
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

struct RunBoxOverlayBuffer {
    texture: Texture,
    buffer_id: u32,
    width: u32,
    height: u32,
}

struct LauncherOverlayBuffer {
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
            background_fade: None,
            cursor: None,
            pending_cursor: None,
            cursor_variants: Vec::new(),
            pointer_overlay: None,
            runbox_overlay: None,
            launcher_overlay: None,
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
        self.background_fade = None;
    }

    /// Synchronously decode and import a wallpaper.
    pub fn prepare_background(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        let (width, height) = display.output_size();

        let mut texture: Texture =
            match Texture::new("bloom.compositor.background", width, height, 4) {
                Some(t) => t,
                None => return,
            };

        let t_prepare_start = stem::time::monotonic_ns();
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
        let t_prepare_done = stem::time::monotonic_ns();

        let mut fade_pixels = Vec::new();
        if success {
            fade_pixels.extend_from_slice(texture.as_slice_mut());
            texture.as_slice_mut().fill(WALLPAPER_FADE_BLACK);
        } else {
            if self.background.is_some() {
                return;
            }
            stem::info!("Using facet-frame fallback background");
            texture.as_slice_mut().fill(0xFF0B0A10);
            self.background_fade = None;
        }

        let t_import_start = stem::time::monotonic_ns();
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
        let t_import_done = stem::time::monotonic_ns();
        stem::debug!(
            "Wallpaper prepare: prepare={}ms import={}ms",
            (t_prepare_done - t_prepare_start) / 1_000_000,
            (t_import_done - t_import_start) / 1_000_000,
        );

        if let Some(old) = self.background.take() {
            display.release_buffer(old.buffer_id);
        }

        self.background = Some(ServerBuffer { _texture: texture, buffer_id });
        if success {
            // Record the path only when the file was decoded successfully so
            // a re-request for the same path after a decode failure is retried.
            self.wallpaper_path = Some(alloc::string::String::from(wallpaper_path));
            self.background_fade = Some(BackgroundFade { pixels: fade_pixels, frame: 0 });
            stem::info!("Fading in wallpaper...");
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

    pub fn advance_background_fade(&mut self) -> bool {
        if self.background_fade.is_none() {
            return false;
        }

        let mut empty_fade = false;
        let mut complete = false;
        {
            let Some(background) = self.background.as_mut() else {
                self.background_fade = None;
                return false;
            };
            let Some(fade) = self.background_fade.as_mut() else {
                return false;
            };
            let dst = background._texture.as_slice_mut();
            let len = dst.len().min(fade.pixels.len());
            if len == 0 {
                empty_fade = true;
            } else {
                let alpha = (u32::from(fade.frame).saturating_mul(255)
                    / u32::from(WALLPAPER_FADE_FRAMES)) as u8;
                write_wallpaper_fade_frame(&mut dst[..len], &fade.pixels[..len], alpha);
                if fade.frame >= WALLPAPER_FADE_FRAMES {
                    complete = true;
                } else {
                    fade.frame = fade.frame.saturating_add(1);
                }
            }
        }

        if empty_fade {
            self.background_fade = None;
            return false;
        }
        if complete {
            self.background_fade = None;
            stem::info!("Wallpaper fade-in complete");
        }
        true
    }

    pub fn background_fade_active(&self) -> bool {
        self.background_fade.is_some()
    }

    pub fn retry_deferred_resources(
        &mut self,
        display: &DisplayBackend,
        wallpaper_path: Option<&str>,
        load_cursor: bool,
        check_fonts: bool,
    ) -> ResourceRetryStatus {
        let mut improved = false;

        if self.pistil.is_none() {
            self.pistil = load_pistil(false);
            if self.pistil.is_some() {
                stem::debug!("Deferred Pistil renderer became available");
                improved = true;
            }
        }

        let mut font_pending = false;
        if check_fonts {
            let Some(ref lib) = self.pistil else {
                let pending = true;
                return ResourceRetryStatus {
                    pending,
                    improved,
                    wallpaper_pending: wallpaper_path.is_some(),
                    cursor_pending: load_cursor && self.needs_asset_cursor(),
                };
            };
            if lib.draw_text.is_some() {
                match lib.default_font_ready {
                    Some(ready) if ready() != 0 => {
                        if !self.default_font_ready {
                            stem::debug!("Deferred default font became available");
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
                            stem::debug!("Deferred symbol font became available");
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
                    stem::debug!("Deferred wallpaper became available: {}", path);
                    improved = true;
                }
            }
        }

        let wallpaper_pending = wallpaper_path
            .map(|path| self.wallpaper_path.as_deref() != Some(path))
            .unwrap_or(false);
        let wallpaper_settled = !wallpaper_pending;

        if load_cursor && wallpaper_settled && self.pistil.is_some() && self.needs_asset_cursor() {
            self.prepare_cursor(display);
            if !self.needs_asset_cursor() {
                improved = true;
            }
        }

        let cursor_pending = load_cursor && self.needs_asset_cursor();
        let pending = self.pistil.is_none()
            || (check_fonts && font_pending)
            || wallpaper_pending
            || cursor_pending;

        ResourceRetryStatus { pending, improved, wallpaper_pending, cursor_pending }
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
        if let Some(old) = self.runbox_overlay.take() {
            display.release_buffer(old.buffer_id);
        }
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }

    pub fn prepare_busy_spinner(&mut self, display: &DisplayBackend) {
        if self.cursor.is_some() {
            return;
        }

        let mut texture =
            match Texture::new("bloom.compositor.cursor.busy", CURSOR_SIZE, CURSOR_SIZE, 4) {
                Some(t) => t,
                None => {
                    stem::warn!("bloom: failed to allocate built-in busy spinner texture");
                    return;
                }
            };
        let stride_pixels = texture.stride / 4;
        let height = texture.height;
        draw_builtin_busy_spinner(texture.as_slice_mut(), stride_pixels, height, 0, 255);

        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            texture.width,
            texture.height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
            0,
        ) else {
            stem::warn!("bloom: failed to import built-in busy spinner texture");
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
        stem::debug!(
            "Built-in busy spinner ready: buffer={} size={}x{} hotspot={},{}",
            buffer_id,
            CURSOR_SIZE,
            CURSOR_SIZE,
            BUSY_CURSOR_HOTSPOT_X,
            BUSY_CURSOR_HOTSPOT_Y
        );
    }

    pub fn draw_busy_spinner_frame(
        &mut self,
        frame: u32,
        opacity: u8,
    ) -> Option<(u32, u32, u32, u32)> {
        let cursor = self.cursor.as_mut()?;
        if !cursor.fallback {
            return None;
        }
        let stride_pixels = cursor._texture.stride / 4;
        let height = cursor._texture.height;
        draw_builtin_busy_spinner(
            cursor._texture.as_slice_mut(),
            stride_pixels,
            height,
            frame,
            opacity,
        );
        Some((cursor.width, cursor.height, cursor.hotspot_x, cursor.hotspot_y))
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

        let cursor = CursorBuffer {
            _texture: texture,
            buffer_id,
            width: CURSOR_SIZE,
            height: CURSOR_SIZE,
            hotspot_x: hotspot[0],
            hotspot_y: hotspot[1],
            fallback: false,
        };
        if self.cursor.as_ref().map(|cursor| cursor.fallback).unwrap_or(false) {
            if let Some(old) = self.pending_cursor.take() {
                display.release_buffer(old.buffer_id);
            }
            self.pending_cursor = Some(cursor);
        } else {
            if let Some(old) = self.cursor.take() {
                display.release_buffer(old.buffer_id);
            }
            self.cursor = Some(cursor);
        }
        stem::debug!(
            "Cursor ready: buffer={} size={}x{} hotspot={},{}",
            buffer_id,
            CURSOR_SIZE,
            CURSOR_SIZE,
            hotspot[0],
            hotspot[1]
        );
    }

    fn needs_asset_cursor(&self) -> bool {
        self.pending_cursor.is_none()
            && self.cursor.as_ref().map(|cursor| cursor.fallback).unwrap_or(true)
    }

    pub fn cursor_is_fallback(&self) -> bool {
        self.cursor.as_ref().map(|cursor| cursor.fallback).unwrap_or(false)
    }

    pub fn pending_cursor_ready(&self) -> bool {
        self.pending_cursor.is_some()
    }

    pub fn activate_pending_cursor(&mut self, display: &DisplayBackend) -> bool {
        let Some(cursor) = self.pending_cursor.take() else {
            return false;
        };
        let buffer_id = cursor.buffer_id;
        if let Some(old) = self.cursor.take() {
            display.release_buffer(old.buffer_id);
        }
        for (_, old) in self.cursor_variants.drain(..) {
            display.release_buffer(old.buffer_id);
        }
        self.cursor = Some(cursor);
        stem::debug!("Busy spinner handoff complete; real cursor active buffer={}", buffer_id);
        true
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
        layout_debug: bool,
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
            let visual_rect = visual_rect_for_entry(entry);
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
                layout_debug,
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
            let stride_pixels = texture.stride / 4;
            let texture_height = texture.height;
            hotspot = draw_builtin_cursor_variant(
                texture.as_slice_mut(),
                stride_pixels,
                texture_height,
                kind,
            );
            stem::debug!("Using built-in cursor {:?}", kind);
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
        stem::debug!("Cursor {:?} ready from {} with buffer={}", kind, cursor_path, buffer_id);
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
        stem::debug!("Pointer debug overlay ready: buffer={} size={}x{}", buffer_id, width, height);
        Some(())
    }

    pub fn runbox_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        state: &blossom::runbox::RunState,
        layout_debug: bool,
    ) -> Option<OverlayPlane> {
        if !state.visible {
            return None;
        }
        self.ensure_runbox_overlay(display)?;
        let (output_w, output_h) = display.output_size();
        let placement = blossom::runbox::runbox_rect(output_w, output_h);

        let overlay = self.runbox_overlay.as_mut()?;
        draw_runbox_overlay(
            overlay.texture.as_slice_mut(),
            overlay.width,
            overlay.height,
            state,
            self.pistil.as_ref().and_then(|lib| lib.draw_text),
            layout_debug,
        );

        Some(OverlayPlane {
            buffer_id: overlay.buffer_id,
            x: placement.x,
            y: placement.y,
            width: overlay.width,
            height: overlay.height,
        })
    }

    fn ensure_runbox_overlay(&mut self, display: &DisplayBackend) -> Option<()> {
        let width = blossom::runbox::RUNBOX_WIDTH;
        let height = blossom::runbox::RUNBOX_HEIGHT;
        if matches!(
            self.runbox_overlay.as_ref(),
            Some(overlay) if overlay.width == width && overlay.height == height
        ) {
            return Some(());
        }

        let texture = Texture::new("bloom.compositor.runbox_overlay", width, height, 4)?;
        let buffer_id = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
            0,
        )?;

        if let Some(old) = self.runbox_overlay.take() {
            display.release_buffer(old.buffer_id);
        }

        self.runbox_overlay = Some(RunBoxOverlayBuffer { texture, buffer_id, width, height });
        stem::debug!("Run dialog overlay ready: buffer={} size={}x{}", buffer_id, width, height);
        Some(())
    }

    pub fn launcher_overlay_plane(
        &mut self,
        display: &DisplayBackend,
        state: &blossom::launcher::LauncherState,
        layout_debug: bool,
    ) -> Option<OverlayPlane> {
        if !state.visible {
            return None;
        }
        self.ensure_launcher_overlay(display)?;
        let (output_w, output_h) = display.output_size();
        let placement = blossom::launcher::launcher_rect(output_w, output_h);

        let overlay = self.launcher_overlay.as_mut()?;
        draw_launcher_overlay(
            overlay.texture.as_slice_mut(),
            overlay.width,
            overlay.height,
            state,
            self.pistil.as_ref().and_then(|lib| lib.draw_text),
            self.pistil.as_ref().and_then(|lib| lib.draw_svg_icon),
            layout_debug,
        );

        Some(OverlayPlane {
            buffer_id: overlay.buffer_id,
            x: placement.x,
            y: placement.y,
            width: overlay.width,
            height: overlay.height,
        })
    }

    fn ensure_launcher_overlay(&mut self, display: &DisplayBackend) -> Option<()> {
        let width = blossom::launcher::LAUNCHER_WIDTH;
        let height = blossom::launcher::LAUNCHER_HEIGHT;
        if matches!(
            self.launcher_overlay.as_ref(),
            Some(overlay) if overlay.width == width && overlay.height == height
        ) {
            return Some(());
        }

        let texture = Texture::new("bloom.compositor.launcher_overlay", width, height, 4)?;
        let buffer_id = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
            0,
        )?;

        if let Some(old) = self.launcher_overlay.take() {
            display.release_buffer(old.buffer_id);
        }

        self.launcher_overlay = Some(LauncherOverlayBuffer { texture, buffer_id, width, height });
        stem::debug!(
            "Application launcher overlay ready: buffer={} size={}x{}",
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
    stem::debug!("Pistil background renderer loaded from {}", PISTIL_PATH);
    if draw_text.is_some() {
        stem::debug!("Pistil font text renderer loaded with default {}", DEFAULT_FONT_PATH);
    }
    if draw_symbol_text.is_some() {
        stem::debug!("Pistil symbol renderer loaded with {}", SYMBOL_FONT_PATH);
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

fn draw_builtin_busy_spinner(dst: &mut [u32], stride: u32, height: u32, frame: u32, opacity: u8) {
    dst.fill(0);
    if stride < CURSOR_SIZE || height < CURSOR_SIZE || opacity == 0 {
        return;
    }

    let phase = (frame as usize) % BUSY_SPINNER_DOT_OFFSETS.len();
    let center = (CURSOR_SIZE / 2) as i32;

    for &(dx, dy) in &BUSY_SPINNER_DOT_OFFSETS {
        draw_spinner_dot(
            dst,
            stride,
            center + dx + 1,
            center + dy + 2,
            3,
            argb(scale_alpha(0x24, opacity), 0x000000),
        );
    }

    for (idx, &(dx, dy)) in BUSY_SPINNER_DOT_OFFSETS.iter().enumerate() {
        let age = (BUSY_SPINNER_DOT_OFFSETS.len() + idx - phase) % BUSY_SPINNER_DOT_OFFSETS.len();
        let alpha = scale_alpha(BUSY_SPINNER_ALPHA[age], opacity);
        let radius = if age <= 2 { 3 } else { 2 };
        draw_spinner_dot(
            dst,
            stride,
            center + dx,
            center + dy,
            radius,
            argb(alpha, BUSY_SPINNER_RGB),
        );
    }
}

fn draw_builtin_cursor_variant(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    kind: CursorKind,
) -> [u32; 2] {
    dst.fill(0);
    if stride < CURSOR_SIZE || height < CURSOR_SIZE {
        return [CURSOR_SIZE / 2, CURSOR_SIZE / 2];
    }

    let center = (CURSOR_SIZE / 2) as i32;
    let shadow = 0xE0181B24;
    let fill = 0xFFFFFFFF;
    match kind {
        CursorKind::Move => {
            draw_cursor_double_arrow(dst, stride, height, 12, center, 36, center, shadow, fill);
            draw_cursor_double_arrow(dst, stride, height, center, 12, center, 36, shadow, fill);
        }
        CursorKind::ResizeNorth | CursorKind::ResizeSouth => {
            draw_cursor_double_arrow(dst, stride, height, center, 10, center, 38, shadow, fill);
        }
        CursorKind::ResizeEast | CursorKind::ResizeWest => {
            draw_cursor_double_arrow(dst, stride, height, 10, center, 38, center, shadow, fill);
        }
        CursorKind::ResizeNorthEast | CursorKind::ResizeSouthWest => {
            draw_cursor_double_arrow(dst, stride, height, 36, 12, 12, 36, shadow, fill);
        }
        CursorKind::ResizeNorthWest | CursorKind::ResizeSouthEast => {
            draw_cursor_double_arrow(dst, stride, height, 12, 12, 36, 36, shadow, fill);
        }
        CursorKind::Default => {}
    }
    [CURSOR_SIZE / 2, CURSOR_SIZE / 2]
}

fn draw_cursor_double_arrow(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    shadow: u32,
    fill: u32,
) {
    draw_cursor_arrow_layer(dst, stride, height, x0, y0, x1, y1, 2, shadow);
    draw_cursor_arrow_layer(dst, stride, height, x0, y0, x1, y1, 1, fill);
}

fn draw_cursor_arrow_layer(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    color: u32,
) {
    draw_cursor_segment(dst, stride, height, x0, y0, x1, y1, radius, color);
    let dx = (x1 - x0).signum();
    let dy = (y1 - y0).signum();
    draw_cursor_arrow_head(dst, stride, height, x1, y1, dx, dy, radius, color);
    draw_cursor_arrow_head(dst, stride, height, x0, y0, -dx, -dy, radius, color);
}

fn draw_cursor_arrow_head(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    tip_x: i32,
    tip_y: i32,
    dir_x: i32,
    dir_y: i32,
    radius: i32,
    color: u32,
) {
    let len = 7;
    let spread = 5;
    let back_x = tip_x - dir_x * len;
    let back_y = tip_y - dir_y * len;
    let perp_x = -dir_y;
    let perp_y = dir_x;
    draw_cursor_segment(
        dst,
        stride,
        height,
        tip_x,
        tip_y,
        back_x + perp_x * spread,
        back_y + perp_y * spread,
        radius,
        color,
    );
    draw_cursor_segment(
        dst,
        stride,
        height,
        tip_x,
        tip_y,
        back_x - perp_x * spread,
        back_y - perp_y * spread,
        radius,
        color,
    );
}

fn draw_cursor_segment(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    color: u32,
) {
    let steps = (x1 - x0).abs().max((y1 - y0).abs()).max(1);
    for step in 0..=steps {
        let x = x0 + (x1 - x0) * step / steps;
        let y = y0 + (y1 - y0) * step / steps;
        draw_cursor_brush(dst, stride, height, x, y, radius, color);
    }
}

fn draw_cursor_brush(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    cx: i32,
    cy: i32,
    radius: i32,
    color: u32,
) {
    for y in cy - radius..=cy + radius {
        if y < 0 || y >= height as i32 || y >= CURSOR_SIZE as i32 {
            continue;
        }
        let row = y as usize * stride as usize;
        for x in cx - radius..=cx + radius {
            if x < 0 || x >= CURSOR_SIZE as i32 {
                continue;
            }
            dst[row + x as usize] = color;
        }
    }
}

fn scale_alpha(alpha: u8, opacity: u8) -> u8 {
    ((alpha as u16 * opacity as u16 + 127) / 255) as u8
}

fn write_wallpaper_fade_frame(dst: &mut [u32], src: &[u32], alpha: u8) {
    for (dst_px, src_px) in dst.iter_mut().zip(src.iter().copied()) {
        *dst_px = fade_pixel_from_black(src_px, alpha);
    }
}

fn fade_pixel_from_black(pixel: u32, alpha: u8) -> u32 {
    let alpha = alpha as u32;
    let a = pixel & 0xFF000000;
    let r = (((pixel >> 16) & 0xFF) * alpha + 127) / 255;
    let g = (((pixel >> 8) & 0xFF) * alpha + 127) / 255;
    let b = ((pixel & 0xFF) * alpha + 127) / 255;
    a | (r << 16) | (g << 8) | b
}

fn draw_spinner_dot(dst: &mut [u32], stride: u32, cx: i32, cy: i32, radius: i32, color: u32) {
    let sample_center_x = cx * 8 + 4;
    let sample_center_y = cy * 8 + 4;
    let sample_radius = radius * 8;
    let sample_radius_sq = sample_radius * sample_radius;
    let outer = radius + 1;
    let base_alpha = color >> 24;
    let rgb = color & 0x00FFFFFF;

    for y in cy - outer..=cy + outer {
        if y < 0 || y >= CURSOR_SIZE as i32 {
            continue;
        }
        let row = y as usize * stride as usize;
        for x in cx - outer..=cx + outer {
            if x < 0 || x >= CURSOR_SIZE as i32 {
                continue;
            }
            let mut coverage = 0u32;
            for sy in BUSY_SPINNER_SAMPLE_OFFSETS {
                for sx in BUSY_SPINNER_SAMPLE_OFFSETS {
                    let sample_x = x * 8 + 4 + sx;
                    let sample_y = y * 8 + 4 + sy;
                    let dx = sample_x - sample_center_x;
                    let dy = sample_y - sample_center_y;
                    if dx * dx + dy * dy <= sample_radius_sq {
                        coverage += 1;
                    }
                }
            }
            if coverage > 0 {
                let bayer =
                    BUSY_SPINNER_BAYER_4X4[((y as usize & 3) << 2) | (x as usize & 3)] as u32;
                let alpha = (base_alpha * (coverage * 16 + bayer) / 256).min(base_alpha);
                dst[row + x as usize] = (alpha << 24) | rgb;
            }
        }
    }
}

fn argb(alpha: u8, rgb: u32) -> u32 {
    ((alpha as u32) << 24) | (rgb & 0x00FFFFFF)
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
            stem::debug!(
                "Flat window overlay ready: surface={} size={}x{}",
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
    let rect = visual_rect_for_entry(entry);
    if rect.w == 0 || rect.h == 0 { None } else { Some((rect.w, rect.h)) }
}

fn visual_rect_for_entry(entry: &CompositionEntry) -> abi::display_protocol::Rect {
    crate::scene::from_blossom_rect(blossom::wm::surface_visual_rect(
        crate::scene::to_blossom_rect(entry.dest_rect),
        to_blossom_chrome(entry.chrome),
    ))
}

fn to_blossom_chrome(chrome: SurfaceChrome) -> blossom::wm::SurfaceChrome {
    blossom::wm::SurfaceChrome {
        titlebar_height: chrome.titlebar_height.min(i32::MAX as u32) as i32,
        frame_thickness: chrome.frame_thickness.min(i32::MAX as u32) as i32,
    }
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
    _pistil_draw_symbol_text: Option<DrawTextFn>,
    theme: Theme,
    pointer_x: i32,
    pointer_y: i32,
    primary_button_down: bool,
    layout_debug: bool,
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
        let window_hovered = rect_contains(visual_rect_for_entry(entry), pointer_x, pointer_y);
        let plan = blossom::chrome::window_chrome_plan(
            crate::scene::to_blossom_rect(rect),
            to_blossom_chrome(chrome),
            theme,
            blossom::chrome::ChromeVisualState {
                active: entry.active,
                hovered: window_hovered,
                primary_button_down,
                pointer_x,
                pointer_y,
                shaded: entry.is_shaded,
                fullscreen: entry.is_fullscreen,
            },
            entry.title.as_deref(),
        );
        execute_blossom_chrome_plan(
            dst,
            stride,
            height,
            &plan,
            pistil_draw_svg_icon,
            pistil_draw_text,
        );

        if layout_debug {
            draw_chrome_layout_debug(dst, stride, height, entry, theme);
        }
    }

    let len = (stride.saturating_mul(height)) as usize;
    if dst.len() > len {
        for px in &mut dst[len..] {
            *px = 0;
        }
    }
}

/// Draw layout-debug bounding boxes on a chrome overlay by building a petals
/// `UiTree` for the window chrome and calling [`petals::draw_layout_debug`].
fn draw_chrome_layout_debug(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    entry: &CompositionEntry,
    theme: Theme,
) {
    let title = entry.title.as_deref().unwrap_or("");
    let Ok((mut tree, _)) = petals::window_chrome_tree(title) else {
        return;
    };
    let rules = petals::window_chrome_rules_for_theme(theme);
    if tree.restyle(&rules).is_err() {
        return;
    }
    if tree
        .apply_style(
            tree.root(),
            petals::ResolvedStyle {
                width: Some(stride as f32),
                height: Some(height as f32),
                flex_direction: Some(petals::FlexDirection::Column),
                align_items: Some(petals::AlignItems::Stretch),
                ..petals::ResolvedStyle::default()
            },
        )
        .is_err()
    {
        return;
    }
    if tree
        .compute_layout(petals::Size {
            width: petals::AvailableSpace::Definite(stride as f32),
            height: petals::AvailableSpace::Definite(height as f32),
        })
        .is_err()
    {
        return;
    }
    petals::draw_layout_debug(&tree, dst, stride, height);
}

fn execute_blossom_chrome_plan(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    plan: &blossom::chrome::WindowPaintPlan<'_>,
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    pistil_draw_text: Option<DrawTextFn>,
) {
    let mut clip_stack: Vec<Option<ThemeRect>> = Vec::new();
    let mut clip: Option<ThemeRect> = None;
    for command in &plan.commands {
        match command {
            PaintCommand::FillRect { rect, color } => {
                if let Some(rect) = clipped_rect(*rect, clip) {
                    fill_rect_i32(dst, stride, height, rect.x, rect.y, rect.w, rect.h, *color);
                }
            }
            PaintCommand::VerticalGradient { rect, top, bottom } => {
                if let Some(rect) = clipped_rect(*rect, clip) {
                    fill_vertical_gradient(
                        dst,
                        stride,
                        height,
                        rect.x,
                        rect.y,
                        rect.w as u32,
                        rect.h as u32,
                        *top,
                        *bottom,
                    );
                }
            }
            PaintCommand::HorizontalGradient { rect, left, center, right } => {
                if let Some(rect) = clipped_rect(*rect, clip) {
                    fill_horizontal_gradient(
                        dst,
                        stride,
                        height,
                        rect.x,
                        rect.y,
                        rect.w as u32,
                        rect.h as u32,
                        *left,
                        *center,
                        *right,
                    );
                }
            }
            PaintCommand::StrokeRect { rect, thickness, color } => {
                if let Some(rect) = clipped_rect(*rect, clip) {
                    if *thickness <= 0 {
                        continue;
                    }
                    draw_rect_stroke(
                        dst,
                        stride,
                        height,
                        rect.x,
                        rect.y,
                        rect.w as u32,
                        rect.h as u32,
                        *thickness as u32,
                        *color,
                    );
                }
            }
            PaintCommand::Shadow { rect, offset_x, offset_y, blur_radius, color } => {
                draw_theme_shadow(
                    dst,
                    stride,
                    height,
                    *rect,
                    *offset_x,
                    *offset_y,
                    *blur_radius,
                    *color,
                    clip,
                );
            }
            PaintCommand::PushClip { rect } => {
                clip_stack.push(clip);
                clip = match clip {
                    Some(current) => intersect_theme_rect(current, *rect),
                    None => clipped_rect(*rect, None),
                };
            }
            PaintCommand::PopClip => {
                clip = clip_stack.pop().unwrap_or(None);
            }
            PaintCommand::Text { x, y, px_size_bits, text, color } => {
                if let Some(clip) = clip {
                    if *x < clip.x || *x >= clip.x + clip.w || *y < clip.y || *y >= clip.y + clip.h
                    {
                        continue;
                    }
                }
                draw_overlay_text_bold(
                    pistil_draw_text,
                    dst,
                    stride,
                    height,
                    *x,
                    *y,
                    f32::from_bits(*px_size_bits),
                    text.as_ref(),
                    *color,
                );
            }
            PaintCommand::Icon { rect, icon, color } => {
                let Some(rect) = clipped_rect(*rect, clip) else {
                    continue;
                };
                if draw_theme_svg_icon(
                    pistil_draw_svg_icon,
                    dst,
                    stride,
                    height,
                    rect,
                    theme_icon_path(*icon),
                    *color,
                ) {
                    continue;
                }
                draw_facet_control_glyph(
                    dst,
                    stride,
                    height,
                    crate::scene::from_blossom_rect(blossom::Rect::new(
                        rect.x, rect.y, rect.w, rect.h,
                    )),
                    chrome_button_for_theme_icon(*icon),
                    false,
                    false,
                    *color,
                );
            }
        }
    }
}

fn clipped_rect(rect: ThemeRect, clip: Option<ThemeRect>) -> Option<ThemeRect> {
    if rect.w <= 0 || rect.h <= 0 {
        return None;
    }
    match clip {
        Some(clip) => intersect_theme_rect(rect, clip),
        None => Some(rect),
    }
}

fn intersect_theme_rect(a: ThemeRect, b: ThemeRect) -> Option<ThemeRect> {
    let x0 = a.x.max(b.x);
    let y0 = a.y.max(b.y);
    let x1 = a.x.saturating_add(a.w).min(b.x.saturating_add(b.w));
    let y1 = a.y.saturating_add(a.h).min(b.y.saturating_add(b.h));
    if x1 <= x0 || y1 <= y0 { None } else { Some(ThemeRect::new(x0, y0, x1 - x0, y1 - y0)) }
}

fn draw_theme_shadow(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: ThemeRect,
    offset_x: i32,
    offset_y: i32,
    blur_radius: i32,
    color: u32,
    clip: Option<ThemeRect>,
) {
    if rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let blur = blur_radius.max(0);
    let layers = blur.max(1);
    for layer in (0..=layers).rev() {
        let spread = layer;
        let alpha_scale = ((layers - layer + 1) as u32).saturating_mul(255) / (layers + 1) as u32;
        let layer_color = scale_argb_alpha(color, alpha_scale);
        let shadow = ThemeRect::new(
            rect.x + offset_x - spread,
            rect.y + offset_y - spread,
            rect.w + spread * 2,
            rect.h + spread * 2,
        );
        if let Some(shadow) = clipped_rect(shadow, clip) {
            fill_rect_i32(dst, stride, height, shadow.x, shadow.y, shadow.w, shadow.h, layer_color);
        }
    }
}

fn scale_argb_alpha(color: u32, scale: u32) -> u32 {
    let scale = scale.min(255);
    let alpha = ((color >> 24) & 0xFF).saturating_mul(scale) / 255;
    (alpha << 24) | (color & 0x00FFFFFF)
}

fn draw_theme_svg_icon(
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    rect: ThemeRect,
    icon_path: Option<&'static str>,
    color: u32,
) -> bool {
    let (Some(draw_svg_icon), Some(path)) = (pistil_draw_svg_icon, icon_path) else {
        return false;
    };
    if rect.w <= 0 || rect.h <= 0 {
        return false;
    }
    let icon_size = (rect.w.min(rect.h) as u32).saturating_sub(16).clamp(18, 26);
    let dst_x = rect.x + (rect.w - icon_size as i32) / 2;
    let dst_y = rect.y + (rect.h - icon_size as i32) / 2;
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

fn theme_icon_path(icon: ThemeIcon) -> Option<&'static str> {
    match icon {
        ThemeIcon::Shade => Some(WINDOW_ICON_SHADE_PATH),
        ThemeIcon::Unshade => Some(WINDOW_ICON_UNSHADE_PATH),
        ThemeIcon::Fullscreen => Some(WINDOW_ICON_FULLSCREEN_PATH),
        ThemeIcon::Restore => Some(WINDOW_ICON_RESTORE_PATH),
        ThemeIcon::Close => Some(WINDOW_ICON_CLOSE_PATH),
    }
}

fn chrome_button_for_theme_icon(icon: ThemeIcon) -> ChromeButton {
    match icon {
        ThemeIcon::Shade | ThemeIcon::Unshade => ChromeButton::Shade,
        ThemeIcon::Fullscreen | ThemeIcon::Restore => ChromeButton::Fullscreen,
        ThemeIcon::Close => ChromeButton::Close,
    }
}

fn draw_window_body_overlay(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    composition: &[CompositionEntry],
    theme: Theme,
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
        if frame == 0 || rect.w == 0 || rect.h == 0 {
            continue;
        }

        let body_x = rect.x as i32;
        let body_y = rect.y as i32;
        let body_w = rect.w;
        let body_h = rect.h;
        let visual_rect = visual_rect_for_entry(entry);
        draw_contact_shadow(dst, stride, height, visual_rect, theme);
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
    theme: Theme,
) {
    let base_alpha = (theme.contact_shadow >> 24) & 0xFF;
    let rgb = theme.contact_shadow & 0x00FFFFFF;

    // Soft multi-layer drop shadow offset to the right and down.
    // Layers are drawn outermost-first (lightest) and innermost-last
    // (darkest), so later layers overwrite the overlap region and the
    // visible outer ring fades smoothly from full opacity to transparent.
    const OFFSET_X: i32 = 4;
    const OFFSET_Y: i32 = 6;
    const LAYERS: u32 = 5;
    for layer in 0..LAYERS {
        let expand = (LAYERS - 1 - layer) as i32;
        // Round to nearest to avoid truncation banding across layers.
        let alpha = (base_alpha as u32 * (layer + 1) + LAYERS / 2) / LAYERS;
        fill_rect_i32(
            dst,
            stride,
            height,
            rect.x as i32 + OFFSET_X,
            rect.y as i32 + OFFSET_Y,
            rect.w as i32 + expand,
            rect.h as i32 + expand,
            (alpha << 24) | rgb,
        );
    }
}

fn draw_content_field(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    theme: Theme,
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

fn rect_contains(rect: abi::display_protocol::Rect, x: i32, y: i32) -> bool {
    let x0 = rect.x as i32;
    let y0 = rect.y as i32;
    let x1 = rect.x.saturating_add(rect.w) as i32;
    let y1 = rect.y.saturating_add(rect.h) as i32;
    x >= x0 && x < x1 && y >= y0 && y < y1
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
    let grid = 18i32;
    let stroke = 3i32;

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

fn draw_runbox_overlay(
    dst: &mut [u32],
    width: u32,
    height: u32,
    state: &blossom::runbox::RunState,
    pistil_draw_text: Option<DrawTextFn>,
    layout_debug: bool,
) {
    dst.fill(0);
    fill_rect(dst, width, 0, 0, width, height, 0xF5181C26);
    draw_rect_stroke(dst, width, height, 0, 0, width, height, 1, 0xFF2A2F3D);

    draw_overlay_text(pistil_draw_text, dst, width, height, 16, 24, 14.0, "Run", 0xFF8F98AA);

    let field_x = 16;
    let field_y = 38;
    let field_w = width.saturating_sub(32);
    let field_h = 42;
    fill_rect(dst, width, field_x, field_y, field_w, field_h, 0xFF111318);
    draw_rect_stroke(dst, width, height, field_x, field_y, field_w, field_h, 1, 0xFFD8A657);

    let text_x = field_x + 10;
    let text_y = field_y + 27;
    draw_overlay_text(
        pistil_draw_text,
        dst,
        width,
        height,
        text_x,
        text_y,
        18.0,
        &state.input,
        0xFFE6EAF0,
    );
    let caret_x = text_x + (state.cursor as i32 * 10);
    fill_rect(
        dst,
        width,
        caret_x,
        field_y + 10,
        RUNBOX_CARET_W,
        field_h.saturating_sub(20),
        0xFFD8A657,
    );

    let button_w = 72;
    let button_h = 32;
    let button_x = width.saturating_sub(16 + button_w) as i32;
    let button_y = height.saturating_sub(16 + button_h) as i32;
    fill_rect(dst, width, button_x, button_y, button_w, button_h, 0xFFD8A657);
    draw_overlay_text(
        pistil_draw_text,
        dst,
        width,
        height,
        button_x + 22,
        button_y + 22,
        14.0,
        "Run",
        0xFF111318,
    );

    if layout_debug {
        // Run dialog doesn't use a petals tree, so draw known hit regions manually
        // using the same color palette as `petals::debug_overlay`.
        // Layout bound: full dialog (depth 0 color).
        draw_debug_border_argb(dst, width, height, 0, 0, width, height, petals::DEPTH_COLORS[0]);
        // Text input field (depth 1 color – layout bound + hit region).
        draw_debug_border_argb(
            dst,
            width,
            height,
            field_x,
            field_y,
            field_w,
            field_h,
            petals::DEPTH_COLORS[1],
        );
        // Run button (hit region color).
        draw_debug_border_argb(
            dst,
            width,
            height,
            button_x,
            button_y,
            button_w,
            button_h,
            petals::HIT_REGION_COLOR,
        );
    }
}

fn draw_launcher_overlay(
    dst: &mut [u32],
    width: u32,
    height: u32,
    state: &blossom::launcher::LauncherState,
    pistil_draw_text: Option<DrawTextFn>,
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
    layout_debug: bool,
) {
    dst.fill(0);
    fill_rect(dst, width, 0, 0, width, height, 0xF516181D);
    draw_rect_stroke(dst, width, height, 0, 0, width, height, 1, 0xFF2A2F3D);

    let Ok((tree, nodes)) = blossom::launcher::launcher_tree(state) else {
        draw_overlay_text(
            pistil_draw_text,
            dst,
            width,
            height,
            16,
            32,
            20.0,
            "Applications",
            0xFFF1F4F8,
        );
        return;
    };

    draw_tree_text(&tree, nodes.title, dst, width, height, pistil_draw_text, 20.0, 0xFFF1F4F8);

    for tile in nodes.tiles {
        let Ok(tile_box) = tree.global_layout_box(tile.node) else {
            continue;
        };
        let tile_node = tree.node(tile.node);
        let bg = tile_node
            .and_then(|node| node.style.background_color)
            .map(color_argb)
            .unwrap_or(0xFF242933);
        let border =
            if state.pressed_index == Some(tile.entry_index) { 0xFFD8A657 } else { 0xFF343B47 };
        fill_rect(
            dst,
            width,
            tile_box.x as i32,
            tile_box.y as i32,
            tile_box.width as u32,
            tile_box.height as u32,
            bg,
        );
        draw_rect_stroke(
            dst,
            width,
            height,
            tile_box.x as i32,
            tile_box.y as i32,
            tile_box.width as u32,
            tile_box.height as u32,
            1,
            border,
        );

        draw_launcher_icon(&tree, tile.icon, dst, width, height, pistil_draw_svg_icon);
        draw_tree_text(&tree, tile.label, dst, width, height, pistil_draw_text, 15.0, 0xFFF1F4F8);
        draw_tree_text(&tree, tile.path, dst, width, height, pistil_draw_text, 10.0, 0xFF9DA8B7);
    }

    draw_tree_text(&tree, nodes.status, dst, width, height, pistil_draw_text, 12.0, 0xFFADB7C7);

    if layout_debug {
        petals::draw_layout_debug(&tree, dst, width, height);
    }
}

fn draw_launcher_icon(
    tree: &petals::UiTree,
    node_id: petals::NodeId,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    pistil_draw_svg_icon: Option<DrawSvgIconFn>,
) {
    let Ok(icon_box) = tree.global_layout_box(node_id) else {
        return;
    };
    let color =
        tree.node(node_id).and_then(|node| node.style.color).map(color_argb).unwrap_or(0xFF8FD1FF);
    if let (Some(draw_svg), Some(path)) =
        (pistil_draw_svg_icon, petals::node_icon_path(tree, node_id))
    {
        if call_draw_svg_icon(
            draw_svg,
            path,
            dst,
            stride,
            height,
            icon_box.x as i32,
            icon_box.y as i32,
            icon_box.width as u32,
            icon_box.height as u32,
            color,
        ) == 0
        {
            return;
        }
    }

    let glyph = tree
        .node(node_id)
        .and_then(|node| match node.attrs.get("glyph") {
            Some(petals::AttrValue::Str(value)) => value.chars().next(),
            _ => None,
        })
        .unwrap_or('?');
    let mut fallback = String::new();
    fallback.push(glyph);
    draw_overlay_text(
        None,
        dst,
        stride,
        height,
        icon_box.x as i32 + 8,
        icon_box.y as i32 + icon_box.height as i32 - 7,
        18.0,
        &fallback,
        color,
    );
}

fn draw_tree_text(
    tree: &petals::UiTree,
    node_id: petals::NodeId,
    dst: &mut [u32],
    stride: u32,
    height: u32,
    pistil_draw_text: Option<DrawTextFn>,
    fallback_size: f32,
    fallback_color: u32,
) {
    let Some(text) = tree.node(node_id).and_then(text_attr) else {
        return;
    };
    let Ok(layout) = tree.global_layout_box(node_id) else {
        return;
    };
    let Some(node) = tree.node(node_id) else {
        return;
    };
    let px_size = node.style.font_size.unwrap_or(fallback_size);
    let color = node.style.color.map(color_argb).unwrap_or(fallback_color);
    let text = petals::ellipsize_ascii(text, text_capacity(layout.width, px_size));
    draw_overlay_text(
        pistil_draw_text,
        dst,
        stride,
        height,
        layout.x as i32,
        layout.y as i32 + px_size as i32,
        px_size,
        &text,
        color,
    );
}

fn text_attr(node: &petals::Node) -> Option<&str> {
    match node.attrs.get("text") {
        Some(petals::AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn text_capacity(width: f32, px_size: f32) -> usize {
    let advance = (px_size * 0.62).max(6.0);
    ((width / advance) as usize).max(1)
}

fn color_argb(color: petals::Color) -> u32 {
    ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32
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

/// Draw a 1-pixel border rectangle using `color` (ARGB). Delegates to
/// [`draw_rect_stroke`], which writes the color as-is (no alpha blending).
/// Suitable for layout-debug borders that need to stand out.
fn draw_debug_border_argb(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: u32,
) {
    draw_rect_stroke(dst, stride, height, x, y, w, h, 1, color);
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
        '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100],
        '=' => [0b00000, 0b11111, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        _ => [0b11111, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100],
    }
}

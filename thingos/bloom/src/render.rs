use abi::pixel::PixelFormat;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use pistil_types::Texture;

use crate::display::DisplayBackend;

const PISTIL_PATH: &str = "/lib/libpistil.so";
const PREPARE_BACKGROUND_SYMBOL: &[u8] = b"pistil_prepare_background";
const DRAW_DEBUG_TEXT_SYMBOL: &[u8] = b"pistil_draw_debug_text";
const PISTIL_DEBUG_TEXT: &str = "PISTIL FONT RENDERER\nBLOOM DLOPEN PATH OK";

type PrepareBackgroundFn = extern "C" fn(
    path: *const u8,
    dst: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32;

type DrawDebugTextFn = extern "C" fn(
    text: *const u8,
    dst: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    pistil: Option<PistilLib>,
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
    draw_debug_text: Option<DrawDebugTextFn>,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        let pistil = load_pistil();

        Self { background: None, pistil }
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
            let res = if let Some(draw_debug_text) = lib.draw_debug_text {
                stem::info!("bloom: asking pistil to draw font diagnostic text");
                call_draw_debug_text(
                    draw_debug_text,
                    PISTIL_DEBUG_TEXT,
                    texture.as_slice_mut().as_mut_ptr(),
                    width,
                    height,
                )
            } else {
                call_prepare_bg(
                    lib.prepare_bg,
                    wallpaper_path,
                    texture.as_slice_mut().as_mut_ptr(),
                    width,
                    height,
                )
            };
            if res != 0 {
                stem::error!(
                    "bloom: pistil background render failed (code {}); keeping previous wallpaper",
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
    let draw_debug_text = {
        let sym = dlsym_bytes(handle, DRAW_DEBUG_TEXT_SYMBOL);
        if sym.is_null() {
            log_dlerror("bloom: failed to resolve pistil_draw_debug_text");
            None
        } else {
            Some(unsafe { core::mem::transmute(sym) })
        }
    };
    stem::info!("bloom: pistil background renderer loaded from {}", PISTIL_PATH);
    Some(PistilLib { _handle: handle, prepare_bg, draw_debug_text })
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

fn call_draw_debug_text(
    draw_debug_text: DrawDebugTextFn,
    text: &str,
    dst: *mut u32,
    width: u32,
    height: u32,
) -> i32 {
    let bytes = text.as_bytes();
    if bytes.len() >= 256 {
        return -4;
    }

    let mut text_c = [0u8; 256];
    text_c[..bytes.len()].copy_from_slice(bytes);
    draw_debug_text(text_c.as_ptr(), dst, width, height, width)
}

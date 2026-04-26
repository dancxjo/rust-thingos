use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use abi::pixel::PixelFormat;
use pistil::compositor::pistil_prepare_background;
use pistil_types::Texture;
use spin::Mutex;

use crate::display::DisplayBackend;

type PrepareBackgroundFn = extern "C" fn(
    path: *const u8,
    dst: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
    pistil: Option<PistilLib>,
    loader: WallpaperLoader,
}

struct PistilLib {
    prepare_bg: PrepareBackgroundFn,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

/// State shared between the compositor main thread and the background wallpaper
/// decode worker.
struct WallpaperLoader {
    /// The freshly decoded texture deposited by the worker, waiting to be
    /// imported into the display driver by the render thread.
    ready: Arc<Mutex<Option<Texture>>>,
    /// Set to `true` while a worker thread is running; cleared when the worker
    /// deposits its result (or fails).  Prevents spawning duplicate workers.
    loading: Arc<AtomicBool>,
}

impl WallpaperLoader {
    fn new() -> Self {
        Self { ready: Arc::new(Mutex::new(None)), loading: Arc::new(AtomicBool::new(false)) }
    }
}

impl CompositorVisuals {
    pub fn new() -> Self {
        stem::info!("bloom: pistil background renderer ready");
        let pistil = Some(PistilLib { prepare_bg: pistil_prepare_background });

        Self { background: None, pistil, loader: WallpaperLoader::new() }
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
            let mut path_c = alloc::vec::Vec::from(wallpaper_path.as_bytes());
            path_c.push(0);

            let res = (lib.prepare_bg)(
                path_c.as_ptr(),
                texture.as_slice_mut().as_mut_ptr(),
                width,
                height,
                width,
            );
            res == 0
        } else {
            false
        };

        if !success {
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

    /// Kick off a background wallpaper decode without blocking the render loop.
    ///
    /// A worker thread is spawned that creates a [`Texture`], calls
    /// `pistil_prepare_background`, and deposits the result in `loader.ready`.
    /// The render thread must call [`poll_ready_background`] once per frame to
    /// pick up finished results.
    ///
    /// If a worker is already running the call is a no-op so repeated watch
    /// events cannot pile up pending workers.  If thread spawn fails the method
    /// falls back to a synchronous load.
    pub fn start_background_load(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        // Only one outstanding worker at a time.
        if self.loader.loading.load(Ordering::Acquire) {
            return;
        }

        let (width, height) = display.output_size();

        let prepare_bg = match self.pistil.as_ref() {
            Some(lib) => lib.prepare_bg,
            None => {
                // No pistil library: write the periwinkle solid colour directly
                // into the ready slot so poll_ready_background will pick it up.
                let mut texture =
                    match Texture::new("bloom.compositor.background", width, height, 4) {
                        Some(t) => t,
                        None => return,
                    };
                texture.as_slice_mut().fill(0xFFCCCCFF);
                *self.loader.ready.lock() = Some(texture);
                return;
            }
        };

        // Null-terminate the path for the C FFI call inside the worker.
        let mut path_c: Vec<u8> = wallpaper_path.as_bytes().to_vec();
        path_c.push(0);

        let ready_slot = self.loader.ready.clone();
        let loading_flag = self.loader.loading.clone();
        loading_flag.store(true, Ordering::Release);

        let spawn_result = stem::thread::spawn_task_detached(move || {
            let mut texture = match Texture::new("bloom.compositor.background", width, height, 4) {
                Some(t) => t,
                None => {
                    loading_flag.store(false, Ordering::Release);
                    return;
                }
            };

            let res = (prepare_bg)(
                path_c.as_ptr(),
                texture.as_slice_mut().as_mut_ptr(),
                width,
                height,
                width,
            );
            if res != 0 {
                stem::error!(
                    "bloom: wallpaper worker: decode failed (code {}); keeping previous wallpaper",
                    res
                );
                loading_flag.store(false, Ordering::Release);
                return;
            }

            *ready_slot.lock() = Some(texture);
            loading_flag.store(false, Ordering::Release);
        });

        if spawn_result.is_err() {
            // Thread spawn failed; fall back to a synchronous load so the
            // compositor always has a valid background.
            self.loader.loading.store(false, Ordering::Release);
            self.prepare_background(display, wallpaper_path);
        }
    }

    /// Check whether the background worker has finished.  If a decoded texture
    /// is waiting in the ready slot, import it into the display driver and swap
    /// it in as the current background.
    ///
    /// Returns `true` when a new background was installed (the caller should
    /// mark damage and schedule a redraw).
    pub fn poll_ready_background(&mut self, display: &DisplayBackend) -> bool {
        let texture = self.loader.ready.lock().take();
        let Some(texture) = texture else {
            return false;
        };

        let (width, height) = display.output_size();
        let Some(buffer_id) = display.import_buffer(
            texture.fd,
            width,
            height,
            texture.stride,
            PixelFormat::Bgra8888,
            0,
        ) else {
            return false;
        };

        if let Some(old) = self.background.take() {
            display.release_buffer(old.buffer_id);
        }

        self.background = Some(ServerBuffer { _texture: texture, buffer_id });
        true
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }
}

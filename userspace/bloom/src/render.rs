use abi::pixel::PixelFormat;
use petals::Texture;
use libdl::{dlopen_str, dlsym_bytes, RTLD_NOW};

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
}

struct PistilLib {
    _handle: *mut core::ffi::c_void,
    prepare_bg: PrepareBackgroundFn,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        let handle = dlopen_str("/lib/libpistil.so", RTLD_NOW);
        let pistil = if !handle.is_null() {
            let sym = dlsym_bytes(handle, b"pistil_prepare_background");
            if !sym.is_null() {
                Some(PistilLib {
                    _handle: handle,
                    prepare_bg: unsafe { core::mem::transmute(sym) },
                })
            } else {
                stem::error!("bloom: failed to find pistil_prepare_background");
                None
            }
        } else {
            stem::error!("bloom: failed to load /lib/libpistil.so");
            None
        };

        Self {
            background: None,
            pistil,
        }
    }

    pub fn prepare_background(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        let (width, height) = display.output_size();

        let mut texture: Texture = match Texture::new("bloom.compositor.background", width, height, 4) {
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
            texture.as_slice_mut().fill(0xFF333333);
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

        self.background = Some(ServerBuffer {
            _texture: texture,
            buffer_id,
        });
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }
}

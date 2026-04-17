use abi::pixel::PixelFormat;
use petals::bmp::load_bmp;
use petals::Texture;
use pistil::compositor::blit_centered_nearest;

use crate::display::DisplayBackend;

pub struct CompositorVisuals {
    background: Option<ServerBuffer>,
}

struct ServerBuffer {
    _texture: Texture,
    buffer_id: u32,
}

impl CompositorVisuals {
    pub fn new() -> Self {
        Self { background: None }
    }

    pub fn prepare_background(&mut self, display: &DisplayBackend, wallpaper_path: &str) {
        let (width, height) = display.output_size();

        let mut texture = match Texture::new("bloom.compositor.background", width, height, 4) {
            Some(t) => t,
            None => return,
        };

        {
            let pixels = texture.as_slice_mut();
            pixels.fill(0xFF000000);
        }

        if let Ok(mut wallpaper) = load_bmp(wallpaper_path) {
            let src_stride = (wallpaper.stride / 4) as usize;
            let src_w = wallpaper.width as usize;
            let src_h = wallpaper.height as usize;
            let src = wallpaper.as_slice_mut().to_vec();

            let dst_stride = (texture.stride / 4) as usize;
            let dst = texture.as_slice_mut();
            blit_centered_nearest(
                dst,
                dst_stride,
                width as usize,
                height as usize,
                &src,
                src_stride,
                src_w,
                src_h,
            );
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

        self.background = Some(ServerBuffer {
            _texture: texture,
            buffer_id,
        });
    }

    pub fn fallback_buffer_id(&self) -> Option<u32> {
        self.background.as_ref().map(|b| b.buffer_id)
    }
}

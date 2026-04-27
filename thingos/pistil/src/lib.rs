extern crate alloc;

use abi::seed::{SEED_ABI_VERSION, Seed, SeedInterface};

pub mod blit;
pub mod bmp;
pub mod compositor;
pub mod font;
pub mod geometry;
pub mod raster;
pub mod tessellate;
pub mod typography;

pub use pistil_types::{Canvas, Color, Point, Rect, Size, Texture};

const SEED_NAME: &[u8] = b"pistil";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_SEED: Seed = Seed {
    // Shared libraries are planted as inspectable Seeds but are not germinated
    // directly as Program/Lifecycle/Driver shoots.
    abi_version: SEED_ABI_VERSION,
    interface_count: 0,
    hosting_modes: 0,
    capabilities: 0,
    name_ptr: SEED_NAME.as_ptr(),
    name_len: SEED_NAME.len(),
    interfaces: [
        SeedInterface::zero(),
        SeedInterface::zero(),
        SeedInterface::zero(),
        SeedInterface::zero(),
    ],
};

pub struct Atlas {
    pub texture: Texture,
    pub next_x: u32,
    pub next_y: u32,
    pub row_h: u32,
    pub padding: u32,
}

impl Atlas {
    pub fn new(name: &str, w: u32, h: u32, bpp: u8) -> Option<Self> {
        let mut texture = Texture::new(name, w, h, bpp)?;
        texture.as_bytes_mut().fill(0);
        Some(Self { texture, next_x: 0, next_y: 0, row_h: 0, padding: 1 })
    }

    pub fn pack(&mut self, w: u32, h: u32, pixels: &[u8]) -> Option<(u32, u32)> {
        if self.next_x + w + self.padding > self.texture.width {
            self.next_x = 0;
            self.next_y += self.row_h + self.padding;
            self.row_h = 0;
        }

        if self.next_y + h + self.padding > self.texture.height {
            return None;
        }

        let x = self.next_x;
        let y = self.next_y;

        let atlas_stride = self.texture.stride as usize;
        let tex_buf = self.texture.as_bytes_mut();
        for sy in 0..h {
            let dy = y + sy;
            let dst_off = dy as usize * atlas_stride + x as usize;
            let src_off = sy as usize * w as usize;
            tex_buf[dst_off..dst_off + w as usize]
                .copy_from_slice(&pixels[src_off..src_off + w as usize]);
        }

        self.next_x += w + self.padding;
        self.row_h = self.row_h.max(h);

        Some((x, y))
    }
}

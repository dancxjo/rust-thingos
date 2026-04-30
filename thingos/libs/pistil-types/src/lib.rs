#![no_std]

extern crate alloc;

use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use serde::{Deserialize, Serialize};
use stem::syscall::{memfd_create, vfs_close, vm_map, vm_unmap};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub const fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { origin: Point::new(x, y), size: Size::new(width, height) }
    }

    pub fn x(&self) -> i32 {
        self.origin.x
    }
    pub fn y(&self) -> i32 {
        self.origin.y
    }
    pub fn width(&self) -> i32 {
        self.size.width
    }
    pub fn height(&self) -> i32 {
        self.size.height
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x0 = self.x().max(other.x());
        let y0 = self.y().max(other.y());
        let x1 = (self.x() + self.width()).min(other.x() + other.width());
        let y1 = (self.y() + self.height()).min(other.y() + other.height());

        if x1 > x0 && y1 > y0 { Some(Rect::new(x0, y0, x1 - x0, y1 - y0)) } else { None }
    }

    pub const fn is_empty(&self) -> bool {
        self.size.width <= 0 || self.size.height <= 0
    }

    pub fn clip(self, bounds: Rect) -> Self {
        let x0 = self.x().max(bounds.x());
        let y0 = self.y().max(bounds.y());
        let x1 = (self.x() + self.width()).min(bounds.x() + bounds.width());
        let y1 = (self.y() + self.height()).min(bounds.y() + bounds.height());

        if x1 <= x0 || y1 <= y0 { Self::default() } else { Self::new(x0, y0, x1 - x0, y1 - y0) }
    }

    pub fn intersect(self, other: Rect) -> Self {
        self.intersection(&other).unwrap_or_default()
    }

    pub fn expand(self, px: i32) -> Self {
        Self::new(self.x() - px, self.y() - px, self.width() + px * 2, self.height() + px * 2)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x()
            && x < self.x() + self.width()
            && y >= self.y()
            && y < self.y() + self.height()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub const fn from_u32(val: u32) -> Self {
        let a = ((val >> 24) & 0xFF) as u8;
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >> 8) & 0xFF) as u8;
        let b = (val & 0xFF) as u8;
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

pub struct Texture {
    pub fd: u32,
    pub ptr: *mut u8,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u8,
    pub size: usize,
}

impl Texture {
    pub fn new(name: &str, width: u32, height: u32, bpp: u8) -> Option<Self> {
        let stride = width * bpp as u32;
        let size = (height as usize) * (stride as usize);
        let fd = memfd_create(name, size).ok()? as u32;

        let req = VmMapReq {
            addr_hint: 0,
            len: size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::SHARED,
            backing: VmBacking::File { thing: fd, offset: 0 },
        };

        let resp = match vm_map(&req) {
            Ok(resp) => resp,
            Err(_) => {
                let _ = vfs_close(fd);
                return None;
            }
        };

        Some(Self { fd, ptr: resp.addr as *mut u8, width, height, stride, bpp, size })
    }

    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        if self.bpp != 4 {
            panic!("Texture is not 4bpp");
        }
        unsafe { core::slice::from_raw_parts_mut(self.ptr as *mut u32, self.size / 4) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let _ = vm_unmap(self.ptr as usize, self.size);
    }
}

unsafe impl Send for Texture {}

pub struct Canvas<'a> {
    pub buffer: &'a mut [u32],
    pub width: u32,
    pub height: u32,
    pub stride_pixels: u32,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u32], width: u32, height: u32, stride_pixels: u32) -> Self {
        Self { buffer, width, height, stride_pixels }
    }

    pub fn from_texture(texture: &'a mut Texture) -> Self {
        let width = texture.width;
        let height = texture.height;
        let stride_pixels = texture.stride / 4;
        Self { buffer: texture.as_slice_mut(), width, height, stride_pixels }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[(y * self.stride_pixels + x) as usize] = color;
        }
    }
}

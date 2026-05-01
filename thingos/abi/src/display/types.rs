//! Core Display Types for Thing-OS
//!
//! Aligned with the "Broker of Buffers" architectural model.

use crate::display_protocol::Rect;
use crate::pixel::PixelFormat;

/// Opaque handle for a persistent imported buffer in the driver.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufferId(pub u32);

/// Opaque handle for a hardware display plane (e.g., Primary, Cursor, Overlay).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlaneId(pub u32);

/// Standard display modes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_mhz: u32, // Refresh rate in milli-Hertz (e.g. 60000 = 60Hz)
}

/// Default display refresh rate in milli-Hertz (60 Hz).
///
/// Used as a fallback when the hardware does not report a refresh rate and in
/// software vsync implementations to derive the target frame interval.
pub const DEFAULT_REFRESH_MHZ: u32 = 60_000;

/// Conversion factor for computing frame intervals: nanoseconds-per-second × 1000.
///
/// Dividing this constant by a `refresh_mhz` value (in milli-Hertz) yields the
/// frame period in nanoseconds.
///
/// ```text
/// frame_ns = NS_PER_SECOND_PER_MILLI_HZ / refresh_mhz
/// e.g. 60 Hz → 1_000_000_000_000 / 60_000 = 16_666_667 ns ≈ 16.67 ms
/// ```
pub const NS_PER_SECOND_PER_MILLI_HZ: u64 = 1_000_000_000_000;

/// Explicit FD-backed pixel buffer description for importation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferHandle {
    /// File descriptor of the backing storage (e.g. memfd).
    pub handle: u32,
    /// Offset into the FD where pixel data begins.
    pub offset: u64,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Bytes between the start of one row and the next.
    pub stride: u32,
    /// Pixel format (using the canonical abi::pixel::PixelFormat).
    pub format: PixelFormat,
    /// Hardware-specific tiling/compression modifier (0 = linear).
    pub modifier: u64,
}

/// Description of how to display a single buffer on a plane.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaneCommit {
    /// The ID of the hardware plane to use.
    pub plane_id: PlaneId,
    /// The ID of the imported buffer to display.
    pub buffer_id: BufferId,
    /// Destination rectangle in screen coordinates.
    pub dest_rect: Rect,
    /// Source rectangle in buffer coordinates (for cropping).
    pub src_rect: Rect,
    /// Z-order/stacking order (optional, 0 = default).
    pub z_order: i32,
    /// Opacity (0 = fully transparent, 255 = fully opaque).
    pub alpha: u8,
    pub _reserved: [u8; 7],
}

/// Plane `_reserved[0]` bit: clip the plane to a rounded rectangle before
/// compositing. The radius is stored in `_reserved[1]` in destination pixels.
pub const PLANE_FLAG_CLIP_ROUNDED: u8 = 1 << 0;
pub const PLANE_RESERVED_FLAGS: usize = 0;
pub const PLANE_RESERVED_RADIUS: usize = 1;

impl PlaneCommit {
    pub const fn with_rounded_clip(mut self, radius: u8) -> Self {
        if radius > 0 {
            self._reserved[PLANE_RESERVED_FLAGS] |= PLANE_FLAG_CLIP_ROUNDED;
            self._reserved[PLANE_RESERVED_RADIUS] = radius;
        }
        self
    }

    pub const fn rounded_clip_radius(&self) -> Option<u8> {
        if self._reserved[PLANE_RESERVED_FLAGS] & PLANE_FLAG_CLIP_ROUNDED != 0
            && self._reserved[PLANE_RESERVED_RADIUS] > 0
        {
            Some(self._reserved[PLANE_RESERVED_RADIUS])
        } else {
            None
        }
    }
}

/// Atomic commit request containing multiple plane updates.
#[repr(C)]
pub struct CommitRequest {
    /// Number of plane commits in the following array.
    pub commit_count: u32,
    /// Flags (e.g. VSync, Test-only).
    pub flags: CommitFlags,
    /// Pointer to an array of PlaneCommit structures.
    pub commits_ptr: u64,
    /// Number of damaged output rectangles in the following array.
    ///
    /// A value of zero means the damage is unknown and the driver should treat
    /// the commit as full-output damage.
    pub damage_count: u32,
    pub _reserved: u32,
    /// Pointer to an array of output-space Rect structures.
    pub damage_ptr: u64,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommitFlags: u32 {
        /// Block until the next VBlank (vsync).
        const VSYNC = 1 << 0;
        /// Validation only - check if the commit would succeed without applying it.
        const TEST_ONLY = 1 << 1;
        /// Allow non-blocking presentation if the hardware supports it.
        const ASYNC = 1 << 2;
    }
}

/// Capability information for the display device.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub card_id: u32,
    pub preferred_mode: DisplayMode,
    pub plane_count: u32,
    /// Maximum number of buffers that can be imported simultaneously.
    pub max_buffers: u32,
    /// Bitmask of supported pixel formats.
    pub supported_formats: u64,
    /// Bitmask of hardware capabilities.
    pub caps: DisplayCaps,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DisplayCaps: u32 {
        /// Driver supports hardware cursor planes.
        const HARDWARE_CURSOR = 1 << 0;
        /// Driver supports hardware overlay planes.
        const OVERLAYS = 1 << 1;
        /// Driver supports atomic commits.
        const ATOMIC = 1 << 2;
        /// Driver supports VBlank events.
        const VBLANK = 1 << 3;
        /// Driver can import linear dma-buf/FD-backed client buffers.
        const DMABUF_IMPORT = 1 << 4;
    }
}

/// Request to upload a hardware cursor image and configure its hotspot.
///
/// Sent with [`DISPLAY_OP_SET_CURSOR`][super::ioctl::DISPLAY_OP_SET_CURSOR].
/// The referenced buffer must have been imported previously via
/// [`DISPLAY_OP_IMPORT_BUFFER`][super::ioctl::DISPLAY_OP_IMPORT_BUFFER].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCursorRequest {
    /// ID of the imported buffer containing the ARGB cursor image.
    pub buffer_id: BufferId,
    /// Width of the cursor image in pixels.
    pub width: u32,
    /// Height of the cursor image in pixels.
    pub height: u32,
    /// Horizontal hotspot offset within the cursor image (pixels from left).
    pub hotspot_x: u32,
    /// Vertical hotspot offset within the cursor image (pixels from top).
    pub hotspot_y: u32,
    /// Non-zero to show the cursor; zero to hide it.
    pub visible: u32,
    pub _pad: u32,
}

/// Request to move the hardware cursor hotspot to a new screen position.
///
/// Sent with [`DISPLAY_OP_MOVE_CURSOR`][super::ioctl::DISPLAY_OP_MOVE_CURSOR].
/// The driver uses the hotspot configured by the last
/// [`SetCursorRequest`] to compute where to paint the cursor image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MoveCursorRequest {
    /// New X position of the cursor hotspot in screen coordinates.
    pub x: i32,
    /// New Y position of the cursor hotspot in screen coordinates.
    pub y: i32,
    /// Non-zero to show the cursor; zero to hide it.
    pub visible: u32,
    pub _pad: u32,
}

impl CommitRequest {
    pub fn planes(&self) -> &[PlaneCommit] {
        if self.commit_count == 0 || self.commits_ptr == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.commits_ptr as *const PlaneCommit,
                    self.commit_count as usize,
                )
            }
        }
    }

    pub fn damage_rects(&self) -> &[Rect] {
        if self.damage_count == 0 || self.damage_ptr == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.damage_ptr as *const Rect,
                    self.damage_count as usize,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounded_clip_hint_uses_reserved_plane_bytes() {
        let plane = PlaneCommit {
            plane_id: PlaneId(1),
            buffer_id: BufferId(2),
            dest_rect: Rect { x: 0, y: 0, w: 100, h: 80 },
            src_rect: Rect { x: 0, y: 0, w: 100, h: 80 },
            z_order: 0,
            alpha: 255,
            _reserved: [0; 7],
        }
        .with_rounded_clip(7);

        assert_eq!(plane.rounded_clip_radius(), Some(7));
        assert_eq!(plane._reserved[PLANE_RESERVED_FLAGS] & PLANE_FLAG_CLIP_ROUNDED, 1);
        assert_eq!(plane._reserved[PLANE_RESERVED_RADIUS], 7);
    }
}

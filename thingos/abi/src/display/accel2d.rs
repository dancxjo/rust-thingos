//! 2D Acceleration Command Model for Display Backends
//!
//! This module defines a minimal vocabulary of 2D rendering commands that
//! display backends can implement progressively.  Bloom continues to submit
//! scene intent through the existing plane-list [`super::types::CommitRequest`]
//! path; these commands give backends a richer set of compositing primitives
//! for CPU or GPU-accelerated drawing without requiring a full graphics API.
//!
//! # Wire Format
//!
//! Commands are submitted via [`super::ioctl::DISPLAY_OP_ACCEL2D`].  The
//! payload begins with an [`Accel2dBatch`] header immediately followed by
//! `cmd_count` instances of [`Accel2dCommand`] (each
//! [`ACCEL2D_COMMAND_SIZE`] bytes on the wire).
//!
//! # Buffer References
//!
//! Every command that names a *destination* buffer uses [`BufferId`].
//! `BufferId(0)` is reserved as a reference to the driver-owned output
//! framebuffer.  All other `BufferId` values reference buffers previously
//! imported via `DISPLAY_OP_IMPORT_BUFFER`.
//!
//! # CPU Fallback
//!
//! The `display_bootfb` driver provides a software fallback for all commands
//! when the destination is `BufferId(0)` (the boot framebuffer).  Operations
//! targeting other buffers return `ENOSYS` on backends that do not support
//! off-screen compositing.  Capability flags in [`super::types::DisplayCaps`]
//! tell callers exactly which operations the backend supports.

use crate::display::BufferId;
use crate::display_protocol::Rect;

// ─── Command-kind discriminants ────────────────────────────────────────────

/// Fill a rectangular region of a buffer with a solid colour (ARGB8888).
pub const ACCEL2D_CMD_CLEAR_RECT: u32 = 1;

/// Copy a rectangular region from one buffer to another (no blending).
///
/// Source and destination rectangles must be the same size; no scaling is
/// applied.
pub const ACCEL2D_CMD_COPY_RECT: u32 = 2;

/// Scale-copy from a source rectangle to a (potentially different-size)
/// destination rectangle.  The `filter` field hints at interpolation quality.
pub const ACCEL2D_CMD_STRETCH_BLIT: u32 = 3;

/// Blend the source over the destination using a per-command global alpha
/// factor (0 = fully transparent, 255 = fully opaque).  Per-pixel source
/// alpha is multiplied with `global_alpha` before compositing.
pub const ACCEL2D_CMD_ALPHA_BLIT: u32 = 4;

/// Blend the source over the destination using a second *mask* buffer as the
/// per-pixel alpha.  The mask buffer's alpha channel (or luminance for opaque
/// formats) provides per-pixel coverage.
pub const ACCEL2D_CMD_MASKED_BLIT: u32 = 5;

/// Blit the source to the destination, clipped to a rounded rectangle defined
/// by `radius` pixels at each corner.
pub const ACCEL2D_CMD_ROUNDED_CLIP_BLIT: u32 = 6;

/// Hint to the backend that the listed output rectangles are dirty and need to
/// be presented to the display hardware.  A `rect_count` of 0 means the entire
/// output surface is dirty.
pub const ACCEL2D_CMD_FLUSH_DAMAGE: u32 = 7;

// ─── Scaling filter hint ───────────────────────────────────────────────────

/// Pixel-sampling hint for [`StretchBlitCmd`].
///
/// Backends may ignore this and fall back to their preferred method.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleFilter {
    /// Nearest-neighbour: fast, may appear pixelated.
    Nearest = 0,
    /// Bilinear interpolation: smoother but more expensive.
    Bilinear = 1,
}

// ─── Per-command payload structs ───────────────────────────────────────────

/// Fill `rect` in `dst_buffer` with `color` (ARGB8888, little-endian).
///
/// `dst_buffer = BufferId(0)` targets the output framebuffer.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClearRectCmd {
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    pub _pad: u32,
    /// Rectangle to fill in destination coordinates.
    pub rect: Rect,
    /// Fill colour in ARGB8888 format.
    pub color: u32,
    pub _pad2: u32,
}

/// Copy pixels from `src_rect` in `src_buffer` to `dst_rect` in `dst_buffer`.
///
/// Rectangles must be the same size; no scaling is performed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CopyRectCmd {
    /// Source buffer.
    pub src_buffer: BufferId,
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    /// Region to read from the source buffer.
    pub src_rect: Rect,
    /// Region to write in the destination buffer.
    pub dst_rect: Rect,
}

/// Scale-copy from `src_rect` in `src_buffer` to `dst_rect` in `dst_buffer`.
///
/// `filter` hints at the preferred interpolation quality.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StretchBlitCmd {
    /// Source buffer.
    pub src_buffer: BufferId,
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    /// Source crop rectangle.
    pub src_rect: Rect,
    /// Destination rectangle (may differ in size from `src_rect`).
    pub dst_rect: Rect,
    /// Interpolation hint (`ScaleFilter` cast to `u8`).
    pub filter: u8,
    pub _pad: [u8; 3],
}

/// Blend `src_buffer` over `dst_buffer` with a global alpha multiplier.
///
/// `global_alpha` (0 = fully transparent, 255 = fully opaque) is multiplied
/// with any per-pixel source alpha before compositing.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AlphaBlitCmd {
    /// Source buffer.
    pub src_buffer: BufferId,
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    /// Source crop rectangle.
    pub src_rect: Rect,
    /// Destination rectangle.
    pub dst_rect: Rect,
    /// Global opacity multiplier (0–255).
    pub global_alpha: u8,
    pub _pad: [u8; 3],
}

/// Blend `src_buffer` over `dst_buffer` using `mask_buffer` as per-pixel alpha.
///
/// The mask buffer's alpha channel provides per-pixel coverage.  For opaque
/// pixel formats, the mask's luminance is used instead.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MaskedBlitCmd {
    /// Source buffer to composite.
    pub src_buffer: BufferId,
    /// Buffer whose alpha channel (or luminance) drives per-pixel coverage.
    pub mask_buffer: BufferId,
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    pub _pad: u32,
    /// Region to sample from the source buffer.
    pub src_rect: Rect,
    /// Region to sample from the mask buffer (same size as `src_rect`).
    pub mask_rect: Rect,
    /// Destination region.
    pub dst_rect: Rect,
}

/// Blit `src_buffer` to `dst_buffer`, clipped to a rounded rectangle.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RoundedClipBlitCmd {
    /// Source buffer.
    pub src_buffer: BufferId,
    /// Destination buffer; `BufferId(0)` = output framebuffer.
    pub dst_buffer: BufferId,
    /// Source crop rectangle.
    pub src_rect: Rect,
    /// Destination rectangle.
    pub dst_rect: Rect,
    /// Corner radius in destination pixels.
    pub radius: u8,
    pub _pad: [u8; 3],
}

// ─── Flush-damage command ──────────────────────────────────────────────────

/// Maximum number of damage rectangles that fit inline in a single
/// [`FlushDamageCmd`].
pub const ACCEL2D_MAX_DAMAGE_RECTS: usize = 4;

/// Tell the backend which output regions require presentation.
///
/// A `rect_count` of 0 means the entire output surface is dirty.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FlushDamageCmd {
    /// Number of valid entries in `rects` (0 = full-surface flush).
    pub rect_count: u32,
    pub _pad: u32,
    /// Damaged output rectangles (only the first `rect_count` are valid).
    pub rects: [Rect; ACCEL2D_MAX_DAMAGE_RECTS],
}

// ─── Command body and tagged command ──────────────────────────────────────

/// Size in bytes of the command body union; equals `sizeof(FlushDamageCmd)`,
/// which is the largest variant (8 + 4 × 16 = 72 bytes).
pub const ACCEL2D_CMD_BODY_SIZE: usize = 72;

/// Union of all per-command payload types.
///
/// Select the active variant by inspecting [`Accel2dCommand::kind`].
///
/// # Safety
///
/// Reading a variant that does not match `kind` is undefined behaviour.
/// Always check `kind` before accessing a named variant.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Accel2dCommandBody {
    pub clear_rect: ClearRectCmd,
    pub copy_rect: CopyRectCmd,
    pub stretch_blit: StretchBlitCmd,
    pub alpha_blit: AlphaBlitCmd,
    pub masked_blit: MaskedBlitCmd,
    pub rounded_clip_blit: RoundedClipBlitCmd,
    pub flush_damage: FlushDamageCmd,
    /// Raw bytes; useful for zero-initialisation.
    pub _raw: [u8; ACCEL2D_CMD_BODY_SIZE],
}

/// Total on-wire size of one [`Accel2dCommand`] in bytes.
pub const ACCEL2D_COMMAND_SIZE: usize = 8 + ACCEL2D_CMD_BODY_SIZE; // = 80

/// A single 2D acceleration command (exactly [`ACCEL2D_COMMAND_SIZE`] = 80
/// bytes on the wire).
///
/// `kind` identifies which union variant in `body` is active.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Accel2dCommand {
    /// One of the `ACCEL2D_CMD_*` constants.
    pub kind: u32,
    pub _pad: u32,
    /// Command-specific payload; interpret based on `kind`.
    pub body: Accel2dCommandBody,
}

/// Batch submission header for [`super::ioctl::DISPLAY_OP_ACCEL2D`].
///
/// Immediately followed on the wire by `cmd_count` ×
/// [`Accel2dCommand`] structures.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Accel2dBatch {
    /// Number of [`Accel2dCommand`] records that follow this header.
    pub cmd_count: u32,
    pub _pad: u32,
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn flush_damage_cmd_is_largest_variant() {
        assert!(size_of::<FlushDamageCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<ClearRectCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<CopyRectCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<StretchBlitCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<AlphaBlitCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<MaskedBlitCmd>() <= ACCEL2D_CMD_BODY_SIZE);
        assert!(size_of::<RoundedClipBlitCmd>() <= ACCEL2D_CMD_BODY_SIZE);
    }

    #[test]
    fn accel2d_command_body_size_matches_union() {
        assert_eq!(size_of::<Accel2dCommandBody>(), ACCEL2D_CMD_BODY_SIZE);
    }

    #[test]
    fn accel2d_command_total_size() {
        assert_eq!(size_of::<Accel2dCommand>(), ACCEL2D_COMMAND_SIZE);
    }

    #[test]
    fn flush_damage_cmd_exact_size() {
        // Verify the constant matches the concrete type.
        assert_eq!(size_of::<FlushDamageCmd>(), ACCEL2D_CMD_BODY_SIZE);
    }

    #[test]
    fn masked_blit_cmd_fits_in_body() {
        assert_eq!(size_of::<MaskedBlitCmd>(), 64);
        assert!(64 <= ACCEL2D_CMD_BODY_SIZE);
    }
}

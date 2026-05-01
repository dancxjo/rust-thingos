//! ACCEL2D batch builder for the Bloom compositor.
//!
//! Translates compositor plane operations into `DISPLAY_OP_ACCEL2D` command
//! batches.  The builder accumulates commands in a fixed-size inline array
//! (no heap allocation) and serialises them into a byte payload ready for
//! submission to the display driver via [`abi::display::ioctl::DISPLAY_OP_ACCEL2D`].
//!
//! # Usage
//!
//! ```ignore
//! let mut batch = Accel2dBatchBuilder::new();
//! batch.copy_rect(
//!     BufferId(bg_id),
//!     Rect { x: 0, y: 0, w, h },
//!     BufferId(0),
//!     Rect { x: 0, y: 0, w, h },
//! );
//! batch.alpha_blit(
//!     BufferId(surface_id),
//!     src_rect,
//!     BufferId(0),
//!     dst_rect,
//!     alpha,
//! );
//! batch.flush_damage(&damage_rects);
//!
//! let mut payload = [0u8; MAX_BLOOM_ACCEL2D_PAYLOAD];
//! let len = batch.write_payload(&mut payload);
//! // submit payload[..len] via DISPLAY_OP_ACCEL2D
//! ```

use abi::display::BufferId;
use abi::display::accel2d::{
    ACCEL2D_CMD_ALPHA_BLIT, ACCEL2D_CMD_CLEAR_RECT, ACCEL2D_CMD_COPY_RECT,
    ACCEL2D_CMD_FLUSH_DAMAGE, ACCEL2D_CMD_ROUNDED_CLIP_BLIT, ACCEL2D_CMD_STRETCH_BLIT,
    ACCEL2D_CMD_BODY_SIZE, ACCEL2D_COMMAND_SIZE, ACCEL2D_MAX_DAMAGE_RECTS, Accel2dBatch,
    Accel2dCommand, Accel2dCommandBody, AlphaBlitCmd, ClearRectCmd, CopyRectCmd, FlushDamageCmd,
    RoundedClipBlitCmd, ScaleFilter, StretchBlitCmd,
};
use abi::display_protocol::Rect;

// ── Capacity ──────────────────────────────────────────────────────────────────

/// Maximum number of ACCEL2D commands in a single Bloom-issued batch.
///
/// Sized conservatively for:
/// - 1 background plane
/// - `MAX_COMMIT_PLANES` windows × 3 planes each (body + content + chrome)
/// - 1 pointer overlay + 1 cursor + 1 flush-damage
///
/// 64 provides comfortable headroom over the expected worst-case of ~52.
pub const MAX_BLOOM_ACCEL2D_CMDS: usize = 64;

/// Byte length of the largest possible Bloom ACCEL2D batch payload.
///
/// = `sizeof(Accel2dBatch)` + `MAX_BLOOM_ACCEL2D_CMDS` × `ACCEL2D_COMMAND_SIZE`
pub const MAX_BLOOM_ACCEL2D_PAYLOAD: usize =
    core::mem::size_of::<Accel2dBatch>() + MAX_BLOOM_ACCEL2D_CMDS * ACCEL2D_COMMAND_SIZE;

// ── Builder ───────────────────────────────────────────────────────────────────

/// A fixed-capacity, allocation-free ACCEL2D batch builder.
///
/// Call the drawing methods to accumulate commands bottom-to-top, then use
/// [`write_payload`][Self::write_payload] to serialise the batch into a byte
/// buffer suitable for submission via `DISPLAY_OP_ACCEL2D`.
///
/// All push methods silently drop commands once capacity is reached;
/// callers may check [`is_full`][Self::is_full] if saturation must be detected.
pub struct Accel2dBatchBuilder {
    cmds: [Accel2dCommand; MAX_BLOOM_ACCEL2D_CMDS],
    count: usize,
}

impl Accel2dBatchBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self { cmds: [zero_command(); MAX_BLOOM_ACCEL2D_CMDS], count: 0 }
    }

    /// Returns `true` if no commands have been accumulated yet.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns `true` when the batch is at maximum capacity.
    ///
    /// Further push calls will be silently dropped.
    pub fn is_full(&self) -> bool {
        self.count >= MAX_BLOOM_ACCEL2D_CMDS
    }

    /// Number of commands accumulated so far.
    pub fn len(&self) -> usize {
        self.count
    }

    // ── Drawing commands ──────────────────────────────────────────────────

    /// Fill `rect` in `dst` with a solid ARGB8888 `color`.
    pub fn clear_rect(&mut self, dst: BufferId, rect: Rect, color: u32) {
        if self.is_full() {
            return;
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_CLEAR_RECT,
            _pad: 0,
            body: Accel2dCommandBody {
                clear_rect: ClearRectCmd { dst_buffer: dst, _pad: 0, rect, color, _pad2: 0 },
            },
        };
        self.count += 1;
    }

    /// Pixel-exact copy of `src_rect` from `src` to `dst_rect` in `dst`
    /// (no blending).  Both rectangles must be the same size.
    pub fn copy_rect(&mut self, src: BufferId, src_rect: Rect, dst: BufferId, dst_rect: Rect) {
        if self.is_full() {
            return;
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_COPY_RECT,
            _pad: 0,
            body: Accel2dCommandBody {
                copy_rect: CopyRectCmd { src_buffer: src, dst_buffer: dst, src_rect, dst_rect },
            },
        };
        self.count += 1;
    }

    /// Scale-copy from `src_rect` in `src` to `dst_rect` in `dst`
    /// (nearest-neighbour filter hint).
    pub fn stretch_blit(&mut self, src: BufferId, src_rect: Rect, dst: BufferId, dst_rect: Rect) {
        if self.is_full() {
            return;
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_STRETCH_BLIT,
            _pad: 0,
            body: Accel2dCommandBody {
                stretch_blit: StretchBlitCmd {
                    src_buffer: src,
                    dst_buffer: dst,
                    src_rect,
                    dst_rect,
                    filter: ScaleFilter::Nearest as u8,
                    _pad: [0; 3],
                },
            },
        };
        self.count += 1;
    }

    /// Blend `src_rect` from `src` over `dst_rect` in `dst` with
    /// `global_alpha` (0 = transparent, 255 = opaque).
    pub fn alpha_blit(
        &mut self,
        src: BufferId,
        src_rect: Rect,
        dst: BufferId,
        dst_rect: Rect,
        global_alpha: u8,
    ) {
        if self.is_full() {
            return;
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_ALPHA_BLIT,
            _pad: 0,
            body: Accel2dCommandBody {
                alpha_blit: AlphaBlitCmd {
                    src_buffer: src,
                    dst_buffer: dst,
                    src_rect,
                    dst_rect,
                    global_alpha,
                    _pad: [0; 3],
                },
            },
        };
        self.count += 1;
    }

    /// Blit `src_rect` from `src` to `dst_rect` in `dst`, clipped to a
    /// rounded rectangle with `radius` pixel corner radius.
    pub fn rounded_clip_blit(
        &mut self,
        src: BufferId,
        src_rect: Rect,
        dst: BufferId,
        dst_rect: Rect,
        radius: u8,
    ) {
        if self.is_full() {
            return;
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_ROUNDED_CLIP_BLIT,
            _pad: 0,
            body: Accel2dCommandBody {
                rounded_clip_blit: RoundedClipBlitCmd {
                    src_buffer: src,
                    dst_buffer: dst,
                    src_rect,
                    dst_rect,
                    radius,
                    _pad: [0; 3],
                },
            },
        };
        self.count += 1;
    }

    /// Append a `FLUSH_DAMAGE` command listing the dirty output regions.
    ///
    /// Only the first [`ACCEL2D_MAX_DAMAGE_RECTS`] rects are sent inline;
    /// excess rects are silently dropped (the driver still flushes correctly
    /// but with fewer precise hints).  An empty `rects` slice requests a
    /// full-surface flush.
    pub fn flush_damage(&mut self, rects: &[Rect]) {
        if self.is_full() {
            return;
        }
        let n = rects.len().min(ACCEL2D_MAX_DAMAGE_RECTS);
        let mut flush = FlushDamageCmd {
            rect_count: n as u32,
            _pad: 0,
            rects: [Rect { x: 0, y: 0, w: 0, h: 0 }; ACCEL2D_MAX_DAMAGE_RECTS],
        };
        for i in 0..n {
            flush.rects[i] = rects[i];
        }
        self.cmds[self.count] = Accel2dCommand {
            kind: ACCEL2D_CMD_FLUSH_DAMAGE,
            _pad: 0,
            body: Accel2dCommandBody { flush_damage: flush },
        };
        self.count += 1;
    }

    // ── Serialisation ─────────────────────────────────────────────────────

    /// Serialise the batch header followed by all accumulated commands into
    /// `out`, returning the number of bytes written.
    ///
    /// Returns `0` when `out` is too small; callers must ensure
    /// `out.len() >= MAX_BLOOM_ACCEL2D_PAYLOAD`.
    pub fn write_payload(&self, out: &mut [u8]) -> usize {
        let needed =
            core::mem::size_of::<Accel2dBatch>() + self.count * ACCEL2D_COMMAND_SIZE;
        if out.len() < needed {
            return 0;
        }

        let batch = Accel2dBatch { cmd_count: self.count as u32, _pad: 0 };
        let mut off = 0usize;

        // Header
        let header_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &batch as *const Accel2dBatch as *const u8,
                core::mem::size_of::<Accel2dBatch>(),
            )
        };
        out[off..off + header_bytes.len()].copy_from_slice(header_bytes);
        off += header_bytes.len();

        // Commands
        let cmd_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                self.cmds.as_ptr() as *const u8,
                self.count * ACCEL2D_COMMAND_SIZE,
            )
        };
        out[off..off + cmd_bytes.len()].copy_from_slice(cmd_bytes);
        off += cmd_bytes.len();

        off
    }
}

// ── Internal ──────────────────────────────────────────────────────────────────

const fn zero_command() -> Accel2dCommand {
    Accel2dCommand {
        kind: 0,
        _pad: 0,
        body: Accel2dCommandBody { _raw: [0u8; ACCEL2D_CMD_BODY_SIZE] },
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn full_rect() -> Rect {
        Rect { x: 0, y: 0, w: 1920, h: 1080 }
    }

    #[test]
    fn new_builder_is_empty() {
        let b = Accel2dBatchBuilder::new();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
        assert!(!b.is_full());
    }

    #[test]
    fn copy_rect_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        let r = full_rect();
        b.copy_rect(BufferId(1), r, BufferId(0), r);
        assert_eq!(b.len(), 1);
        assert!(!b.is_empty());
    }

    #[test]
    fn alpha_blit_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        let r = full_rect();
        b.alpha_blit(BufferId(2), r, BufferId(0), r, 128);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn clear_rect_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        b.clear_rect(BufferId(0), full_rect(), 0xFF00_0000);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn stretch_blit_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        let src = Rect { x: 0, y: 0, w: 800, h: 600 };
        let dst = full_rect();
        b.stretch_blit(BufferId(3), src, BufferId(0), dst);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn rounded_clip_blit_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        let r = full_rect();
        b.rounded_clip_blit(BufferId(4), r, BufferId(0), r, 8);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn flush_damage_increments_count() {
        let mut b = Accel2dBatchBuilder::new();
        b.flush_damage(&[full_rect()]);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn flush_damage_empty_is_full_surface() {
        let mut b = Accel2dBatchBuilder::new();
        b.flush_damage(&[]);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn flush_damage_truncates_to_max_rects() {
        let mut b = Accel2dBatchBuilder::new();
        let rects = [full_rect(); ACCEL2D_MAX_DAMAGE_RECTS + 2];
        b.flush_damage(&rects);
        // Only the first ACCEL2D_MAX_DAMAGE_RECTS are stored; cmd is still 1.
        assert_eq!(b.len(), 1);
        let cmd = b.cmds[0];
        let flush = unsafe { cmd.body.flush_damage };
        assert_eq!(flush.rect_count, ACCEL2D_MAX_DAMAGE_RECTS as u32);
    }

    #[test]
    fn write_payload_returns_correct_size() {
        let mut b = Accel2dBatchBuilder::new();
        b.copy_rect(BufferId(1), full_rect(), BufferId(0), full_rect());
        b.flush_damage(&[full_rect()]);
        let mut out = [0u8; MAX_BLOOM_ACCEL2D_PAYLOAD];
        let len = b.write_payload(&mut out);
        let expected =
            core::mem::size_of::<Accel2dBatch>() + 2 * ACCEL2D_COMMAND_SIZE;
        assert_eq!(len, expected);
    }

    #[test]
    fn write_payload_small_buffer_returns_zero() {
        let b = Accel2dBatchBuilder::new();
        let mut tiny = [0u8; 1];
        assert_eq!(b.write_payload(&mut tiny), 0);
    }

    #[test]
    fn capacity_is_not_exceeded() {
        let mut b = Accel2dBatchBuilder::new();
        let r = full_rect();
        for _ in 0..MAX_BLOOM_ACCEL2D_CMDS + 10 {
            b.copy_rect(BufferId(1), r, BufferId(0), r);
        }
        assert_eq!(b.len(), MAX_BLOOM_ACCEL2D_CMDS);
        assert!(b.is_full());
    }
}

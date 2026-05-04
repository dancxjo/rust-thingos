//! Nine-slice window shadow primitive.
//!
//! Generates a small shadow patch texture once (on first call) and caches it.
//! Subsequent calls nine-slice blit the cached patch into the destination
//! buffer, which is vastly cheaper than re-rendering multiple blended rounded
//! rectangles per window per frame.

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Patch dimensions.  The shadow is rendered into a square patch large enough
/// to contain the full multi-layer rounded shadow plus the window inset area.
/// The nine-slice insets cut the patch into the 9 regions:
///   ┌────┬──────┬────┐
///   │ TL │  T   │ TR │
///   ├────┼──────┼────┤
///   │  L │  (C) │  R │  ← C is transparent (window covers it)
///   ├────┴──────┼────┤
///   │  BL │  B  │ BR │
///   └─────┴─────┴────┘
///
/// `INSET` is the distance from the patch edge to the window edge on each side.
/// Top/left insets are small (shadow barely extends there); right/bottom are
/// larger (shadow extends further there).

const PATCH_INSET_LEFT: u32 = 4;
const PATCH_INSET_TOP: u32 = 4;
const PATCH_INSET_RIGHT: u32 = 20;
const PATCH_INSET_BOTTOM: u32 = 24;

/// Inner content area of the patch (the "window" region that will be
/// transparent).  Must be large enough that rounded corners look correct.
const PATCH_INNER_W: u32 = 48;
const PATCH_INNER_H: u32 = 48;

const PATCH_W: u32 = PATCH_INSET_LEFT + PATCH_INNER_W + PATCH_INSET_RIGHT;
const PATCH_H: u32 = PATCH_INSET_TOP + PATCH_INNER_H + PATCH_INSET_BOTTOM;
const PATCH_PIXELS: usize = (PATCH_W * PATCH_H) as usize;

/// Near-black shadow colour: rgb(0, 0, 0).  A neutral black provides strong
/// contrast against both light and dark wallpapers.
const SHADOW_R: u32 = 0;
const SHADOW_G: u32 = 0;
const SHADOW_B: u32 = 0;
const SHADOW_RGB: u32 = (SHADOW_R << 16) | (SHADOW_G << 8) | SHADOW_B;

/// Corner radius used when generating the patch.  This should match the
/// compositor's window corner radius.  A slight mismatch is fine since the
/// shadow is blurry.
const SHADOW_CORNER_RADIUS: u32 = 10;

struct ShadowLayer {
    offset_x: i32,
    offset_y: i32,
    pad_right: u32,
    pad_bottom: u32,
    radius_add: u32,
    alpha: u32,
}

const ACTIVE_LAYERS: &[ShadowLayer] = &[
    ShadowLayer { offset_x: 7, offset_y: 8, pad_right: 6, pad_bottom: 8, radius_add: 6, alpha: 30 },
    ShadowLayer { offset_x: 4, offset_y: 5, pad_right: 3, pad_bottom: 4, radius_add: 3, alpha: 60 },
    ShadowLayer { offset_x: 2, offset_y: 2, pad_right: 1, pad_bottom: 2, radius_add: 0, alpha: 100 },
];

const INACTIVE_LAYERS: &[ShadowLayer] = &[
    ShadowLayer { offset_x: 5, offset_y: 6, pad_right: 4, pad_bottom: 6, radius_add: 4, alpha: 18 },
    ShadowLayer { offset_x: 3, offset_y: 4, pad_right: 2, pad_bottom: 3, radius_add: 2, alpha: 40 },
    ShadowLayer { offset_x: 1, offset_y: 1, pad_right: 1, pad_bottom: 1, radius_add: 0, alpha: 64 },
];

// ---------------------------------------------------------------------------
// Cached patches (active + inactive).
// ---------------------------------------------------------------------------

static PATCHES_READY: AtomicBool = AtomicBool::new(false);
static mut PATCH_ACTIVE: [u32; PATCH_PIXELS] = [0u32; PATCH_PIXELS];
static mut PATCH_INACTIVE: [u32; PATCH_PIXELS] = [0u32; PATCH_PIXELS];

fn ensure_patches() {
    if PATCHES_READY.load(Ordering::Acquire) {
        return;
    }
    // Safety: single-threaded pistil calls; worst case we render twice.
    unsafe {
        render_shadow_patch(&mut PATCH_ACTIVE, ACTIVE_LAYERS);
        render_shadow_patch(&mut PATCH_INACTIVE, INACTIVE_LAYERS);
    }
    PATCHES_READY.store(true, Ordering::Release);
}

/// Render shadow layers into the patch buffer.
fn render_shadow_patch(patch: &mut [u32; PATCH_PIXELS], layers: &[ShadowLayer]) {
    patch.fill(0); // fully transparent

    // The "window" sits at (PATCH_INSET_LEFT, PATCH_INSET_TOP) inside the
    // patch with size (PATCH_INNER_W, PATCH_INNER_H).
    let win_x = PATCH_INSET_LEFT as i32;
    let win_y = PATCH_INSET_TOP as i32;
    let win_w = PATCH_INNER_W;
    let win_h = PATCH_INNER_H;

    for layer in layers {
        let sx = win_x + layer.offset_x;
        let sy = win_y + layer.offset_y;
        let sw = win_w + layer.pad_right;
        let sh = win_h + layer.pad_bottom;
        let radius = SHADOW_CORNER_RADIUS + layer.radius_add;
        let color = (layer.alpha << 24) | SHADOW_RGB;

        fill_rounded_rect_blend(patch, PATCH_W, PATCH_H, sx, sy, sw, sh, radius, color);
    }
}

fn fill_rounded_rect_blend(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    radius: u32,
    color: u32,
) {
    if w == 0 || h == 0 {
        return;
    }
    let radius = radius.min(w / 2).min(h / 2);
    for row in 0..h {
        let inset = rounded_rect_row_inset(radius, row, h);
        if inset * 2 >= w {
            continue;
        }
        let row_w = w - inset * 2;
        let ry = y + row as i32;
        if ry < 0 || ry >= height as i32 {
            continue;
        }
        let rx = (x + inset as i32).max(0);
        let rx_end = (x + inset as i32 + row_w as i32).min(stride as i32);
        if rx >= rx_end {
            continue;
        }
        let row_off = (ry as u32 * stride) as usize;
        for px in rx as u32..rx_end as u32 {
            let idx = row_off + px as usize;
            if idx < dst.len() {
                dst[idx] = blend_over(dst[idx], color);
            }
        }
    }
}

fn rounded_rect_row_inset(radius: u32, row: u32, height: u32) -> u32 {
    if radius == 0 || height == 0 {
        return 0;
    }
    let r = radius as i64;
    let dy = if row < radius {
        r - row as i64
    } else if row >= height - radius {
        row as i64 - (height as i64 - r - 1)
    } else {
        return 0;
    };
    let dx_sq = r * r - dy * dy;
    if dx_sq <= 0 {
        return radius;
    }
    // integer sqrt
    let dx = isqrt(dx_sq as u64);
    radius.saturating_sub(dx as u32)
}

fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn blend_over(dst: u32, src: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa == 255 {
        return src;
    }
    let inv_sa = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let da = (dst >> 24) & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let out_a = sa + (da * inv_sa + 127) / 255;
    if out_a == 0 {
        return 0;
    }
    let r = (sr * sa + (dr * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let g = (sg * sa + (dg * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let b = (sb * sa + (db * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    (out_a << 24) | (r.min(255) << 16) | (g.min(255) << 8) | b.min(255)
}

// ---------------------------------------------------------------------------
// Nine-slice blitting
// ---------------------------------------------------------------------------

/// Blit a shadow around a window rectangle using the cached nine-slice patch.
///
/// `dst` is the full-screen shadow overlay buffer (ARGB, stride = dst_w).
/// `win_x/y/w/h` describe the window position in screen coordinates.
/// `active` selects the active or inactive shadow variant.
///
/// The nine-slice patch has insets that define the border regions.
/// The center of the patch (the window area) is skipped during blit since the
/// window itself will be drawn on top.
pub fn draw_nine_slice_shadow(
    dst: &mut [u32],
    dst_stride: u32,
    dst_h: u32,
    win_x: i32,
    win_y: i32,
    win_w: u32,
    win_h: u32,
    active: bool,
) {
    ensure_patches();

    let patch: &[u32; PATCH_PIXELS] = unsafe {
        if active { &PATCH_ACTIVE } else { &PATCH_INACTIVE }
    };

    // The shadow rectangle in screen coords:
    //   top-left:  (win_x - PATCH_INSET_LEFT, win_y - PATCH_INSET_TOP)
    //   size:      (win_w + PATCH_INSET_LEFT + PATCH_INSET_RIGHT,
    //               win_h + PATCH_INSET_TOP  + PATCH_INSET_BOTTOM)

    let left = PATCH_INSET_LEFT;
    let top = PATCH_INSET_TOP;
    let right = PATCH_INSET_RIGHT;
    let bottom = PATCH_INSET_BOTTOM;

    // Patch slice boundaries (in patch pixel coords)
    let p_left = left;                      // end of left border column in patch
    let p_right = PATCH_W - right;          // start of right border column
    let p_top = top;                        // end of top border row
    let p_bottom = PATCH_H - bottom;        // start of bottom border row

    // Screen coords of shadow outer edges
    let scr_x0 = win_x - left as i32;
    let scr_y0 = win_y - top as i32;
    let scr_x_right = win_x + win_w as i32;
    let scr_y_bottom = win_y + win_h as i32;

    // Top-left corner
    blit_patch_region(
        dst, dst_stride, dst_h, patch, PATCH_W,
        0, 0, p_left, p_top,
        scr_x0, scr_y0,
    );

    // Top edge (stretched horizontally)
    blit_patch_region_stretch_h(
        dst, dst_stride, dst_h, patch, PATCH_W,
        p_left, 0, p_right - p_left, p_top,
        scr_x0 + left as i32, scr_y0,
        win_w,
    );

    // Top-right corner
    blit_patch_region(
        dst, dst_stride, dst_h, patch, PATCH_W,
        p_right, 0, right, p_top,
        scr_x_right, scr_y0,
    );

    // Left edge (stretched vertically)
    blit_patch_region_stretch_v(
        dst, dst_stride, dst_h, patch, PATCH_W,
        0, p_top, p_left, p_bottom - p_top,
        scr_x0, scr_y0 + top as i32,
        win_h,
    );

    // Center: skip (transparent — window covers this)

    // Right edge (stretched vertically)
    blit_patch_region_stretch_v(
        dst, dst_stride, dst_h, patch, PATCH_W,
        p_right, p_top, right, p_bottom - p_top,
        scr_x_right, scr_y0 + top as i32,
        win_h,
    );

    // Bottom-left corner
    blit_patch_region(
        dst, dst_stride, dst_h, patch, PATCH_W,
        0, p_bottom, p_left, bottom,
        scr_x0, scr_y_bottom,
    );

    // Bottom edge (stretched horizontally)
    blit_patch_region_stretch_h(
        dst, dst_stride, dst_h, patch, PATCH_W,
        p_left, p_bottom, p_right - p_left, bottom,
        scr_x0 + left as i32, scr_y_bottom,
        win_w,
    );

    // Bottom-right corner
    blit_patch_region(
        dst, dst_stride, dst_h, patch, PATCH_W,
        p_right, p_bottom, right, bottom,
        scr_x_right, scr_y_bottom,
    );
}

/// Blit a fixed-size region from patch → dst with alpha blending.
fn blit_patch_region(
    dst: &mut [u32],
    dst_stride: u32,
    dst_h: u32,
    patch: &[u32],
    patch_stride: u32,
    // Source region in patch
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
    // Destination position
    dx: i32,
    dy: i32,
) {
    for row in 0..sh {
        let out_y = dy + row as i32;
        if out_y < 0 || out_y >= dst_h as i32 {
            continue;
        }
        let src_off = ((sy + row) * patch_stride + sx) as usize;
        let dst_off = (out_y as u32 * dst_stride) as usize;
        for col in 0..sw {
            let out_x = dx + col as i32;
            if out_x < 0 || out_x >= dst_stride as i32 {
                continue;
            }
            let src_px = patch[src_off + col as usize];
            if src_px >> 24 == 0 {
                continue;
            }
            let di = dst_off + out_x as usize;
            if di < dst.len() {
                dst[di] = blend_over(dst[di], src_px);
            }
        }
    }
}

/// Blit a region, stretching it horizontally to `out_w` pixels.
fn blit_patch_region_stretch_h(
    dst: &mut [u32],
    dst_stride: u32,
    dst_h: u32,
    patch: &[u32],
    patch_stride: u32,
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
    dx: i32,
    dy: i32,
    out_w: u32,
) {
    if sw == 0 || out_w == 0 {
        return;
    }
    for row in 0..sh {
        let out_y = dy + row as i32;
        if out_y < 0 || out_y >= dst_h as i32 {
            continue;
        }
        let src_off = ((sy + row) * patch_stride + sx) as usize;
        let dst_off = (out_y as u32 * dst_stride) as usize;
        for col in 0..out_w {
            let out_x = dx + col as i32;
            if out_x < 0 || out_x >= dst_stride as i32 {
                continue;
            }
            // Map output column back to source column (nearest-neighbour)
            let src_col = (col as u64 * sw as u64 / out_w as u64) as usize;
            let src_px = patch[src_off + src_col.min(sw as usize - 1)];
            if src_px >> 24 == 0 {
                continue;
            }
            let di = dst_off + out_x as usize;
            if di < dst.len() {
                dst[di] = blend_over(dst[di], src_px);
            }
        }
    }
}

/// Blit a region, stretching it vertically to `out_h` pixels.
fn blit_patch_region_stretch_v(
    dst: &mut [u32],
    dst_stride: u32,
    dst_h: u32,
    patch: &[u32],
    patch_stride: u32,
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
    dx: i32,
    dy: i32,
    out_h: u32,
) {
    if sh == 0 || out_h == 0 {
        return;
    }
    for row in 0..out_h {
        let out_y = dy + row as i32;
        if out_y < 0 || out_y >= dst_h as i32 {
            continue;
        }
        // Map output row back to source row
        let src_row = (row as u64 * sh as u64 / out_h as u64) as u32;
        let src_off = ((sy + src_row.min(sh - 1)) * patch_stride + sx) as usize;
        let dst_off = (out_y as u32 * dst_stride) as usize;
        for col in 0..sw {
            let out_x = dx + col as i32;
            if out_x < 0 || out_x >= dst_stride as i32 {
                continue;
            }
            let src_px = patch[src_off + col as usize];
            if src_px >> 24 == 0 {
                continue;
            }
            let di = dst_off + out_x as usize;
            if di < dst.len() {
                dst[di] = blend_over(dst[di], src_px);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// C-ABI entry point
// ---------------------------------------------------------------------------

/// Draw a nine-slice shadow behind a window.
///
/// # Parameters
/// - `dst_ptr`: pointer to the ARGB pixel buffer
/// - `dst_w`, `dst_h`: buffer dimensions
/// - `dst_stride`: buffer stride in pixels
/// - `win_x`, `win_y`: window position
/// - `win_w`, `win_h`: window size
/// - `active`: 1 for active (focused) window, 0 for inactive
///
/// Returns 0 on success.
#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_nine_slice_shadow(
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride: u32,
    win_x: i32,
    win_y: i32,
    win_w: u32,
    win_h: u32,
    active: u32,
) -> i32 {
    if dst_ptr.is_null() || dst_w == 0 || dst_h == 0 || dst_stride < dst_w {
        return -3;
    }
    if win_w == 0 || win_h == 0 {
        return 0; // nothing to shadow
    }

    let dst = unsafe {
        core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride) as usize)
    };

    draw_nine_slice_shadow(
        dst,
        dst_stride,
        dst_h,
        win_x,
        win_y,
        win_w,
        win_h,
        active != 0,
    );

    0
}

/// Returns the shadow padding values so the compositor knows how much extra
/// space to reserve around each window for the shadow.
///
/// `out` must point to 4 u32s: [left, top, right, bottom].
#[unsafe(no_mangle)]
pub extern "C" fn pistil_shadow_padding(out: *mut u32) -> i32 {
    if out.is_null() {
        return -3;
    }
    unsafe {
        *out.add(0) = PATCH_INSET_LEFT;
        *out.add(1) = PATCH_INSET_TOP;
        *out.add(2) = PATCH_INSET_RIGHT;
        *out.add(3) = PATCH_INSET_BOTTOM;
    }
    0
}

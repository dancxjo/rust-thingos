//! Opaque-region culling and composition optimisation for the Bloom
//! compositor.
//!
//! This module implements visible-region calculation from the sorted plane
//! list.  Walking planes from top to bottom, it maintains a set of covered
//! screen rects contributed by fully-opaque planes, then skips planes whose
//! entire `dest_rect` is already covered.  Planes with a full-surface opaque
//! region and alpha 255 are flagged for an opaque copy (blit) instead of
//! alpha-blend.
//!
//! # Design (V1 — simple rect list)
//!
//! 1. Walk planes from top to bottom (reverse of incoming z-order sort).
//! 2. Maintain `covered`: screen-space rects contributed by opaque planes seen
//!    so far.
//! 3. For each plane, check whether its `dest_rect` is fully contained in any
//!    single covered rect (conservative, correct for axis-aligned rects).
//!    * Fully covered → skip (hidden_regions_skipped++).
//!    * Fully opaque  → add `dest_rect` to covered, mark blit (opaque_planes_copied++).
//!    * Otherwise     → alpha_blend_planes++.
//! 4. Return surviving planes in original bottom-to-top order.
//!
//! Multi-rect coverage (several smaller opaque rects together hiding a lower
//! plane) is intentionally deferred to a future version with proper region
//! algebra.

use alloc::vec::Vec;

use abi::display_protocol::Rect;

use crate::scene::CompositionEntry;

// ── public types ─────────────────────────────────────────────────────────────

/// Debug counters produced by a single culling pass.
///
/// Counts are per-frame (not cumulative); callers should accumulate if needed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompositorCounters {
    /// Fully opaque planes that can be copied without alpha-blending.
    pub opaque_planes_copied: u64,
    /// Planes that require alpha-blending (partially transparent or no opaque
    /// region declared).
    pub alpha_blend_planes: u64,
    /// Planes skipped because they are entirely hidden by opaque planes above.
    pub hidden_regions_skipped: u64,
    /// Fullscreen opaque surfaces detected as direct-present candidates
    /// (covering the full output, no lower-plane work strictly necessary).
    pub fullscreen_direct_present: u64,
}

// ── public API ────────────────────────────────────────────────────────────────

/// Apply opaque-region culling to a **bottom-to-top sorted** composition list.
///
/// Returns a filtered list with fully-hidden planes removed, plus debug
/// counters for the pass.  The surviving planes are returned in the same
/// bottom-to-top order as the input.
///
/// `output_w` / `output_h` are used for fullscreen direct-present detection.
pub fn cull_composition(
    entries: &[CompositionEntry],
    output_w: u32,
    output_h: u32,
) -> (Vec<CompositionEntry>, CompositorCounters) {
    let mut counters = CompositorCounters::default();
    // Screen-space rects that are fully covered by opaque planes encountered
    // while walking top-to-bottom.
    let mut covered: Vec<Rect> = Vec::new();
    // `keep[i]` tracks whether entries[i] survives culling.
    let mut keep = alloc::vec![true; entries.len()];

    // Detect fullscreen direct-present: the topmost fully-opaque plane covers
    // the entire output, making the background plane redundant.
    //
    // We search from the top of the stack downward and stop at the first
    // fully-opaque fullscreen surface.
    if output_w > 0 && output_h > 0 {
        for entry in entries.iter().rev() {
            if entry.is_fullscreen && is_fully_opaque(entry) {
                let dr = entry.dest_rect;
                if dr.w >= output_w && dr.h >= output_h {
                    counters.fullscreen_direct_present += 1;
                    break;
                }
            }
        }
    }

    // Walk top-to-bottom (reverse of the z-order-sorted input).
    for (i, entry) in entries.iter().enumerate().rev() {
        if is_fully_covered(&covered, entry.dest_rect) {
            keep[i] = false;
            counters.hidden_regions_skipped += 1;
            continue;
        }
        if is_fully_opaque(entry) {
            covered.push(entry.dest_rect);
            counters.opaque_planes_copied += 1;
        } else {
            counters.alpha_blend_planes += 1;
        }
    }

    let surviving: Vec<CompositionEntry> = entries
        .iter()
        .zip(keep.iter())
        .filter(|(_, &k)| k)
        .map(|(e, _)| e.clone())
        .collect();

    (surviving, counters)
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Returns `true` when `inner` is entirely contained within `outer`.
pub fn rect_contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.x.saturating_add(inner.w) <= outer.x.saturating_add(outer.w)
        && inner.y.saturating_add(inner.h) <= outer.y.saturating_add(outer.h)
}

/// Returns `true` when at least one rectangle in `covered` fully contains
/// `rect`.
///
/// This is a conservative check: when multiple smaller rects together cover
/// `rect` but none individually does, this returns `false`.  Region algebra
/// for multi-rect coverage is deferred to a future version.
pub fn is_fully_covered(covered: &[Rect], rect: Rect) -> bool {
    if rect.w == 0 || rect.h == 0 {
        return true; // degenerate rects are trivially covered
    }
    covered.iter().any(|c| rect_contains(*c, rect))
}

/// Returns `true` when `entry` represents a fully-opaque plane.
///
/// A plane is fully opaque when **all** of the following hold:
///
/// * `alpha == 255` — per-plane opacity is fully opaque.
/// * No compositor-drawn chrome — chrome (title bars, frames, rounded corners)
///   makes the visual boundary partially transparent.
/// * The surface has an explicitly committed opaque region that covers the
///   entire source buffer area (`x = 0, y = 0, w ≥ src_rect.w,
///   h ≥ src_rect.h`).
pub fn is_fully_opaque(entry: &CompositionEntry) -> bool {
    if entry.alpha < 255 {
        return false;
    }
    // Surfaces with compositor-drawn chrome (title bars, frames) have
    // rounded corners and shadows that leave partial transparency at the
    // visual boundary.
    if !entry.chrome.is_empty() {
        return false;
    }
    let Some(opaque) = entry.opaque_region else {
        return false;
    };
    opaque.x == 0
        && opaque.y == 0
        && opaque.w >= entry.src_rect.w
        && opaque.h >= entry.src_rect.h
}

// ── unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::SurfaceChrome;

    fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }

    /// Build a minimal `CompositionEntry` for testing.
    fn entry(
        z: i32,
        dest: Rect,
        src_w: u32,
        src_h: u32,
        opaque: Option<Rect>,
        chrome: SurfaceChrome,
    ) -> CompositionEntry {
        CompositionEntry {
            surface_id: z as u32,
            buffer_id: z as u32,
            src_rect: r(0, 0, src_w, src_h),
            dest_rect: dest,
            z_order: z,
            alpha: 255,
            chrome,
            active: false,
            title: None,
            is_fullscreen: false,
            is_shaded: false,
            opaque_region: opaque,
        }
    }

    fn opaque_entry(z: i32, dest: Rect) -> CompositionEntry {
        let w = dest.w;
        let h = dest.h;
        entry(z, dest, w, h, Some(r(0, 0, w, h)), SurfaceChrome::default())
    }

    fn alpha_entry(z: i32, dest: Rect) -> CompositionEntry {
        let w = dest.w;
        let h = dest.h;
        // No opaque_region → alpha blend
        entry(z, dest, w, h, None, SurfaceChrome::default())
    }

    // ── rect_contains ────────────────────────────────────────────────────────

    #[test]
    fn rect_contains_self() {
        let a = r(10, 10, 50, 50);
        assert!(rect_contains(a, a));
    }

    #[test]
    fn rect_contains_inner() {
        let outer = r(0, 0, 100, 100);
        let inner = r(10, 10, 80, 80);
        assert!(rect_contains(outer, inner));
        assert!(!rect_contains(inner, outer));
    }

    #[test]
    fn rect_contains_touching_edges() {
        let outer = r(0, 0, 100, 100);
        // inner right edge aligns exactly with outer right edge
        let inner = r(50, 0, 50, 100);
        assert!(rect_contains(outer, inner));
    }

    #[test]
    fn rect_not_contains_partial_overlap() {
        let a = r(0, 0, 60, 60);
        let b = r(40, 40, 60, 60);
        assert!(!rect_contains(a, b));
        assert!(!rect_contains(b, a));
    }

    // ── is_fully_covered ─────────────────────────────────────────────────────

    #[test]
    fn fully_covered_single_rect() {
        let covered = alloc::vec![r(0, 0, 200, 200)];
        assert!(is_fully_covered(&covered, r(50, 50, 100, 100)));
    }

    #[test]
    fn not_covered_when_no_single_rect_contains() {
        // Two rects that together cover the target, but neither alone does.
        let covered = alloc::vec![r(0, 0, 100, 200), r(100, 0, 100, 200)];
        assert!(!is_fully_covered(&covered, r(50, 0, 100, 200)));
    }

    #[test]
    fn degenerate_rect_is_trivially_covered() {
        assert!(is_fully_covered(&[], r(0, 0, 0, 0)));
        assert!(is_fully_covered(&[], r(10, 10, 5, 0)));
    }

    // ── is_fully_opaque ──────────────────────────────────────────────────────

    #[test]
    fn opaque_entry_is_detected() {
        let e = opaque_entry(1, r(0, 0, 800, 600));
        assert!(is_fully_opaque(&e));
    }

    #[test]
    fn no_opaque_region_is_not_opaque() {
        let e = alpha_entry(1, r(0, 0, 800, 600));
        assert!(!is_fully_opaque(&e));
    }

    #[test]
    fn partial_opaque_region_is_not_fully_opaque() {
        // Opaque region covers only half the surface.
        let e = entry(1, r(0, 0, 800, 600), 800, 600, Some(r(0, 0, 400, 600)), SurfaceChrome::default());
        assert!(!is_fully_opaque(&e));
    }

    #[test]
    fn chrome_prevents_opaque_classification() {
        let chrome = SurfaceChrome { titlebar_height: 24, frame_thickness: 4 };
        let e = entry(
            1,
            r(0, 0, 800, 600),
            800,
            600,
            Some(r(0, 0, 800, 600)),
            chrome,
        );
        assert!(!is_fully_opaque(&e));
    }

    #[test]
    fn alpha_below_255_is_not_opaque() {
        let mut e = opaque_entry(1, r(0, 0, 800, 600));
        e.alpha = 200;
        assert!(!is_fully_opaque(&e));
    }

    // ── cull_composition: no-cover cases ────────────────────────────────────

    #[test]
    fn no_cover_all_planes_survive() {
        // Two non-overlapping windows.
        let entries = alloc::vec![
            opaque_entry(1, r(0, 0, 100, 100)),
            opaque_entry(2, r(200, 0, 100, 100)),
        ];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(surviving.len(), 2);
        assert_eq!(counters.hidden_regions_skipped, 0);
        assert_eq!(counters.opaque_planes_copied, 2);
        assert_eq!(counters.alpha_blend_planes, 0);
    }

    // ── cull_composition: full cover ────────────────────────────────────────

    #[test]
    fn fully_covered_lower_plane_is_skipped() {
        // Top opaque plane covers the entire lower plane.
        let lower = opaque_entry(1, r(0, 0, 100, 100));
        let upper = opaque_entry(2, r(0, 0, 100, 100)); // same rect, higher z
        let entries = alloc::vec![lower, upper];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        // Only the upper plane survives.
        assert_eq!(surviving.len(), 1);
        assert_eq!(surviving[0].z_order, 2);
        assert_eq!(counters.hidden_regions_skipped, 1);
        assert_eq!(counters.opaque_planes_copied, 1);
    }

    #[test]
    fn larger_opaque_top_hides_smaller_lower() {
        let lower = opaque_entry(1, r(10, 10, 50, 50));
        let upper = opaque_entry(2, r(0, 0, 200, 200)); // covers lower entirely
        let entries = alloc::vec![lower, upper];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(surviving.len(), 1);
        assert_eq!(counters.hidden_regions_skipped, 1);
    }

    // ── cull_composition: partial cover ─────────────────────────────────────

    #[test]
    fn partial_cover_lower_plane_survives() {
        // Upper covers only half of lower.
        let lower = opaque_entry(1, r(0, 0, 200, 200));
        let upper = opaque_entry(2, r(0, 0, 100, 200));
        let entries = alloc::vec![lower, upper];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        // Lower is only partially covered; it must survive.
        assert_eq!(surviving.len(), 2);
        assert_eq!(counters.hidden_regions_skipped, 0);
    }

    // ── cull_composition: transparent top plane ─────────────────────────────

    #[test]
    fn transparent_top_does_not_add_to_covered() {
        // A transparent top plane does NOT contribute to the covered set, so
        // the lower opaque plane must survive.
        let lower = opaque_entry(1, r(0, 0, 200, 200));
        let upper = alpha_entry(2, r(0, 0, 200, 200)); // alpha-blend, no opaque region
        let entries = alloc::vec![lower, upper];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(surviving.len(), 2);
        assert_eq!(counters.hidden_regions_skipped, 0);
        assert_eq!(counters.alpha_blend_planes, 1);
        assert_eq!(counters.opaque_planes_copied, 1);
    }

    #[test]
    fn chrome_top_does_not_occlude_lower_plane() {
        // A windowed surface with chrome (rounded corners) is NOT fully opaque;
        // the surface behind it must not be skipped.
        let lower = opaque_entry(1, r(0, 0, 300, 300));
        let chrome = SurfaceChrome { titlebar_height: 24, frame_thickness: 4 };
        let upper = entry(2, r(50, 50, 200, 200), 200, 200, Some(r(0, 0, 200, 200)), chrome);
        let entries = alloc::vec![lower, upper];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(surviving.len(), 2);
        assert_eq!(counters.hidden_regions_skipped, 0);
        assert_eq!(counters.alpha_blend_planes, 1); // upper counts as alpha
        assert_eq!(counters.opaque_planes_copied, 1); // lower is opaque
    }

    // ── cull_composition: fullscreen direct-present ──────────────────────────

    #[test]
    fn fullscreen_opaque_detected_as_direct_present() {
        let mut e = opaque_entry(1, r(0, 0, 800, 600));
        e.is_fullscreen = true;
        let entries = alloc::vec![e];
        let (_, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(counters.fullscreen_direct_present, 1);
    }

    #[test]
    fn fullscreen_alpha_not_a_direct_present_candidate() {
        let mut e = alpha_entry(1, r(0, 0, 800, 600));
        e.is_fullscreen = true;
        let entries = alloc::vec![e];
        let (_, counters) = cull_composition(&entries, 800, 600);
        assert_eq!(counters.fullscreen_direct_present, 0);
    }

    #[test]
    fn three_planes_middle_fully_covered_skipped() {
        // Background, a mid-level window, and a top-level window covering all.
        let bg = alpha_entry(0, r(0, 0, 800, 600));  // wallpaper (no opaque region)
        let mid = opaque_entry(1, r(100, 100, 200, 200));
        let top = opaque_entry(2, r(0, 0, 800, 600)); // covers everything
        let entries = alloc::vec![bg, mid, top];
        let (surviving, counters) = cull_composition(&entries, 800, 600);
        // Only the top opaque plane survives (bg and mid are hidden).
        assert_eq!(surviving.len(), 1, "expected only top plane, got {:?}", surviving.iter().map(|e| e.z_order).collect::<alloc::vec::Vec<_>>());
        assert_eq!(counters.hidden_regions_skipped, 2);
        assert_eq!(counters.opaque_planes_copied, 1);
        assert_eq!(counters.alpha_blend_planes, 0);
    }
}

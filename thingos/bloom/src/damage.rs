//! Damage tracking for the Bloom compositor.
//!
//! Damage is collected throughout a frame and then drained at present time.
//! The tracker is responsible for:
//!
//! * Clipping incoming rectangles to the bound output (so off-screen damage is
//!   never propagated to the display backend).
//! * Dropping rectangles that become empty after clipping or are otherwise
//!   degenerate.
//! * Coalescing rectangles that overlap or that are cheap to merge so the
//!   display pipeline does not have to handle a long list of fragments.
//! * Keeping cursor damage out of the main pool so that pointer motion does
//!   not inflate window/app damage and trip the full-output fallback.
//! * Falling back to a single full-output rectangle when the tracked rectangle
//!   count exceeds [`MAX_DAMAGE_RECTS`].
//!
//! See `bloom: implement smarter damage coalescing` for the design intent.

use alloc::vec::Vec;

use abi::display_protocol::Rect;

/// Threshold beyond which the tracker collapses to full-output damage.
pub const MAX_DAMAGE_RECTS: usize = 32;

/// Numerator/denominator of the "merge if cheap" heuristic.
///
/// Two rectangles are merged into their bounding box when the merged area is
/// at most `MERGE_RATIO_NUM/MERGE_RATIO_DEN` of the sum of the two original
/// areas. With `5/4` (== 1.25) this allows a small amount of slack so that
/// nearby rects collapse instead of producing a long fragment list.
const MERGE_RATIO_NUM: u64 = 5;
const MERGE_RATIO_DEN: u64 = 4;

/// Lightweight counters exposed for debugging / observability. Counts are
/// cumulative across the lifetime of the tracker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DamageCounters {
    /// Total rectangles pushed in via `mark_rect` / `push_clipped` /
    /// `mark_cursor_rect` (before clipping).
    pub raw: u64,
    /// Rectangles dropped because they were empty or fully off-output after
    /// clipping.
    pub dropped: u64,
    /// Rectangles emitted by `take()` after coalescing (only counted on takes
    /// that did not collapse to a full-output fallback).
    pub merged: u64,
    /// Number of times `take()` produced a single full-output rectangle as a
    /// fallback because the threshold was exceeded.
    pub full_fallbacks: u64,
}

pub struct DamageTracker {
    dirty: bool,
    /// Window/app damage (subject to coalescing + threshold fallback).
    regions: Vec<Rect>,
    /// Cursor damage tracked separately so that cursor motion does not
    /// inflate the main rect count or push it over the fallback threshold.
    cursor_regions: Vec<Rect>,
    /// Output bounds used for automatic clipping inside `take()`. When
    /// `None`, no automatic clipping is performed (legacy behaviour).
    output_bounds: Option<(u32, u32)>,
    counters: DamageCounters,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self {
            dirty: true,
            regions: Vec::new(),
            cursor_regions: Vec::new(),
            output_bounds: None,
            counters: DamageCounters::default(),
        }
    }

    /// Bind the tracker to a known output size. After this is called, `take()`
    /// will clip every rectangle to `[0, width) x [0, height)` and apply the
    /// full-output fallback whenever the rect count exceeds
    /// [`MAX_DAMAGE_RECTS`].
    pub fn set_output_bounds(&mut self, width: u32, height: u32) {
        self.output_bounds = Some((width, height));
    }

    /// Add a rectangle to the main damage pool without clipping. Callers that
    /// already know the output bounds should prefer [`Self::push_clipped`].
    pub fn mark_rect(&mut self, rect: Rect) {
        self.counters.raw = self.counters.raw.saturating_add(1);
        self.dirty = true;
        if rect.w == 0 || rect.h == 0 {
            self.counters.dropped = self.counters.dropped.saturating_add(1);
            return;
        }
        self.regions.push(rect);
    }

    /// Convenience: damage the entire `width x height` output.
    pub fn mark_full(&mut self, width: u32, height: u32) {
        self.mark_rect(Rect { x: 0, y: 0, w: width, h: height });
    }

    /// Push a rectangle clipped to `output_bounds`. Returns `true` when the
    /// clipped rectangle was non-empty and recorded, `false` when it was
    /// dropped. The bounds are interpreted as `[0, output_bounds.w) x [0,
    /// output_bounds.h)`; the `x`/`y` fields are ignored to match the way the
    /// compositor describes outputs.
    #[allow(dead_code)]
    pub fn push_clipped(&mut self, rect: Rect, output_bounds: Rect) -> bool {
        self.counters.raw = self.counters.raw.saturating_add(1);
        self.dirty = true;
        match clip_rect(rect, output_bounds.w, output_bounds.h) {
            Some(clipped) => {
                self.regions.push(clipped);
                true
            }
            None => {
                self.counters.dropped = self.counters.dropped.saturating_add(1);
                false
            }
        }
    }

    /// Add a rectangle to the cursor damage pool. Cursor damage is tracked
    /// independently from window/app damage so that a moving pointer does not
    /// push the main rect list over [`MAX_DAMAGE_RECTS`].
    pub fn mark_cursor_rect(&mut self, rect: Rect) {
        self.counters.raw = self.counters.raw.saturating_add(1);
        self.dirty = true;
        if rect.w == 0 || rect.h == 0 {
            self.counters.dropped = self.counters.dropped.saturating_add(1);
            return;
        }
        self.cursor_regions.push(rect);
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns `true` when the tracker is dirty but carries **only** cursor
    /// damage — no window or content damage.  Used by the compositor to decide
    /// whether hardware-cursor-only motion is sufficient (skipping a full scene
    /// recomposition).
    pub fn has_only_cursor_damage(&self) -> bool {
        self.dirty && self.regions.is_empty() && !self.cursor_regions.is_empty()
    }

    /// Coalesce the currently buffered window/app damage rects in place using
    /// the overlap + cheap-merge heuristic. Cursor damage is left untouched.
    #[allow(dead_code)]
    pub fn coalesce(&mut self) {
        if let Some((w, h)) = self.output_bounds {
            self.regions = clip_all(core::mem::take(&mut self.regions), w, h, &mut self.counters);
        } else {
            self.regions.retain(|r| r.w != 0 && r.h != 0);
        }
        coalesce_in_place(&mut self.regions);
    }

    /// Drain the tracker, returning either a coalesced list of damage rects or
    /// a single full-output fallback rectangle. Cursor damage (if any) is
    /// appended at the end and clipped to the output bounds when known.
    ///
    /// When `set_output_bounds` has not been called this falls back to the
    /// pre-coalescing behaviour (return all currently held rects, including
    /// degenerate ones), purely as a safety net for paths that have not been
    /// migrated to provide bounds yet.
    pub fn take(&mut self) -> Vec<Rect> {
        self.dirty = false;
        let cursor = core::mem::take(&mut self.cursor_regions);
        let mut rects = core::mem::take(&mut self.regions);

        let Some((w, h)) = self.output_bounds else {
            // Legacy path: no clipping/coalescing — preserve previous semantics
            // for any caller that has not yet bound an output.
            rects.extend(cursor);
            return rects;
        };

        rects = clip_all(rects, w, h, &mut self.counters);
        coalesce_in_place(&mut rects);

        if rects.len() > MAX_DAMAGE_RECTS {
            // Too many rects after coalescing — collapse to a single full-output
            // damage. Cursor damage is dropped here because it is already
            // covered by the full-output rectangle.
            self.counters.full_fallbacks = self.counters.full_fallbacks.saturating_add(1);
            let mut out = Vec::with_capacity(1);
            out.push(Rect { x: 0, y: 0, w, h });
            return out;
        }

        // Append cursor damage (clipped) on top of the main damage. Cursor
        // damage intentionally bypasses coalescing with window damage so that
        // tiny cursor rects do not get fattened into large window rects.
        let cursor_clipped = clip_all(cursor, w, h, &mut self.counters);
        rects.extend(cursor_clipped);

        // If appending cursor pushed us over the threshold, fall back to full.
        if rects.len() > MAX_DAMAGE_RECTS {
            self.counters.full_fallbacks = self.counters.full_fallbacks.saturating_add(1);
            let mut out = Vec::with_capacity(1);
            out.push(Rect { x: 0, y: 0, w, h });
            return out;
        }

        self.counters.merged = self.counters.merged.saturating_add(rects.len() as u64);
        rects
    }

    /// Return the currently buffered rects either as the explicit list (after
    /// clipping + coalescing) or as a single full-output rectangle when the
    /// threshold is exceeded. Unlike [`Self::take`] this borrows the tracker
    /// and does not clear state — useful for inspection or a peek-then-commit
    /// pipeline. Counters are not updated by this call.
    #[allow(dead_code)]
    pub fn as_rects_or_full(&self, width: u32, height: u32) -> Vec<Rect> {
        let mut rects = clip_all(self.regions.clone(), width, height, &mut DamageCounters::default());
        coalesce_in_place(&mut rects);
        if rects.len() > MAX_DAMAGE_RECTS {
            return alloc::vec![Rect { x: 0, y: 0, w: width, h: height }];
        }
        let cursor = clip_all(
            self.cursor_regions.clone(),
            width,
            height,
            &mut DamageCounters::default(),
        );
        rects.extend(cursor);
        if rects.len() > MAX_DAMAGE_RECTS {
            return alloc::vec![Rect { x: 0, y: 0, w: width, h: height }];
        }
        rects
    }

    /// Restore previously-taken damage (used when a present fails so the work
    /// is retried on the next frame).
    pub fn restore(&mut self, mut regions: Vec<Rect>) {
        if !regions.is_empty() {
            self.regions.append(&mut regions);
        }
        self.dirty = true;
    }

    /// Snapshot of the cumulative debug counters.
    #[allow(dead_code)]
    pub fn counters(&self) -> DamageCounters {
        self.counters
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Clip a single rectangle against `[0, width) x [0, height)`. Returns `None`
/// when the rectangle has no visible area inside the output.
fn clip_rect(rect: Rect, width: u32, height: u32) -> Option<Rect> {
    if width == 0 || height == 0 || rect.w == 0 || rect.h == 0 {
        return None;
    }
    // If the origin is at or beyond the far edge, the rect is fully off-output.
    if rect.x >= width || rect.y >= height {
        return None;
    }
    let x = rect.x;
    let y = rect.y;
    let w = rect.w.min(width - x);
    let h = rect.h.min(height - y);
    if w == 0 || h == 0 { None } else { Some(Rect { x, y, w, h }) }
}

/// Clip `rects` to the given output bounds, dropping degenerate ones and
/// updating `counters.dropped` accordingly.
fn clip_all(rects: Vec<Rect>, w: u32, h: u32, counters: &mut DamageCounters) -> Vec<Rect> {
    let mut out = Vec::with_capacity(rects.len());
    for rect in rects {
        match clip_rect(rect, w, h) {
            Some(clipped) => out.push(clipped),
            None => counters.dropped = counters.dropped.saturating_add(1),
        }
    }
    out
}

/// Bounding rectangle of `a` and `b`.
fn bounding(a: Rect, b: Rect) -> Rect {
    let x0 = a.x.min(b.x);
    let y0 = a.y.min(b.y);
    let x1 = a.x.saturating_add(a.w).max(b.x.saturating_add(b.w));
    let y1 = a.y.saturating_add(a.h).max(b.y.saturating_add(b.h));
    Rect { x: x0, y: y0, w: x1 - x0, h: y1 - y0 }
}

fn area(r: Rect) -> u64 {
    r.w as u64 * r.h as u64
}

/// True if `a` and `b` overlap (share at least one pixel).
fn overlaps(a: Rect, b: Rect) -> bool {
    let ax1 = a.x.saturating_add(a.w);
    let ay1 = a.y.saturating_add(a.h);
    let bx1 = b.x.saturating_add(b.w);
    let by1 = b.y.saturating_add(b.h);
    a.x < bx1 && b.x < ax1 && a.y < by1 && b.y < ay1
}

/// Decide whether `a` and `b` should be merged into their bounding rectangle.
///
/// They merge when:
/// * one fully contains the other (overlap with shared area), or
/// * the bounding-box area is at most `MERGE_RATIO_NUM/MERGE_RATIO_DEN` of the
///   sum of their individual areas — i.e. merging does not significantly
///   inflate the painted area.
fn should_merge(a: Rect, b: Rect) -> bool {
    if overlaps(a, b) {
        return true;
    }
    let merged = area(bounding(a, b));
    let sum = area(a).saturating_add(area(b));
    // merged * MERGE_RATIO_DEN <= sum * MERGE_RATIO_NUM
    merged.saturating_mul(MERGE_RATIO_DEN) <= sum.saturating_mul(MERGE_RATIO_NUM)
}

/// Iteratively merge rectangles in `rects` until no further merges apply. The
/// algorithm is O(n²) which is fine for the tracker's tiny working set
/// (bounded by [`MAX_DAMAGE_RECTS`] before fallback kicks in).
fn coalesce_in_place(rects: &mut Vec<Rect>) {
    if rects.len() < 2 {
        return;
    }
    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < rects.len() {
            let mut j = i + 1;
            while j < rects.len() {
                if should_merge(rects[i], rects[j]) {
                    rects[i] = bounding(rects[i], rects[j]);
                    rects.swap_remove(j);
                    changed = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }

    #[test]
    fn empty_rects_are_dropped() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(10, 10, 0, 5));
        d.mark_rect(r(10, 10, 5, 0));
        let out = d.take();
        assert!(out.is_empty(), "all-zero-size damage should drop, got {:?}", out);
        assert_eq!(d.counters().dropped, 2);
    }

    #[test]
    fn fully_offscreen_rects_are_dropped() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        // Push a rectangle wholly off the right edge.
        d.mark_rect(r(900, 10, 50, 50));
        let out = d.take();
        assert!(out.is_empty());
        assert!(d.counters().dropped >= 1);
    }

    #[test]
    fn rects_are_clipped_to_output_bounds() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        // 100x100 starting near the bottom-right; should clip to 50x50.
        d.mark_rect(r(750, 550, 100, 100));
        let out = d.take();
        assert_eq!(out, alloc::vec![r(750, 550, 50, 50)]);
    }

    #[test]
    fn push_clipped_returns_false_for_offscreen() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        let bounds = r(0, 0, 800, 600);
        assert!(!d.push_clipped(r(900, 0, 10, 10), bounds));
        assert!(d.push_clipped(r(10, 10, 20, 20), bounds));
        let out = d.take();
        assert_eq!(out, alloc::vec![r(10, 10, 20, 20)]);
    }

    #[test]
    fn overlapping_rects_are_merged() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(10, 10, 50, 50));
        d.mark_rect(r(40, 40, 50, 50));
        let out = d.take();
        assert_eq!(out, alloc::vec![r(10, 10, 80, 80)]);
    }

    #[test]
    fn nearby_cheap_merge_combines_rects() {
        // Two adjacent 10x10 rects with a 1-pixel gap. Bounding area = 21*10
        // = 210, sum = 200, ratio 1.05 <= 1.25 → merge.
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(0, 0, 10, 10));
        d.mark_rect(r(11, 0, 10, 10));
        let out = d.take();
        assert_eq!(out.len(), 1, "expected merge, got {:?}", out);
        assert_eq!(out[0], r(0, 0, 21, 10));
    }

    #[test]
    fn far_apart_rects_are_kept_separate() {
        // A tiny rect at (0,0) and another at (700,500): merging would create
        // a huge bounding box, so they must stay separate.
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(0, 0, 10, 10));
        d.mark_rect(r(700, 500, 10, 10));
        let out = d.take();
        assert_eq!(out.len(), 2, "expected 2 rects, got {:?}", out);
    }

    #[test]
    fn threshold_falls_back_to_full_output() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        // Push (MAX_DAMAGE_RECTS + 1) far-apart 1x1 rects so coalescing
        // cannot merge them.
        for i in 0..(MAX_DAMAGE_RECTS as u32 + 1) {
            d.mark_rect(r(i * 20, i * 10, 1, 1));
        }
        let out = d.take();
        assert_eq!(out, alloc::vec![r(0, 0, 800, 600)]);
        assert_eq!(d.counters().full_fallbacks, 1);
    }

    #[test]
    fn cursor_damage_kept_separate_from_window_damage() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(100, 100, 50, 50));
        d.mark_cursor_rect(r(400, 300, 24, 24));
        let out = d.take();
        // Cursor rect must be present and unmerged with the window rect.
        assert!(out.contains(&r(100, 100, 50, 50)));
        assert!(out.contains(&r(400, 300, 24, 24)));
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn cursor_alone_does_not_trigger_full_fallback() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        // A small number of cursor rects (e.g. old + new position) should be
        // emitted as-is, not collapsed.
        d.mark_cursor_rect(r(100, 100, 24, 24));
        d.mark_cursor_rect(r(160, 130, 24, 24));
        let out = d.take();
        assert_eq!(out.len(), 2);
        assert_eq!(d.counters().full_fallbacks, 0);
    }

    #[test]
    fn empty_take_is_empty() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        // Nothing pushed.
        let out = d.take();
        assert!(out.is_empty());
        assert_eq!(d.counters().full_fallbacks, 0);
    }

    #[test]
    fn restore_re_marks_dirty() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(800, 600);
        d.mark_rect(r(0, 0, 10, 10));
        let pending = d.take();
        assert!(!d.is_dirty());
        d.restore(pending);
        assert!(d.is_dirty());
        let out = d.take();
        assert_eq!(out, alloc::vec![r(0, 0, 10, 10)]);
    }

    #[test]
    fn counters_track_raw_and_dropped() {
        let mut d = DamageTracker::new();
        d.set_output_bounds(100, 100);
        d.mark_rect(r(0, 0, 10, 10));
        d.mark_rect(r(0, 0, 0, 0));
        d.mark_rect(r(500, 500, 10, 10));
        let _ = d.take();
        let c = d.counters();
        assert_eq!(c.raw, 3);
        assert!(c.dropped >= 2, "dropped={}", c.dropped);
    }
}

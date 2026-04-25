//! Frame-pacing clock for the Bloom compositor.
//!
//! `FrameClock` answers two questions:
//!   1. Has a repaint been requested?
//!   2. Is it time to repaint yet (have we waited at least one frame interval
//!      since the last commit)?
//!
//! The clock is *not* a timer source inside the WaitSet — the compositor loop
//! passes the result of [`FrameClock::next_deadline_ms`] as the `timeout`
//! argument to `WaitSet::wait`, so the loop wakes up naturally when the
//! deadline arrives without any separate timer FD.

use stem::time::monotonic_ns;

/// Frame-pacing clock.
pub struct FrameClock {
    /// A repaint has been requested but not yet committed.
    repaint_requested: bool,
    /// Minimum nanoseconds between successive commits (derived from
    /// `refresh_mhz` at construction).
    frame_interval_ns: u64,
    /// Monotonic nanoseconds at the time of the last display commit.
    last_commit_ns: u64,
}

impl FrameClock {
    /// Build a `FrameClock` tuned to `refresh_mhz` milli-Hertz.
    ///
    /// `refresh_mhz` uses the same units as [`crate::display::OutputInfo`]:
    /// 60 Hz → 60 000 mHz.  Passing zero falls back to 60 fps.
    pub fn new(refresh_mhz: u32) -> Self {
        // frame_interval_ns = 1e9 ns/s × 1e3 mHz/Hz / refresh_mhz
        let frame_interval_ns = if refresh_mhz > 0 {
            1_000_000_000_000u64 / refresh_mhz as u64
        } else {
            16_666_666 // ≈ 60 fps
        };
        Self {
            // Request the first paint immediately so we get a frame on boot.
            repaint_requested: true,
            frame_interval_ns,
            last_commit_ns: 0,
        }
    }

    /// Ask the clock to schedule a repaint.
    ///
    /// The repaint will be issued on the next loop iteration where the frame
    /// interval has elapsed.
    pub fn request_repaint(&mut self) {
        self.repaint_requested = true;
    }

    /// Returns `true` when a repaint has been requested *and* the minimum
    /// frame interval since the last commit has passed.
    pub fn repaint_due(&self) -> bool {
        if !self.repaint_requested {
            return false;
        }
        let now = monotonic_ns();
        let next = self.last_commit_ns.saturating_add(self.frame_interval_ns);
        now >= next
    }

    /// Milliseconds until the next repaint is allowed.
    ///
    /// Returns `Some(0)` when a repaint is due now, and `None` only when no
    /// repaint is pending.
    ///
    /// Pass this as the `timeout` to `WaitSet::wait` so the loop wakes up
    /// exactly when the next frame window opens.
    pub fn next_deadline_ms(&self) -> Option<u64> {
        if !self.repaint_requested {
            return None;
        }
        let now = monotonic_ns();
        let next = self.last_commit_ns.saturating_add(self.frame_interval_ns);
        if now >= next {
            Some(0)
        } else {
            let diff_ns = next - now;
            // Convert nanoseconds to milliseconds, rounding up to avoid
            // busy-waking a fraction of a millisecond early.
            Some((diff_ns + 999_999) / 1_000_000)
        }
    }

    /// Record that a display commit just completed.
    ///
    /// Clears the `repaint_requested` flag and records the commit timestamp so
    /// the minimum frame interval is enforced before the next repaint.
    pub fn after_commit(&mut self) {
        self.repaint_requested = false;
        self.last_commit_ns = monotonic_ns();
    }
}

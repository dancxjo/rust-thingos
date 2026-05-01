//! Bloom compositor service loop infrastructure.
//!
//! Defines the [`BloomService`] trait, the [`Interest`] / [`LoopEvent`] /
//! [`LoopAction`] vocabulary types, and the [`BloomLoop`] struct that wires
//! them together into a readiness-driven compositor event loop.
//!
//! # Design
//!
//! The compositor loop is **readiness-driven, not sleep-driven**.  One loop
//! body per wake:
//!
//! ```text
//! loop {
//!     wait(min(frame_deadline, next_timer))   ← blocks until FD ready or timeout
//!     for each ready FD → dispatch to owning service
//!     fire elapsed one-shot timers → dispatch Timer(id) to requesting service
//!     poll background reload state
//!     if frame_clock.repaint_due() && damage.is_dirty() → present + callbacks
//! }
//! ```
//!
//! Services are small, focused structs that implement [`BloomService`].  Each
//! service declares which FDs it wants to watch via [`interests()`], and the
//! loop calls [`dispatch()`] when one of those FDs fires.  Services mutate
//! shared state through [`BloomWorld`] and return a [`LoopAction`] telling the
//! loop what to schedule next.
//!
//! ## Timer support
//!
//! A service may return [`LoopAction::ArmTimer`] to request a one-shot
//! callback.  The loop tracks all pending timers and fires the soonest one on
//! each iteration, dispatching [`LoopEvent::Timer`] back to the service that
//! requested it.
//!
//! ## Wake support
//!
//! A service may return [`LoopAction::Wake`] to force an immediate extra
//! iteration (timeout = 0) without sleeping.  This is useful when a service
//! has pending work that does not produce FD readiness.
//!
//! [`interests()`]: BloomService::interests
//! [`dispatch()`]: BloomService::dispatch

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Fallback frame interval used when no display refresh rate is known yet
/// (≈ 60 fps = 16 666 666 ns per frame).
const DEFAULT_FRAME_INTERVAL_NS: u64 = 16_666_666;
const DISPLAY_POLL_INTERVAL_NS: u64 = 250_000_000;

use stem::time::monotonic_ns;
use stem::wait_set::{WaitSet, WaitToken};

use crate::frame_clock::FrameClock;
use crate::world::BloomWorld;

// ── Interest ─────────────────────────────────────────────────────────────────

/// A readiness source that a service wants to watch.
pub enum Interest {
    /// Wake when this file descriptor becomes readable.
    FdReadable(u32),
    /// Wake when this file descriptor becomes writable.
    FdWritable(u32),
}

// ── LoopEvent ────────────────────────────────────────────────────────────────

/// Events dispatched to a [`BloomService`].
#[derive(Debug, Clone, Copy)]
pub enum LoopEvent {
    /// One of the service's registered FDs became readable.
    ///
    /// The `WaitToken` can be used to distinguish multiple interests when a
    /// service registered more than one FD.
    FdReady(WaitToken),
    /// The frame-clock deadline elapsed.  The service may use this to perform
    /// time-sensitive bookkeeping even if no FD fired.
    FrameDeadline,
    /// A one-shot timer armed by a previous [`LoopAction::ArmTimer`] has
    /// fired.  `id` is the opaque value passed to the service when the timer
    /// was added so it can distinguish multiple concurrent timers.
    Timer(u64),
}

// ── LoopAction ───────────────────────────────────────────────────────────────

/// What the service wants the loop to do after `dispatch` returns.
pub enum LoopAction {
    /// No special action needed; keep running.
    None,
    /// Nudge the loop to start a new iteration immediately without sleeping.
    ///
    /// Useful when a service has enqueued work that does not produce FD
    /// readiness (e.g. an internal command queue) and needs one more pass
    /// through the loop body before it can block again.
    Wake,
    /// Arm a one-shot timer relative to *now*.
    ///
    /// When the duration elapses the loop dispatches [`LoopEvent::Timer(id)`]
    /// back to the requesting service.  `id` is an arbitrary value chosen by
    /// the service to correlate the callback with the original request.
    ///
    /// [`LoopEvent::Timer(id)`]: LoopEvent::Timer
    ArmTimer { delay: core::time::Duration, id: u64 },
    /// Ask the frame clock to schedule a repaint.
    RequestRepaint,
    /// Ask the frame clock to schedule an immediate repaint (bypassing pacing).
    RequestImmediateRepaint,
    /// Shut down the compositor gracefully.
    Shutdown,
}

// ── BloomService trait ───────────────────────────────────────────────────────

/// A composable compositor service.
///
/// Each service declares its I/O interests and receives dispatch calls when
/// those interests fire.  All shared compositor state lives in [`BloomWorld`];
/// services never reach out to each other directly.
pub trait BloomService {
    /// Human-readable name for logging and diagnostics.
    fn name(&self) -> &'static str;

    /// Optional startup action scheduled when the service is added to the loop.
    fn on_added(&mut self) -> LoopAction {
        LoopAction::None
    }

    /// The set of readiness sources this service wants to watch.
    ///
    /// Called once when the service is added to [`BloomLoop`]; the loop
    /// registers each interest in the underlying `WaitSet`.  Dynamic
    /// interest changes are not currently supported.
    fn interests(&self) -> &[Interest];

    /// React to a loop event and optionally mutate [`BloomWorld`].
    ///
    /// Returns a [`LoopAction`] instructing the loop what to do next.
    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction;
}

// ── BloomLoop ────────────────────────────────────────────────────────────────

/// Maps a `WaitToken` back to the index of its owning service.
struct Registration {
    token: WaitToken,
    svc_idx: usize,
}

/// A pending one-shot timer registered by a service via [`LoopAction::ArmTimer`].
struct PendingTimer {
    /// Monotonic nanoseconds at which this timer should fire.
    expiry_ns: u64,
    /// Index into `BloomLoop::services` of the service that requested this timer.
    svc_idx: usize,
    /// Opaque id echoed back to the service in [`LoopEvent::Timer`].
    id: u64,
}

/// The compositor event loop.
///
/// Owns the `WaitSet`, the list of registered services, the `FrameClock`, and
/// any pending one-shot timers.
/// Call [`add_service`] for each service, then [`run`] to start the loop.
///
/// [`add_service`]: BloomLoop::add_service
/// [`run`]: BloomLoop::run
pub struct BloomLoop {
    wait_set: WaitSet,
    services: Vec<Box<dyn BloomService>>,
    regs: Vec<Registration>,
    frame_clock: FrameClock,
    next_display_poll_ns: u64,
    /// Pending one-shot timers, kept in ascending expiry order.
    ///
    /// Using `VecDeque` so that draining the front (soonest timer) is O(1).
    timers: VecDeque<PendingTimer>,
}

impl BloomLoop {
    /// Create a new `BloomLoop` with the given frame clock.
    pub fn new(frame_clock: FrameClock) -> Self {
        let next_display_poll_ns = monotonic_ns().saturating_add(DISPLAY_POLL_INTERVAL_NS);
        Self {
            wait_set: WaitSet::new(),
            services: Vec::new(),
            regs: Vec::new(),
            frame_clock,
            next_display_poll_ns,
            timers: VecDeque::new(),
        }
    }

    /// Register a service.
    ///
    /// The service's [`interests()`](BloomService::interests) are added to the
    /// underlying `WaitSet` immediately.  Services are dispatched in
    /// registration order when multiple FDs fire on the same wake.
    pub fn add_service(&mut self, svc: Box<dyn BloomService>) {
        let idx = self.services.len();
        for interest in svc.interests() {
            match interest {
                Interest::FdReadable(fd) => match self.wait_set.add_fd_readable(*fd) {
                    Ok(token) => self.regs.push(Registration { token, svc_idx: idx }),
                    Err(e) => {
                        stem::warn!(
                            "bloom: failed to register {} interest on fd {}: {:?}",
                            svc.name(),
                            fd,
                            e
                        );
                    }
                },
                Interest::FdWritable(fd) => match self.wait_set.add_fd_writable(*fd) {
                    Ok(token) => self.regs.push(Registration { token, svc_idx: idx }),
                    Err(e) => {
                        stem::warn!(
                            "bloom: failed to register {} writable interest on fd {}: {:?}",
                            svc.name(),
                            fd,
                            e
                        );
                    }
                },
            }
        }
        self.services.push(svc);
        let action = self.services[idx].on_added();
        let _ = self.apply_action(action, idx);
    }

    /// Arm a one-shot timer for service `svc_idx`.
    fn arm_timer(&mut self, svc_idx: usize, delay: core::time::Duration, id: u64) {
        let now = monotonic_ns();
        // Saturate the delay to u64 to avoid silent truncation (max ≈ 584 years).
        let delay_ns = delay.as_nanos().min(u64::MAX as u128) as u64;
        let expiry_ns = now.saturating_add(delay_ns);
        // Insert sorted by expiry (ascending) so the soonest timer is at front.
        // Use `<` so ties are inserted before existing same-expiry timers (FIFO
        // within the same deadline avoids starvation).
        let pos = self.timers.partition_point(|t| t.expiry_ns < expiry_ns);
        self.timers.insert(pos, PendingTimer { expiry_ns, svc_idx, id });
    }

    /// Fire all timers whose expiry has passed and dispatch them.
    ///
    /// Returns `(fired, wake)`:
    /// - `fired` is `true` if any timers fired.
    /// - `wake` is `true` if any dispatched timer returned [`LoopAction::Wake`].
    fn fire_due_timers(&mut self, world: &mut BloomWorld) -> (bool, bool) {
        let now = monotonic_ns();
        let mut fired = false;
        let mut wake = false;
        // Pop from the front while the soonest timer is due (O(1) per pop).
        while self.timers.front().map_or(false, |t| t.expiry_ns <= now) {
            let t = self.timers.pop_front().expect("checked above");
            fired = true;
            let action = self.services[t.svc_idx].dispatch(LoopEvent::Timer(t.id), world);
            wake |= self.apply_action(action, t.svc_idx);
        }
        (fired, wake)
    }

    /// Compute the soonest timeout the loop should wait before the next
    /// iteration, taking both the frame clock and pending timers into account.
    ///
    /// Returns a bounded timeout even when no repaint is pending so Bloom can
    /// cheaply poll display metadata and react to host-driven output resizes.
    /// Returns `Some(0)` to skip sleeping entirely.
    ///
    /// All comparisons are done at nanosecond precision to preserve the full
    /// 16.67 ms frame interval without millisecond rounding.
    fn next_timeout(&self, world: &BloomWorld) -> Option<core::time::Duration> {
        let frame_ns = self.frame_clock.next_deadline_ns();
        let timer_ns = self.timers.front().map(|t| {
            let now = monotonic_ns();
            t.expiry_ns.saturating_sub(now)
        });
        let session_ns = world.next_session_fs_sync_delay_ns();

        let poll_ns = {
            let now = monotonic_ns();
            self.next_display_poll_ns.saturating_sub(now)
        };
        let ns = frame_ns
            .into_iter()
            .chain(timer_ns)
            .chain(session_ns)
            .fold(poll_ns, |soonest, ns| soonest.min(ns));
        Some(core::time::Duration::from_nanos(ns))
    }

    fn poll_display_if_due(&mut self, world: &mut BloomWorld) -> bool {
        let now = monotonic_ns();
        if now < self.next_display_poll_ns {
            return false;
        }
        self.next_display_poll_ns = now.saturating_add(DISPLAY_POLL_INTERVAL_NS);
        if world.refresh_display_output() {
            self.frame_clock.request_immediate_repaint();
            true
        } else {
            false
        }
    }

    /// Apply a `LoopAction` returned by a service dispatch.
    ///
    /// Separate from `run` so it can be called both from the FD-dispatch path
    /// and from the timer-dispatch path without borrow issues.
    ///
    /// Returns `true` when the action was [`LoopAction::Wake`], so callers can
    /// set their `wake_requested` flag.
    fn apply_action(&mut self, action: LoopAction, svc_idx: usize) -> bool {
        match action {
            LoopAction::RequestRepaint => {
                self.frame_clock.request_repaint();
                false
            }
            LoopAction::RequestImmediateRepaint => {
                self.frame_clock.request_immediate_repaint();
                true
            }
            LoopAction::Wake => true,
            LoopAction::ArmTimer { delay, id } => {
                self.arm_timer(svc_idx, delay, id);
                false
            }
            LoopAction::Shutdown => {
                stem::info!("bloom: shutdown requested by service");
                loop {
                    stem::sleep_ms(1000);
                }
            }
            LoopAction::None => false,
        }
    }

    /// Run the compositor loop forever.
    ///
    /// The loop:
    ///   1. Waits on all registered FDs with a timeout equal to the earliest
    ///      of the frame deadline and any pending one-shot timers.
    ///   2. Dispatches each ready FD to the owning service.
    ///   3. Fires any one-shot timers whose expiry has elapsed.
    ///   4. Polls background reload state.
    ///   5. Presents a frame when the frame clock is due and damage is dirty.
    pub fn run(mut self, world: &mut BloomWorld) -> ! {
        stem::info!("bloom: service loop started");
        stem::info!(
            "bloom: output0 {}x{} @ {}mHz ready",
            world.primary.width,
            world.primary.height,
            world.primary.refresh_mhz
        );
        // When any service returns `Wake`, this flag is set and the next wait
        // uses a zero timeout so the loop iterates immediately.
        let mut wake_requested = false;
        let mut first_frame_rendered = false;
        loop {
            // ── 1. Wait for the next event ────────────────────────────────
            let timeout = if wake_requested {
                wake_requested = false;
                Some(core::time::Duration::ZERO)
            } else {
                self.next_timeout(world)
            };

            let events = if self.wait_set.is_empty() {
                // No FDs registered yet; pace with the frame interval.
                // Respect a zero timeout (e.g. from Wake) by skipping the sleep
                // entirely so the loop can re-run immediately.
                let sleep_ns = timeout
                    .map(|d| d.as_nanos().min(u64::MAX as u128) as u64)
                    .unwrap_or(DEFAULT_FRAME_INTERVAL_NS);
                if sleep_ns > 0 {
                    stem::time::sleep_ns(sleep_ns);
                }
                // Still need to fire timers even without FDs.
                let (_fired, timer_wake) = self.fire_due_timers(world);
                wake_requested |= timer_wake;
                continue;
            } else {
                match self.wait_set.wait(timeout) {
                    Ok(evs) => evs,
                    Err(_) => {
                        stem::sleep_ms(1);
                        continue;
                    }
                }
            };

            // ── 2. Dispatch ready FD events ───────────────────────────────
            if events.is_empty() {
                // Timeout elapsed: could be frame deadline or a timer expiry.
            } else {
                for ev in events {
                    let svc_idx =
                        self.regs.iter().find(|r| r.token == ev.token()).map(|r| r.svc_idx);

                    if let Some(idx) = svc_idx {
                        let action =
                            self.services[idx].dispatch(LoopEvent::FdReady(ev.token()), world);
                        wake_requested |= self.apply_action(action, idx);
                    }
                }
            }

            // ── 3. Fire elapsed one-shot timers ───────────────────────────
            let (_fired, timer_wake) = self.fire_due_timers(world);
            wake_requested |= timer_wake;

            // ── 4. Poll background reload state ───────────────────────────
            if world.visuals.poll_ready_background(&world.display) {
                world.damage.mark_full(world.primary.width, world.primary.height);
                self.frame_clock.request_repaint();
            }

            // ── 5. Repaint phase ──────────────────────────────────────────
            // `repaint_due()` enforces frame pacing: it returns false until the
            // minimum frame interval has elapsed since the last commit, even if
            // a repaint was requested earlier.  Dirty scene damage and pending
            // cursor motion both need a present: cursor damage is coalesced and
            // stamped into the damage tracker immediately before committing.
            let repaint_needed = world.damage.is_dirty() || world.input.has_pending_cursor_motion();
            if self.frame_clock.repaint_due() && repaint_needed {
                if let Some(composition) = world.try_present() {
                    world.send_frame_callbacks(&composition);
                    self.frame_clock.after_commit();
                    if world.input.has_pending_cursor_motion() {
                        self.frame_clock.request_repaint();
                    }
                    if !first_frame_rendered {
                        stem::info!("First frame rendered");
                        first_frame_rendered = true;
                    }
                } else {
                    // Present failed; retry on the next frame.
                    self.frame_clock.request_repaint();
                }
            }

            // ── 6. Housekeeping ───────────────────────────────────────────
            //
            // These paths can perform synchronous VFS/display RPC.  Keep them
            // after the repaint phase so input/client events that already made
            // a frame due are not delayed by diagnostics or output probing.
            if self.poll_display_if_due(world) {
                wake_requested = true;
            }
            world.publish_wayland_session_fs_if_due();
        }
    }
}

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
//!     wait(next_frame_deadline)   ← blocks until FD ready or timeout
//!     for each ready FD → dispatch to owning service
//!     poll wallpaper worker
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
//! [`interests()`]: BloomService::interests
//! [`dispatch()`]: BloomService::dispatch

use alloc::boxed::Box;
use alloc::vec::Vec;

use stem::wait_set::{WaitSet, WaitToken};

use crate::frame_clock::FrameClock;
use crate::world::BloomWorld;

// ── Interest ─────────────────────────────────────────────────────────────────

/// A readiness source that a service wants to watch.
pub enum Interest {
    /// Wake when this file descriptor becomes readable.
    FdReadable(u32),
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
}

// ── LoopAction ───────────────────────────────────────────────────────────────

/// What the service wants the loop to do after `dispatch` returns.
pub enum LoopAction {
    /// No special action needed; keep running.
    None,
    /// Ask the frame clock to schedule a repaint.
    RequestRepaint,
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

/// The compositor event loop.
///
/// Owns the `WaitSet`, the list of registered services, and the `FrameClock`.
/// Call [`add_service`] for each service, then [`run`] to start the loop.
///
/// [`add_service`]: BloomLoop::add_service
/// [`run`]: BloomLoop::run
pub struct BloomLoop {
    wait_set: WaitSet,
    services: Vec<Box<dyn BloomService>>,
    regs: Vec<Registration>,
    frame_clock: FrameClock,
}

impl BloomLoop {
    /// Create a new `BloomLoop` with the given frame clock.
    pub fn new(frame_clock: FrameClock) -> Self {
        Self {
            wait_set: WaitSet::new(),
            services: Vec::new(),
            regs: Vec::new(),
            frame_clock,
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
                Interest::FdReadable(fd) => {
                    match self.wait_set.add_fd_readable(*fd) {
                        Ok(token) => self.regs.push(Registration { token, svc_idx: idx }),
                        Err(e) => {
                            stem::warn!(
                                "bloom: failed to register {} interest on fd {}: {:?}",
                                svc.name(),
                                fd,
                                e
                            );
                        }
                    }
                }
            }
        }
        self.services.push(svc);
    }

    /// Run the compositor loop forever.
    ///
    /// The loop:
    ///   1. Waits on all registered FDs with a timeout equal to the next frame
    ///      deadline (or indefinitely when no repaint is pending).
    ///   2. Dispatches each ready FD to the owning service.
    ///   3. Polls the wallpaper worker for completed loads.
    ///   4. Presents a frame when the frame clock is due and damage is dirty.
    pub fn run(mut self, world: &mut BloomWorld) -> ! {
        stem::info!("bloom: service loop started");
        loop {
            // ── 1. Wait for the next event or frame deadline ──────────────
            let timeout = self
                .frame_clock
                .next_deadline_ms()
                .map(core::time::Duration::from_millis);

            let events = if self.wait_set.is_empty() {
                // No FDs registered yet; pace with the frame interval.
                stem::sleep_ms(timeout.map(|d| d.as_millis() as u64).unwrap_or(16));
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
                // Timeout elapsed: frame deadline fired.
                self.frame_clock.request_repaint();
            } else {
                for ev in events {
                    let svc_idx = self
                        .regs
                        .iter()
                        .find(|r| r.token == ev.token())
                        .map(|r| r.svc_idx);

                    if let Some(idx) = svc_idx {
                        let action = self.services[idx]
                            .dispatch(LoopEvent::FdReady(ev.token()), world);
                        match action {
                            LoopAction::RequestRepaint => self.frame_clock.request_repaint(),
                            LoopAction::Shutdown => {
                                stem::info!("bloom: shutdown requested by service");
                                loop {
                                    stem::sleep_ms(1000);
                                }
                            }
                            LoopAction::None => {}
                        }
                    }
                }
            }

            // ── 3. Poll the async wallpaper worker ────────────────────────
            if world.visuals.poll_ready_background(&world.display) {
                world.damage.mark_full(world.primary.width, world.primary.height);
                self.frame_clock.request_repaint();
            }

            // ── 4. Repaint phase ──────────────────────────────────────────
            if self.frame_clock.repaint_due() && world.damage.is_dirty() {
                if let Some(composition) = world.try_present() {
                    world.send_frame_callbacks(&composition);
                    self.frame_clock.after_commit();
                } else {
                    // Present failed; retry on the next frame.
                    self.frame_clock.request_repaint();
                }
            }
        }
    }
}

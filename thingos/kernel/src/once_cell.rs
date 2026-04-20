//! Minimal no_std OnceCell primitive.
//!
//! This provides single-assignment semantics with explicit panics on:
//! - Double initialization (footgun prevention for SMP/refactor)
//! - Access before initialization (clear error message)

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, Ordering};

/// State constants for the three-phase initialization protocol.
const UNINIT: u8 = 0;
/// A writer has claimed the cell but has not yet published the value.
const WRITING: u8 = 1;
/// The value has been written and is now visible to all readers.
const READY: u8 = 2;

/// A cell that can be written to exactly once.
///
/// Unlike `spin::Once`, this panics on double-init rather than silently
/// ignoring subsequent calls.
///
/// # Memory ordering
///
/// `set` uses a three-phase protocol to avoid the race where a concurrent
/// `get` observes `initialized == true` before the value write has become
/// visible:
///
/// 1. `compare_exchange(UNINIT → WRITING, Acquire)` — atomically claims
///    the write right; any later `set` call will see `!= UNINIT` and panic.
/// 2. The value is written (no synchronisation needed; we hold exclusive
///    access via the WRITING state token).
/// 3. `store(READY, Release)` — publishes the value; any subsequent
///    `load(Acquire)` in `get` is guaranteed to see the completed write.
pub struct OnceCell<T> {
    state: AtomicU8,
    value: UnsafeCell<Option<T>>,
}

// SAFETY: The three-phase atomic protocol in `set`/`get` makes concurrent
// access safe: exactly one writer claims WRITING, writes, then publishes
// READY with Release.  All readers spin on `load(Acquire)` and only
// dereference after they observe READY.
unsafe impl<T> Send for OnceCell<T> {}
unsafe impl<T> Sync for OnceCell<T> {}

impl<T> OnceCell<T> {
    /// Creates a new uninitialized `OnceCell`.
    pub const fn new() -> Self {
        Self { state: AtomicU8::new(UNINIT), value: UnsafeCell::new(None) }
    }

    /// Sets the value exactly once.
    ///
    /// # Panics
    /// Panics if the cell has already been initialized (or is being
    /// initialized by another caller).
    pub fn set(&self, value: T) {
        // Claim the write right.  If the state is not UNINIT (either WRITING
        // or READY), another caller got here first — panic as documented.
        if self.state.compare_exchange(UNINIT, WRITING, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("OnceCell::set() called on already-initialized cell");
        }
        // SAFETY: We uniquely hold the WRITING state; no other thread can
        // read or write `value` until we store READY below.
        unsafe {
            *self.value.get() = Some(value);
        }
        // Publish the value with Release so that any Acquire load of `state`
        // in `get` is guaranteed to see the completed write to `value`.
        self.state.store(READY, Ordering::Release);
    }

    /// Gets a reference to the value.
    ///
    /// # Panics
    /// Panics if the cell has not been initialized.
    pub fn get(&self) -> &T {
        if self.state.load(Ordering::Acquire) != READY {
            panic!("OnceCell::get() called before initialization");
        }
        // SAFETY: state == READY was observed with Acquire, which synchronises
        // with the Release store in `set`, guaranteeing the value write is
        // visible here.
        unsafe {
            (*self.value.get())
                .as_ref()
                .expect("OnceCell invariant violated: state READY but value is None")
        }
    }

    /// Returns `true` if the cell has been fully initialized and its value
    /// is ready to read.
    #[allow(dead_code)]
    pub fn is_initialized(&self) -> bool {
        self.state.load(Ordering::Acquire) == READY
    }
}

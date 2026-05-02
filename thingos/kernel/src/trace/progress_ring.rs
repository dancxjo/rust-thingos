//! Lightweight "last progress marker" ring buffer for freeze diagnosis.
//!
//! This ring buffer records key system events at runtime so that, when a
//! freeze or hang is detected, the last N events can be inspected to
//! determine whether the scheduler, IRQ subsystem, or VFS RPC layer stalled.
//!
//! ## Design
//!
//! - Fixed-size (64-entry) ring buffer protected by a [`spin::Mutex`].
//! - All pushes use [`try_lock`](spin::Mutex::try_lock); if the lock is
//!   contended the entry is silently dropped.  This ensures IRQ handlers and
//!   hot scheduler paths are never blocked waiting for the ring.
//! - Consumers (e.g. a freeze-detection watchdog or manual trigger) call
//!   [`dump`] which acquires the lock, copies all entries in chronological
//!   order, and emits them via `kinfo!`.

use spin::Mutex;

/// Number of entries in the ring buffer.
const RING_SIZE: usize = 64;

/// Tag for a progress entry.  Kept as a plain `u8` so the struct stays
/// `Copy` without requiring a full enum in the ABI layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProgressTag {
    /// Slot is empty / never written.
    Empty = 0,
    /// Timer interrupt heartbeat: `data` = runqueue length.
    TimerHeartbeat = 1,
    /// Scheduler context switch completed: `data` = to_tid.
    ContextSwitch = 2,
    /// Hardware IRQ handler entered: `data` = IRQ vector.
    IrqEntry = 3,
    /// Hardware IRQ handler exited: `data` = IRQ vector.
    IrqExit = 4,
    /// VFS RPC operation entered: `data` = opcode tag.
    VfsRpcEntry = 5,
    /// VFS RPC operation exited: `data` = opcode tag.
    VfsRpcExit = 6,
}

/// A single entry in the progress ring.
#[derive(Clone, Copy)]
pub struct ProgressEntry {
    /// Event kind.
    pub tag: ProgressTag,
    /// CPU index that recorded this event (truncated to u8).
    pub cpu: u8,
    /// Monotonic timestamp in nanoseconds at the time of the event.
    pub mono_ns: u64,
    /// Event-specific payload (task ID, IRQ vector, VFS opcode, …).
    pub data: u64,
}

impl ProgressEntry {
    const EMPTY: Self =
        Self { tag: ProgressTag::Empty, cpu: 0, mono_ns: 0, data: 0 };
}

struct ProgressRing {
    buffer: [ProgressEntry; RING_SIZE],
    /// Index of the *next* slot to write (wraps around).
    head: usize,
}

impl ProgressRing {
    const fn new() -> Self {
        Self { buffer: [ProgressEntry::EMPTY; RING_SIZE], head: 0 }
    }

    fn push(&mut self, entry: ProgressEntry) {
        self.buffer[self.head] = entry;
        self.head = (self.head + 1) % RING_SIZE;
    }

    /// Copy all non-empty entries into `out` in chronological order (oldest
    /// first) and return the count written.
    fn read_all(&self, out: &mut [ProgressEntry]) -> usize {
        let mut written = 0;
        // Iterate from the oldest entry (current head) to the newest.
        for offset in 0..RING_SIZE {
            let idx = (self.head + offset) % RING_SIZE;
            let e = &self.buffer[idx];
            if e.tag == ProgressTag::Empty {
                continue;
            }
            if written >= out.len() {
                break;
            }
            out[written] = *e;
            written += 1;
        }
        written
    }
}

static PROGRESS_RING: Mutex<ProgressRing> = Mutex::new(ProgressRing::new());

/// Record a progress event.
///
/// Uses `try_lock`; the entry is silently dropped if the ring is already
/// locked by another CPU.  This is safe to call from any context including
/// IRQ handlers and the scheduler hot path.
#[inline]
pub fn push(tag: ProgressTag, cpu: usize, data: u64, mono_ns: u64) {
    if let Some(mut ring) = PROGRESS_RING.try_lock() {
        ring.push(ProgressEntry { tag, cpu: cpu as u8, mono_ns, data });
    }
}

/// Dump the progress ring to the kernel info log.
///
/// Acquires the ring lock (blocking) and emits each non-empty entry via
/// `kinfo!`.  Intended to be called from a freeze-detection path or an
/// explicit diagnostic trigger, not from hot paths.
pub fn dump() {
    let mut entries = [ProgressEntry::EMPTY; RING_SIZE];
    let count = {
        let ring = PROGRESS_RING.lock();
        ring.read_all(&mut entries)
    };
    crate::kinfo!("PROGRESS-RING: {} entries (oldest first):", count);
    for e in &entries[..count] {
        let tag_name = match e.tag {
            ProgressTag::Empty => "empty",
            ProgressTag::TimerHeartbeat => "timer_heartbeat",
            ProgressTag::ContextSwitch => "ctx_switch",
            ProgressTag::IrqEntry => "irq_entry",
            ProgressTag::IrqExit => "irq_exit",
            ProgressTag::VfsRpcEntry => "vfs_rpc_entry",
            ProgressTag::VfsRpcExit => "vfs_rpc_exit",
        };
        crate::kinfo!(
            "  PROGRESS: tag={} cpu={} mono_ns={} data={}",
            tag_name,
            e.cpu,
            e.mono_ns,
            e.data,
        );
    }
}

/// RAII guard that records a [`ProgressTag::VfsRpcExit`] event when dropped.
///
/// Hold this value for the duration of a VFS RPC call so that both entry and
/// exit are recorded in the progress ring regardless of which return path is
/// taken (normal response, error, timeout, or interrupt).
pub struct VfsRpcExitGuard {
    op_tag: u64,
}

impl VfsRpcExitGuard {
    /// Create a new exit guard for the given VFS RPC opcode.
    #[inline]
    pub fn new(op_tag: u64) -> Self {
        Self { op_tag }
    }
}

impl Drop for VfsRpcExitGuard {
    fn drop(&mut self) {
        push(ProgressTag::VfsRpcExit, 0, self.op_tag, crate::trace::now_or_zero());
    }
}

//! ServiceLoop watchdog: detects stuck, spinning, and silent daemons.
//!
//! The watchdog reads `/proc/<pid>/serviceloop/*` files published by
//! [`stem::service_loop::ServiceLoop::publish_metrics`] and flags daemons
//! that are misbehaving:
//!
//! | Condition         | Detection                                                          |
//! |-------------------|--------------------------------------------------------------------|
//! | Stuck in dispatch | `last_dispatch_start_ns > last_dispatch_end_ns` for > threshold   |
//! | Excessive wakeups | wakeup count / interval exceeds threshold (future)               |
//! | No wakeups        | `last_exit_wait_ns == 0` or not updated for > idle threshold       |
//!
//! # Usage
//!
//! Call [`check_daemons`] from Sprout's periodic supervisor tick, passing the
//! list of PIDs to monitor:
//!
//! ```no_run
//! use sprout::watchdog::check_daemons;
//! check_daemons(&[1234, 5678], 5_000_000_000);
//! ```

extern crate alloc;

use alloc::vec::Vec;

use stem::{debug, warn};

/// Threshold in nanoseconds above which a daemon is considered stuck in
/// dispatch.  Default: 5 seconds.
pub const DEFAULT_STUCK_DISPATCH_NS: u64 = 5_000_000_000;

/// Threshold in nanoseconds above which a daemon with no wakeups is
/// considered silent/frozen.  Default: 30 seconds.
pub const DEFAULT_NO_WAKEUP_NS: u64 = 30_000_000_000;

/// Snapshot of serviceloop metrics for one PID, read from procfs.
#[derive(Debug, Clone, Default)]
pub struct DaemonLoopStats {
    pub pid: u64,
    pub last_enter_wait_ns: u64,
    pub last_exit_wait_ns: u64,
    pub last_dispatch_start_ns: u64,
    pub last_dispatch_end_ns: u64,
    pub current_event_kind: u64,
}

impl DaemonLoopStats {
    /// Returns `true` when the daemon appears stuck in a dispatch handler.
    ///
    /// Condition: `last_dispatch_start_ns > last_dispatch_end_ns`
    /// and `now - last_dispatch_start_ns > threshold_ns`.
    pub fn is_stuck_in_dispatch(&self, now_ns: u64, threshold_ns: u64) -> bool {
        self.last_dispatch_start_ns > 0
            && self.last_dispatch_end_ns < self.last_dispatch_start_ns
            && now_ns.saturating_sub(self.last_dispatch_start_ns) > threshold_ns
    }

    /// Returns `true` when the daemon has never received a wakeup, or its
    /// last wakeup was longer ago than `threshold_ns`.
    ///
    /// A `last_exit_wait_ns == 0` means the daemon has not yet published
    /// any stats or has not yet woken from its first wait; this is normal
    /// immediately after startup.
    pub fn is_silent(&self, now_ns: u64, threshold_ns: u64) -> bool {
        if self.last_exit_wait_ns == 0 {
            return false; // Not yet published — not a fault.
        }
        now_ns.saturating_sub(self.last_exit_wait_ns) > threshold_ns
    }

    /// Human-readable description of the `current_event_kind` sentinel.
    pub fn event_kind_name(&self) -> &'static str {
        // Mirrors the `EVENT_KIND_*` constants in `stem::service_loop`.
        match self.current_event_kind {
            0 => "idle",
            v if v == u64::MAX - 1 => "ready",
            v if v == u64::MAX - 2 => "inbox_closed",
            v if v == u64::MAX - 3 => "timeout",
            _ => "message",
        }
    }
}

/// Read a single u64 from a procfs file, returning 0 on any error.
fn read_u64_from_proc(path: &str) -> u64 {
    let fd = match stem::syscall::vfs::vfs_open(path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return 0,
    };
    let mut buf = [0u8; 24];
    let n = stem::syscall::vfs::vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = stem::syscall::vfs::vfs_close(fd);
    let s = core::str::from_utf8(&buf[..n]).unwrap_or("").trim();
    s.parse::<u64>().unwrap_or(0)
}

/// Build a path like `/proc/<pid>/serviceloop/<metric>` into a stack buffer.
///
/// Returns a `&str` slice referencing the written portion of `buf`.
/// Panics if `buf` is too small (256 bytes is sufficient for any real PID).
fn proc_sl_path<'a>(buf: &'a mut [u8; 64], pid: u64, metric: &str) -> &'a str {
    use core::fmt::Write as _;
    let mut w = stem::utils::SliceWriter::new(buf);
    let _ = core::write!(w, "/proc/{}/serviceloop/{}", pid, metric);
    let n = w.written();
    core::str::from_utf8(&buf[..n]).unwrap_or("")
}

/// Read the serviceloop stats for a given PID from procfs.
///
/// Returns `None` if the pid has no published serviceloop stats (i.e. the
/// `/proc/<pid>/serviceloop/last_enter_wait_ns` file cannot be opened or
/// returns 0 for everything, which is indistinguishable from "never published").
pub fn read_daemon_stats(pid: u64) -> Option<DaemonLoopStats> {
    // Probe the directory to confirm stats have been published.
    let mut dir_buf = [0u8; 64];
    let dir_path = proc_sl_path(&mut dir_buf, pid, "");
    // proc_sl_path appends a trailing '/' which the VFS accepts as a dir.
    let dir_path = dir_path.trim_end_matches('/');
    match stem::syscall::vfs::vfs_open(dir_path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let _ = stem::syscall::vfs::vfs_close(fd);
        }
        Err(_) => return None,
    }

    let mut b = [0u8; 64];
    Some(DaemonLoopStats {
        pid,
        last_enter_wait_ns: read_u64_from_proc(proc_sl_path(&mut b, pid, "last_enter_wait_ns")),
        last_exit_wait_ns: read_u64_from_proc(proc_sl_path(&mut b, pid, "last_exit_wait_ns")),
        last_dispatch_start_ns: read_u64_from_proc(proc_sl_path(
            &mut b,
            pid,
            "last_dispatch_start_ns",
        )),
        last_dispatch_end_ns: read_u64_from_proc(proc_sl_path(
            &mut b,
            pid,
            "last_dispatch_end_ns",
        )),
        current_event_kind: read_u64_from_proc(proc_sl_path(
            &mut b,
            pid,
            "current_event_kind",
        )),
    })
}

/// Check a list of daemon PIDs for watchdog conditions and log warnings.
///
/// `pids`: slice of PIDs to check (e.g. all PIDs tracked by the Supervisor).
/// `stuck_threshold_ns`: nanoseconds after which a dispatch is considered stuck
///   (pass `DEFAULT_STUCK_DISPATCH_NS` for the default 5 s threshold).
pub fn check_daemons(pids: &[u64], stuck_threshold_ns: u64) {
    let now_ns = stem::time::monotonic_ns();
    for &pid in pids {
        let stats = match read_daemon_stats(pid) {
            Some(s) => s,
            None => {
                debug!("watchdog: pid {} has no serviceloop stats (not yet published)", pid);
                continue;
            }
        };

        if stats.is_stuck_in_dispatch(now_ns, stuck_threshold_ns) {
            let elapsed_ms =
                now_ns.saturating_sub(stats.last_dispatch_start_ns) / 1_000_000;
            warn!(
                "watchdog: pid {} stuck in dispatch for {}ms (op={})",
                pid,
                elapsed_ms,
                stats.event_kind_name()
            );
        } else if stats.is_silent(now_ns, DEFAULT_NO_WAKEUP_NS) {
            let elapsed_ms =
                now_ns.saturating_sub(stats.last_exit_wait_ns) / 1_000_000;
            warn!(
                "watchdog: pid {} has not received a wakeup for {}ms",
                pid, elapsed_ms
            );
        } else {
            debug!(
                "watchdog: pid {} healthy (op={}, dispatch_ns={})",
                pid,
                stats.event_kind_name(),
                stats.last_dispatch_start_ns,
            );
        }
    }
}

/// Collect PIDs of all managed tasks that have published serviceloop stats.
///
/// A convenience wrapper used by the Supervisor to build the pid list for
/// `check_daemons`.
pub fn collect_monitored_pids(
    tasks: &spin::Mutex<Vec<crate::task::ManagedTask>>,
) -> Vec<u64> {
    tasks.lock().iter().filter_map(|t| t.pid).collect()
}

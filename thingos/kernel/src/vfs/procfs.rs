//! procfs — process information filesystem mounted at `/proc`.
//!
//! Provides a read-only view of running processes and system state.
//!
//! # Paths exposed
//!
//! | Path                             | Contents |
//! |----------------------------------|----------|
//! | `/proc/version`                  | Kernel version string |
//! | `/proc/mounts`                   | Active mount table (text) |
//! | `/proc/meminfo`                  | Heap memory statistics |
//! | `/proc/cpuinfo`                  | CPU model and frequency |
//! | `/proc/uptime`                   | Seconds since boot |
//! | `/proc/self`                     | Directory for the calling process |
//! | `/proc/self/exe`                 | Symlink to calling process's executable |
//! | `/proc/self/authority`           | Canonical `thingos::authority::Authority` for the calling process |
//! | `/proc/self/place`               | Canonical `thingos::place::Place` — world/visibility context for the calling process (Phase 8) |
//! | `/proc/self/presence`            | Canonical `thingos::presence::Presence` — terminal/session person-in-place semantics |
//! | `/proc/<pid>/status`             | Process state, name, ppid |
//! | `/proc/<pid>/cmdline`            | argv as null-delimited bytes |
//! | `/proc/<pid>/exe`                | Symlink to the process's executable |
//! | `/proc/<pid>/fd/`                | Directory of open fd targets |
//! | `/proc/<pid>/task/`              | Directory of threads in the process |
//! | `/proc/<pid>/task/<tid>/name`    | Thread's human-readable name |
//! | `/proc/<pid>/task/<tid>/task`    | Canonical `thingos::task::Task` — state, job, name (Phase 5) |
//! | `/proc/<pid>/task_state`         | Canonical `thingos::task::TaskState` (Phase 1) |
//! | `/proc/<pid>/job_state`          | Canonical `thingos::job::JobState` (Phase 2) |
//! | `/proc/<pid>/job_exit`           | Canonical `thingos::job::JobExit` — state + code (Phase 3) |
//! | `/proc/<pid>/job_wait`           | Canonical `thingos::job::JobWaitResult` — non-blocking poll (Phase 3) |
//! | `/proc/<pid>/group_kind`         | Canonical `thingos::group::GroupKind` — coordination role (Phase 4)     |
//! | `/proc/<pid>/foreground_group`   | Canonical foreground membership — `true`/`false` in Group terms (Phase 4) |
//! | `/proc/<pid>/authority`          | Canonical `thingos::authority::Authority` — permission context (Phase 7) |
//! | `/proc/<pid>/place`              | Canonical `thingos::place::Place` — world/visibility context (Phase 8)  |
//! | `/proc/<pid>/presence`           | Canonical `thingos::presence::Presence` — terminal/session person-in-place semantics |
//! | `/proc/<pid>/job_observer`       | Write-only registration of caller inbox as `JobExit` observer |
//! | `/proc/<pid>/serviceloop/`       | ServiceLoop diagnostic directory (state, name, last\_event, and precision timestamps) |
//! | `/proc/<pid>/serviceloop/name`            | Human-readable loop name |
//! | `/proc/<pid>/serviceloop/state`           | Coarse loop state (idle/waiting/dispatching/shutdown) |
//! | `/proc/<pid>/serviceloop/last_event`      | Label of the last dispatched event |
//! | `/proc/<pid>/serviceloop/last_dispatch_ns`| Monotonic ns of last dispatch |
//! | `/proc/<pid>/serviceloop/wakeups`         | Total wakeup count |
//! | `/proc/<pid>/serviceloop/timeouts`        | Timeout wakeup count |
//! | `/proc/<pid>/serviceloop/errors`          | Dispatch error count |
//! | `/proc/<pid>/serviceloop/last_enter_wait_ns`    | Monotonic ns of last wait entry |
//! | `/proc/<pid>/serviceloop/last_exit_wait_ns`     | Monotonic ns of last wait exit |
//! | `/proc/<pid>/serviceloop/last_dispatch_start_ns`| Monotonic ns of last dispatch start |
//! | `/proc/<pid>/serviceloop/last_dispatch_end_ns`  | Monotonic ns of last dispatch end |
//! | `/proc/<pid>/serviceloop/current_event_kind`    | Current event kind sentinel or KindId high bytes |
//! | `/proc/self/serviceloop/stats`   | Write-only: publish ServiceLoop metrics from userspace |

use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};
use spin::Mutex;

use super::{VfsDriver, VfsNode, VfsStat};

// ── ServiceLoop stats registry ────────────────────────────────────────────────

/// Per-loop instrumentation snapshot stored by a userspace daemon via
/// `/proc/self/serviceloop/stats`.
#[derive(Clone, Debug, Default)]
pub struct ServiceLoopStats {
    pub last_enter_wait_ns: u64,
    pub last_exit_wait_ns: u64,
    pub last_dispatch_start_ns: u64,
    pub last_dispatch_end_ns: u64,
    pub current_event_kind: u64,
}

impl ServiceLoopStats {
    /// Parse the text format written by `stem::service_loop::ServiceLoop::publish_metrics`.
    ///
    /// Format:
    /// ```text
    /// last_enter_wait_ns: <u64>
    /// last_exit_wait_ns: <u64>
    /// last_dispatch_start_ns: <u64>
    /// last_dispatch_end_ns: <u64>
    /// current_event_kind: <u64>
    /// ```
    fn from_text(text: &str) -> Option<Self> {
        let mut s = Self::default();
        let mut found = 0u8;
        for line in text.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("last_enter_wait_ns:") {
                s.last_enter_wait_ns = v.trim().parse().ok()?;
                found |= 1;
            } else if let Some(v) = line.strip_prefix("last_exit_wait_ns:") {
                s.last_exit_wait_ns = v.trim().parse().ok()?;
                found |= 2;
            } else if let Some(v) = line.strip_prefix("last_dispatch_start_ns:") {
                s.last_dispatch_start_ns = v.trim().parse().ok()?;
                found |= 4;
            } else if let Some(v) = line.strip_prefix("last_dispatch_end_ns:") {
                s.last_dispatch_end_ns = v.trim().parse().ok()?;
                found |= 8;
            } else if let Some(v) = line.strip_prefix("current_event_kind:") {
                s.current_event_kind = v.trim().parse().ok()?;
                found |= 16;
            }
        }
        // All 5 fields are required.
        if found == 0b11111 { Some(s) } else { None }
    }

    /// Format the stats as a newline-terminated text block.
    fn as_text(&self) -> String {
        alloc::format!(
            "last_enter_wait_ns: {}\nlast_exit_wait_ns: {}\nlast_dispatch_start_ns: {}\nlast_dispatch_end_ns: {}\ncurrent_event_kind: {}\n",
            self.last_enter_wait_ns,
            self.last_exit_wait_ns,
            self.last_dispatch_start_ns,
            self.last_dispatch_end_ns,
            self.current_event_kind,
        )
    }
}

/// Global per-pid service loop stats registry.
///
/// Written by userspace via `/proc/self/serviceloop/stats`, read via
/// `/proc/<pid>/serviceloop/*`.
static SERVICE_LOOP_STATS: Mutex<BTreeMap<u32, ServiceLoopStats>> =
    Mutex::new(BTreeMap::new());

/// Store (or update) service loop stats for the given PID.
pub fn publish_service_loop_stats(pid: u32, stats: ServiceLoopStats) {
    SERVICE_LOOP_STATS.lock().insert(pid, stats);
}

/// Retrieve service loop stats for the given PID, if any have been published.
pub fn get_service_loop_stats(pid: u32) -> Option<ServiceLoopStats> {
    SERVICE_LOOP_STATS.lock().get(&pid).cloned()
}

/// Remove service loop stats for the given PID (called on process exit).
pub fn remove_service_loop_stats(pid: u32) {
    SERVICE_LOOP_STATS.lock().remove(&pid);
}

// ── ProcFs driver ─────────────────────────────────────────────────────────────

/// The process filesystem driver.  Mounted at `/proc` by `vfs::init`.
pub struct ProcFs;

impl ProcFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for ProcFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match path {
            "" => Ok(Arc::new(ProcDirNode)),
            "version" => Ok(Arc::new(StaticTextNode::new(b"Thing-OS v0.2\n", 2))),
            "mounts" => Ok(Arc::new(MountsNode)),
            "meminfo" => Ok(Arc::new(MemInfoNode)),
            "cpuinfo" => Ok(Arc::new(CpuInfoNode)),
            "uptime" => Ok(Arc::new(UptimeNode)),
            // /proc/ipc — IPC diagnostics directory
            "ipc" => Ok(Arc::new(IpcDirNode)),
            "ipc/ports" => Ok(Arc::new(IpcDiagNode::ports())),
            "ipc/pipes" => Ok(Arc::new(IpcDiagNode::pipes())),
            "ipc/vfs_rpc" => Ok(Arc::new(IpcDiagNode::vfs_rpc())),
            // /proc/sched — scheduler diagnostics directory
            "sched" => Ok(Arc::new(SchedDirNode)),
            "sched/stat" => Ok(Arc::new(SchedStatNode)),
            // /proc/self — virtual directory for the calling process
            "self" => Ok(Arc::new(ProcSelfDirNode)),
            // /proc/self/exe — symlink to the current process's executable
            "self/exe" => Ok(Arc::new(ProcSelfExeNode)),
            // /proc/self/authority — canonical Authority for the calling process (Phase 7).
            //
            // Reports the active permission context of the calling task in
            // canonical Authority terms via `kernel::authority::bridge::authority_for_current`.
            // This is the convenient self-introspection path: callers do not
            // need to know their own PID.
            "self/authority" => Ok(Arc::new(ProcSelfAuthorityNode)),
            // /proc/self/place — canonical Place for the calling process (Phase 8).
            //
            // Reports the execution world-context (cwd, namespace, root) of the
            // calling task in canonical Place terms via
            // `kernel::place::bridge::place_for_current`.  This is the
            // convenient self-introspection path: callers do not need to know
            // their own PID.
            //
            // Note: terminal/UI/console attachment is not reported here. Use
            // `/proc/self/presence` for person-in-place semantics.  This path
            // answers "in what world?", not "who is present?".
            "self/place" => Ok(Arc::new(ProcSelfPlaceNode)),
            // /proc/self/presence — canonical Presence for the calling process.
            //
            // Reports controlling-terminal/session person-in-place semantics in
            // canonical Presence terms via `kernel::presence::bridge::presence_for_current`.
            "self/presence" => Ok(Arc::new(ProcSelfPresenceNode)),
            // /proc/self/inbox — VFS access to the calling process's typed inbox.
            "self/inbox" => {
                let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
                let inbox_id = pinfo.lock().unix_compat.message_inbox;
                let inbox = crate::inbox::get_inbox(inbox_id).ok_or(Errno::ENOENT)?;
                Ok(Arc::new(crate::vfs::inbox_node::InboxNode::new(inbox)))
            }
            // /proc/self/serviceloop — ServiceLoop watchdog metrics directory.
            //
            // Note: `/proc/self/serviceloop/` only exposes `stats` (write-only
            // endpoint for publishing metrics).  The readable metric files
            // (`last_enter_wait_ns`, etc.) are served under
            // `/proc/<pid>/serviceloop/` after the daemon has written its
            // first snapshot.
            "self/serviceloop" => Ok(Arc::new(ProcSelfServiceLoopDirNode)),
            // /proc/self/serviceloop/stats — write-only: publish metrics from userspace.
            "self/serviceloop/stats" => Ok(Arc::new(ProcSelfServiceLoopStatsNode)),
            _ => {
                // Try to match /proc/sched/cpu<N> paths.
                if let Some(rest) = path.strip_prefix("sched/cpu") {
                    if let Ok(cpu_id) = rest.parse::<usize>() {
                        return Ok(Arc::new(SchedCpuNode { cpu_id }));
                    }
                }

                // Try to match /proc/<pid>/... paths.
                // `path` is already relative to the mount point, so it looks
                // like "42/status", "42/cmdline", "42", etc.
                let mut parts = path.splitn(2, '/');
                let pid_str = parts.next().unwrap_or("");
                let rest = parts.next().unwrap_or("");

                if let Ok(pid) = pid_str.parse::<u32>() {
                    return lookup_pid(pid, rest);
                }
                Err(Errno::ENOENT)
            }
        }
    }
}

/// Look up a node inside a per-process `/proc/<pid>/` directory.
fn lookup_pid(pid: u32, rest: &str) -> SysResult<Arc<dyn VfsNode>> {
    // For paths under "task/..." we delegate immediately without needing the
    // process snapshot (which is only used by status/cmdline/exe).
    if rest == "task" || rest.starts_with("task/") {
        let tid_and_rest = rest.strip_prefix("task/").unwrap_or("");
        return lookup_pid_task(pid, tid_and_rest);
    }

    let snap = process_snapshot(pid).ok_or(Errno::ENOENT)?;

    match rest {
        // /proc/<pid> — the per-process directory itself
        "" => Ok(Arc::new(ProcPidDirNode { pid })),
        // /proc/<pid>/status — transitional: legacy process-group surface.
        //
        // Reports basic process state (name, state, pid, ppid) using
        // Unix-process vocabulary.  This path is maintained for compatibility
        // but must not be extended or improved.  New coordination state should
        // use `/proc/<pid>/group_kind` or `/proc/<pid>/foreground_group`.
        "status" => {
            let text = render_proc_status_text(&snap);
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 1)))
        }
        "cmdline" => {
            // Standard Linux /proc/<pid>/cmdline format: each argument is
            // followed by a NUL byte (including the last), so the full content
            // is "arg0\0arg1\0arg2\0".
            let mut data: Vec<u8> = Vec::new();
            for arg in snap.argv.iter() {
                data.extend_from_slice(arg);
                data.push(0);
            }
            Ok(Arc::new(DynamicTextNode::new(data, 300 + pid as u64 * 10 + 2)))
        }
        "fd" => Ok(Arc::new(ProcPidFdDirNode { pid })),
        "exe" => {
            // /proc/<pid>/exe — symlink to the process's executable path.
            Ok(Arc::new(ProcPidExeNode { exec_path: snap.exec_path.clone() }))
        }
        // /proc/<pid>/task_state — canonical thingos::task::TaskState label.
        //
        // Bridges the current kernel ThreadState into the schema-generated
        // TaskState via `kernel::task::bridge`.  This is the first public
        // surface for the Task ontology (Phase 1).
        "task_state" => {
            let task_state = crate::task::bridge::task_state_from_thread(snap.state);
            let text = alloc::format!("{}\n", task_state.as_str());
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 5)))
        }
        // /proc/<pid>/job_state — canonical thingos::job::JobState label.
        //
        // Bridges the current kernel Process/Thread lifecycle into the
        // schema-generated JobState via `kernel::job::bridge`.  This is the
        // first public surface for the Job ontology (Phase 2).
        "job_state" => {
            let job_state = crate::job::bridge::job_state_from_snapshot(&snap);
            let text = alloc::format!("{}\n", job_state.as_str());
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 6)))
        }
        // /proc/<pid>/job_exit — canonical thingos::job::JobExit (Phase 3).
        //
        // Reports the exit state and code in canonical Job terms, bridged from
        // the current Process/Thread model via `kernel::job::bridge`.
        // For live processes `code` is reported as `-`.
        "job_exit" => {
            let job_exit = crate::job::bridge::job_exit_from_snapshot(&snap);
            let text = job_exit.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 7)))
        }
        // /proc/<pid>/job_wait — canonical thingos::job::JobWaitResult (Phase 3).
        //
        // Non-blocking poll of the job's wait result.  Reinterprets the
        // current `poll_task_exit` output through the canonical Job vocabulary
        // via `kernel::job::bridge::job_wait_result_from_poll`.
        "job_wait" => {
            // poll_task_exit_current takes a TaskId (tid).  For the process
            // leader snap.tid == snap.pid as u64.
            let poll = unsafe { crate::sched::poll_task_exit_current(snap.tid) }
                .map_err(|_| Errno::ENOENT)?;
            let wait_result = crate::job::bridge::job_wait_result_from_poll(poll);
            let text = wait_result.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 8)))
        }
        // /proc/<pid>/group_kind — canonical thingos::group::GroupKind (Phase 4).
        //
        // Reports the coordination role of this process's group in canonical
        // Group terms, bridged from source-of-truth tty foreground pgid via
        // `kernel::group::bridge`.  This is the first public surface for the
        // Group ontology (Phase 4).
        "group_kind" => {
            let group = crate::group::bridge::group_from_snapshot(&snap);
            let text = group.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 9)))
        }
        // /proc/<pid>/foreground_group — canonical foreground membership (Phase 4).
        //
        // Reports whether this process belongs to the foreground coordination
        // group, in canonical Group terms via `kernel::group::bridge`.
        // Returns "true\n" when the process is in the foreground group and
        // "false\n" otherwise.
        //
        // This path avoids exposing pgid/sid/session concepts directly.
        // All logic is routed through `kernel::group::bridge::foreground_group_from_snapshot`.
        "foreground_group" => {
            let is_fg = crate::group::bridge::foreground_group_from_snapshot(&snap);
            let text = if is_fg { "true\n" } else { "false\n" };
            Ok(Arc::new(DynamicTextNode::new(text.as_bytes().to_vec(), 300 + pid as u64 * 10 + 12)))
        }
        // /proc/<pid>/authority — canonical thingos::authority::Authority (Phase 7).
        //
        // Reports the active permission context in canonical Authority terms,
        // bridged from the current Process-shaped credential state via
        // `kernel::authority::bridge`.  This is the first public surface for
        // the Authority ontology (Phase 7).
        //
        // In Phase 7 the authority name is derived from the process name and
        // capabilities is always empty (the current `Process` carries no
        // explicit capability mask).  Future phases will populate capabilities
        // once uid/gid-like fields or a capability mask are introduced into
        // the Process struct.
        "authority" => {
            let authority = crate::authority::bridge::authority_from_snapshot(&snap);
            let text = authority.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 10)))
        }
        // /proc/<pid>/place — canonical thingos::place::Place (Phase 8).
        //
        // Reports the execution world-context (cwd, namespace, root) in
        // canonical Place terms, bridged from the current Process-shaped
        // cwd/namespace state via `kernel::place::bridge`.  This is the first
        // public surface for the Place ontology (Phase 8).
        //
        // In Phase 8:
        // * `cwd` is derived from Process::cwd.
        // * `namespace` is derived from Process::namespace.
        // * `root` is derived from Process::root.
        //
        // Note: terminal/UI/console attachment is not reported here. Use
        // `/proc/<pid>/presence` for person-in-place semantics.  This path
        // answers "in what world?", not "who is present?".
        "place" => {
            let place = crate::place::bridge::place_from_snapshot(&snap);
            let text = place.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 11)))
        }
        // /proc/<pid>/presence — canonical thingos::presence::Presence.
        //
        // Reports controlling-terminal/session person-in-place state in
        // canonical Presence terms.
        "presence" => {
            let presence = crate::presence::bridge::presence_from_snapshot(&snap);
            let text = presence.as_text();
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), 300 + pid as u64 * 10 + 13)))
        }
        // /proc/<pid>/inbox — VFS access to the process's typed inbox.
        "inbox" => {
            let pinfo =
                crate::sched::process_info_for_tid_current(pid as u64).ok_or(Errno::ENOENT)?;
            let inbox_id = pinfo.lock().unix_compat.message_inbox;
            let inbox = crate::inbox::get_inbox(inbox_id).ok_or(Errno::ENOENT)?;
            Ok(Arc::new(crate::vfs::inbox_node::InboxNode::new(inbox)))
        }
        // /proc/<pid>/job_observer — register caller inbox for JobExit notifications.
        "job_observer" => Ok(Arc::new(ProcPidJobObserverNode { pid })),
        // /proc/<pid>/serviceloop — ServiceLoop diagnostic directory.
        "serviceloop" => Ok(Arc::new(ProcPidServiceLoopDirNode { pid })),
        // /proc/<pid>/serviceloop/<file> — individual ServiceLoop diagnostic files.
        //
        // Coarse state fields (name/state/last_event/…) come from ProcessInfo,
        // populated by SYS_SERVICE_LOOP_REPORT.  Precision timing fields
        // (last_enter_wait_ns/…) come from the SERVICE_LOOP_STATS registry,
        // populated by /proc/self/serviceloop/stats writes.
        "serviceloop/name"
        | "serviceloop/state"
        | "serviceloop/last_event"
        | "serviceloop/last_dispatch_ns"
        | "serviceloop/wakeups"
        | "serviceloop/timeouts"
        | "serviceloop/errors" => {
            let file = rest.strip_prefix("serviceloop/").unwrap_or(rest);
            let pinfo =
                crate::sched::process_info_for_tid_current(pid as u64).ok_or(Errno::ENOENT)?;
            let diag = pinfo.lock().service_loop.clone();
            let text = match file {
                "name" => {
                    let name = diag.as_ref().map(|d| d.name.as_str()).unwrap_or("-");
                    alloc::format!("{}\n", name)
                }
                "state" => {
                    let s = diag.as_ref().map(|d| d.state.as_str()).unwrap_or("idle");
                    alloc::format!("{}\n", s)
                }
                "last_event" => {
                    let ev = diag.as_ref().map(|d| d.last_event.as_str()).unwrap_or("-");
                    alloc::format!("{}\n", ev)
                }
                "last_dispatch_ns" => {
                    let ts = diag.as_ref().map(|d| d.last_dispatch_ns).unwrap_or(0);
                    alloc::format!("{}\n", ts)
                }
                "wakeups" => {
                    let n = diag.as_ref().map(|d| d.wakeups).unwrap_or(0);
                    alloc::format!("{}\n", n)
                }
                "timeouts" => {
                    let n = diag.as_ref().map(|d| d.timeouts).unwrap_or(0);
                    alloc::format!("{}\n", n)
                }
                "errors" => {
                    let n = diag.as_ref().map(|d| d.errors).unwrap_or(0);
                    alloc::format!("{}\n", n)
                }
                _ => return Err(Errno::ENOENT),
            };
            // Inode: top nibble 0xE, next 32 bits = pid, file discriminant in low bits.
            let file_idx: u64 = match file {
                "name" => 0,
                "state" => 1,
                "last_event" => 2,
                "last_dispatch_ns" => 3,
                "wakeups" => 4,
                "timeouts" => 5,
                "errors" => 6,
                _ => 7,
            };
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | file_idx;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        // /proc/<pid>/serviceloop/<precision-timing-file> — hot-path timing metrics.
        "serviceloop/last_enter_wait_ns" => {
            let text = get_service_loop_stats(pid)
                .map(|s| alloc::format!("{}\n", s.last_enter_wait_ns))
                .unwrap_or_else(|| alloc::string::String::from("0\n"));
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | 8;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "serviceloop/last_exit_wait_ns" => {
            let text = get_service_loop_stats(pid)
                .map(|s| alloc::format!("{}\n", s.last_exit_wait_ns))
                .unwrap_or_else(|| alloc::string::String::from("0\n"));
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | 9;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "serviceloop/last_dispatch_start_ns" => {
            let text = get_service_loop_stats(pid)
                .map(|s| alloc::format!("{}\n", s.last_dispatch_start_ns))
                .unwrap_or_else(|| alloc::string::String::from("0\n"));
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | 10;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "serviceloop/last_dispatch_end_ns" => {
            let text = get_service_loop_stats(pid)
                .map(|s| alloc::format!("{}\n", s.last_dispatch_end_ns))
                .unwrap_or_else(|| alloc::string::String::from("0\n"));
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | 11;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "serviceloop/current_event_kind" => {
            let text = get_service_loop_stats(pid)
                .map(|s| alloc::format!("{}\n", s.current_event_kind))
                .unwrap_or_else(|| alloc::string::String::from("0\n"));
            let ino = 0xE000_0000_0000_0000u64 | ((pid as u64) << 16) | 12;
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        _ => Err(Errno::ENOENT),
    }
}


/// `tid_and_rest` is everything after `"task/"`, e.g. `""` (the directory
/// itself), `"100"` (per-thread directory), or `"100/name"` (thread name).
fn lookup_pid_task(pid: u32, tid_and_rest: &str) -> SysResult<Arc<dyn VfsNode>> {
    // /proc/<pid>/task — directory listing all TIDs.
    if tid_and_rest.is_empty() {
        return Ok(Arc::new(ProcPidTaskDirNode { pid }));
    }

    let mut parts = tid_and_rest.splitn(2, '/');
    let tid_str = parts.next().unwrap_or("");
    let file = parts.next().unwrap_or("");

    let tid: u64 = tid_str.parse().map_err(|_| Errno::ENOENT)?;

    // Find the thread snapshot with the matching pid and tid.
    let procs = crate::sched::list_processes_current();
    let thread = procs.iter().find(|s| s.pid == pid && s.tid == tid).ok_or(Errno::ENOENT)?;

    match file {
        // /proc/<pid>/task/<tid> — per-thread directory.
        "" => Ok(Arc::new(ProcPidTaskTidDirNode { pid, tid })),
        "name" => {
            // Thread name, newline-terminated for compatibility with Linux.
            let mut text = thread.name.clone();
            text.push('\n');
            // Inode: top nibble 0xB, next 32 bits = pid, bottom 28 bits = tid.
            // This avoids collisions for the expected TID range (< 2^28).
            let ino = 0xB000_0000_0000_0000u64 | ((pid as u64) << 28) | (tid & 0x0FFF_FFFF);
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "task_state" => {
            // Canonical Task-shaped state derived via the bridge layer.
            //
            // Maps the kernel's internal ThreadState to the public thingos.task
            // vocabulary defined in tools/kindc/kinds/task.kind, using the
            // explicit bridge in crate::task::bridge rather than ad-hoc
            // conversion.  This is the first real code path that exposes the
            // new ontology at a system boundary.
            let task = crate::task::bridge::thread_state_to_task(thread.state);
            let state_name = match task.state {
                thingos::task::TaskState::New => "new",
                thingos::task::TaskState::Ready => "ready",
                thingos::task::TaskState::Running => "running",
                thingos::task::TaskState::Blocked => "blocked",
                thingos::task::TaskState::Exited => "exited",
            };
            let text = alloc::format!("state: {}\n", state_name);
            let ino = 0xC000_0000_0000_0000u64 | ((pid as u64) << 28) | (tid & 0x0FFF_FFFF);
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        "task" => {
            // Canonical enriched Task shape (Phase 5 — v2).
            //
            // Populates the full thingos.task v2 shape: state, job, and name.
            // Uses `kernel::task::bridge::task_from_snapshot` so that all three
            // canonical fields are derived in one place.
            let task = crate::task::bridge::task_from_snapshot(thread);
            let state_name = match task.state {
                thingos::task::TaskState::New => "new",
                thingos::task::TaskState::Ready => "ready",
                thingos::task::TaskState::Running => "running",
                thingos::task::TaskState::Blocked => "blocked",
                thingos::task::TaskState::Exited => "exited",
            };
            let job_str = match task.job {
                Some(j) => alloc::format!("{}", j),
                None => alloc::string::String::from("-"),
            };
            let name_str = match task.name.as_deref() {
                Some(n) => alloc::string::String::from(n),
                None => alloc::string::String::from("-"),
            };
            let text =
                alloc::format!("state: {}\njob: {}\nname: {}\n", state_name, job_str, name_str);
            let ino = 0xD000_0000_0000_0000u64 | ((pid as u64) << 28) | (tid & 0x0FFF_FFFF);
            Ok(Arc::new(DynamicTextNode::new(text.into_bytes(), ino)))
        }
        _ => Err(Errno::ENOENT),
    }
}

fn process_snapshot(pid: u32) -> Option<crate::sched::ProcessSnapshot> {
    let procs = crate::sched::list_processes_current();
    procs
        .iter()
        .find(|p| p.pid == pid && p.tid == pid as u64)
        .cloned()
        .or_else(|| procs.into_iter().find(|p| p.pid == pid))
}

fn process_ids() -> Vec<u32> {
    let mut seen = BTreeSet::new();
    let mut pids = Vec::new();
    for snap in crate::sched::list_processes_current() {
        if seen.insert(snap.pid) {
            pids.push(snap.pid);
        }
    }
    pids
}

/// Render `/proc/<pid>/status` using explicit projection boundaries.
///
/// Projection layering:
/// - Canonical generated kinds: build `thingos::task::Task` from the snapshot.
/// - Runtime bridge data: derive `thingos::job::JobState` from live thread-state
///   aggregation through `kernel::job::bridge`.
/// - Unix compatibility projection: map canonical task/job state into the
///   legacy single-letter status code (`R`, `S`, `Z`) and emit PID/PPID lines.
fn render_proc_status_text(snapshot: &crate::sched::ProcessSnapshot) -> String {
    // `task_from_snapshot` projects the leader thread's canonical task shape
    // (including TaskState/name), while `job_state_from_snapshot` aggregates
    // runtime thread-group liveness to classify whole-job exit.
    let task = crate::task::bridge::task_from_snapshot(snapshot);
    let job_state = crate::job::bridge::job_state_from_snapshot(snapshot);
    let state_name = unix_status_state_projection(task.state, job_state);
    let name = task.name.unwrap_or_else(|| snapshot.name.clone());
    alloc::format!(
        "Name:\t{}\nState:\t{}\nPid:\t{}\nPPid:\t{}\n",
        name,
        state_name,
        snapshot.pid,
        snapshot.ppid,
    )
}

fn unix_status_state_projection(
    task_state: thingos::task::TaskState,
    job_state: thingos::job::JobState,
) -> &'static str {
    if job_state == thingos::job::JobState::Exited {
        return "Z";
    }
    match task_state {
        thingos::task::TaskState::Blocked => "S",
        thingos::task::TaskState::Exited => "Z",
        thingos::task::TaskState::New
        | thingos::task::TaskState::Ready
        | thingos::task::TaskState::Running => "R",
    }
}

// ── /proc root directory ──────────────────────────────────────────────────────

struct ProcDirNode;

impl VfsNode for ProcDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 200, ..Default::default() })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut names = alloc::vec![
            String::from("version"),
            String::from("mounts"),
            String::from("meminfo"),
            String::from("cpuinfo"),
            String::from("uptime"),
            String::from("ipc"),
            String::from("sched"),
            String::from("self"),
        ];
        for pid in process_ids() {
            names.push(alloc::format!("{}", pid));
        }
        super::write_readdir_entries(names.iter().map(|s: &String| s.as_str()), offset, buf)
    }
}

// ── /proc/<pid>/ directory ────────────────────────────────────────────────────

struct ProcPidDirNode {
    pid: u32,
}

impl VfsNode for ProcPidDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            ino: 300 + self.pid as u64 * 10,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Legacy procfs entries (transitional internal model):
        //   status, cmdline, fd, exe, task
        // Canonical schema entries (Phase 1–8):
        //   task_state, job_state, job_exit, job_wait, group_kind, foreground_group, authority, place, presence
        let entries = [
            "status",
            "cmdline",
            "fd",
            "exe",
            "task",
            "task_state",
            "job_state",
            "job_exit",
            "job_wait",
            "group_kind",
            "foreground_group",
            "authority",
            "place",
            "presence",
            "inbox",
            "job_observer",
            "serviceloop",
        ];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/<pid>/job_observer — register JobExit observer ──────────────────────

/// Write-only registration node for job-exit observer wiring.
///
/// The caller's own inbox is attached as the observer target:
/// - allowed for self (`/proc/<self>/job_observer`)
/// - allowed for direct children
/// - rejected with `EPERM` otherwise.
struct ProcPidJobObserverNode {
    pid: u32,
}

const PROC_PID_BASE_INO: u64 = 300;
const PROC_PID_INO_STRIDE: u64 = 10;
const PROC_PID_JOB_OBSERVER_INO_OFFSET: u64 = 14;

impl VfsNode for ProcPidJobObserverNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EACCES)
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        let caller = crate::sched::process_info_current().ok_or(Errno::ESRCH)?;
        let (caller_pid, caller_inbox) = {
            let caller_lock = caller.lock();
            (caller_lock.pid, caller_lock.unix_compat.message_inbox)
        };

        let target =
            crate::sched::process_info_for_tid_current(self.pid as u64).ok_or(Errno::ESRCH)?;
        let allowed = {
            let target_lock = target.lock();
            caller_pid == self.pid || target_lock.job.ppid == caller_pid
        };
        if !allowed {
            return Err(Errno::EPERM);
        }

        if !crate::job::notify::register_exit_observer(self.pid, caller_inbox) {
            return Err(Errno::ESRCH);
        }
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o200,
            size: 0,
            ino: PROC_PID_BASE_INO
                + self.pid as u64 * PROC_PID_INO_STRIDE
                + PROC_PID_JOB_OBSERVER_INO_OFFSET,
            ..Default::default()
        })
    }
}

// ── /proc/<pid>/serviceloop/ directory ────────────────────────────────────────

/// Directory node for `/proc/<pid>/serviceloop/`.
///
/// Exposes diagnostic state reported via `SYS_SERVICE_LOOP_REPORT` and
/// precision timestamps from the `ServiceLoop` hot path.
struct ProcPidServiceLoopDirNode {
    pid: u32,
}

impl VfsNode for ProcPidServiceLoopDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            ino: 0xE000_0000_0000_0000u64 | ((self.pid as u64) << 16) | 0xFF,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = [
            "name",
            "state",
            "last_event",
            "last_dispatch_ns",
            "wakeups",
            "timeouts",
            "errors",
            "last_enter_wait_ns",
            "last_exit_wait_ns",
            "last_dispatch_start_ns",
            "last_dispatch_end_ns",
            "current_event_kind",
        ];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/self/serviceloop/ directory ────────────────────────────────────────

/// `/proc/self/serviceloop/` directory node.
struct ProcSelfServiceLoopDirNode;

impl VfsNode for ProcSelfServiceLoopDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o755,
            size: 0,
            ino: 0xF100_0000_0000_0000u64,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = ["stats"];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/self/serviceloop/stats — publish precision metrics from userspace ────

/// Write-only node for publishing ServiceLoop precision timing metrics from userspace.
///
/// The calling process writes a key-value text block (as produced by
/// `stem::service_loop::LoopMetricsSnapshot::write_text`) to this path.
/// The kernel parses it and stores the result in [`SERVICE_LOOP_STATS`],
/// making it visible via `/proc/<pid>/serviceloop/{last_enter_wait_ns,…}`.
///
/// Note: coarse loop state (`state`, `name`, `last_event`, etc.) is reported
/// through `SYS_SERVICE_LOOP_REPORT` which writes into `ProcessInfo::service_loop`.
/// Both mechanisms are read by `/proc/<pid>/serviceloop/`.
struct ProcSelfServiceLoopStatsNode;

impl VfsNode for ProcSelfServiceLoopStatsNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EACCES)
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        let pid = {
            let pinfo = crate::sched::process_info_current().ok_or(Errno::ESRCH)?;
            pinfo.lock().pid
        };
        let text = core::str::from_utf8(buf).map_err(|_| Errno::EINVAL)?;
        let stats = ServiceLoopStats::from_text(text).ok_or(Errno::EINVAL)?;
        publish_service_loop_stats(pid, stats);
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o200,
            size: 0,
            ino: 0xF200_0000_0000_0000u64,
            ..Default::default()
        })
    }
}

// ── /proc/<pid>/fd/ directory ─────────────────────────────────────────────────


struct ProcPidFdDirNode {
    #[allow(dead_code)]
    pid: u32,
}

impl VfsNode for ProcPidFdDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            ino: 300 + self.pid as u64 * 10 + 3,
            ..Default::default()
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Stub: empty directory.
        let _ = buf;
        Ok(0)
    }
}

// ── /proc/<pid>/task/ directory ───────────────────────────────────────────────

struct ProcPidTaskDirNode {
    pid: u32,
}

impl VfsNode for ProcPidTaskDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            // Inode: top nibble 0x9, bottom 32 bits = pid.
            ino: 0x9000_0000_0000_0000u64 | self.pid as u64,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let procs = crate::sched::list_processes_current();
        let tids: Vec<String> = procs
            .iter()
            .filter(|s| s.pid == self.pid)
            .map(|s| alloc::format!("{}", s.tid))
            .collect();
        super::write_readdir_entries(tids.iter().map(|s: &String| s.as_str()), offset, buf)
    }
}

// ── /proc/<pid>/task/<tid>/ directory ─────────────────────────────────────────

struct ProcPidTaskTidDirNode {
    #[allow(dead_code)]
    pid: u32,
    #[allow(dead_code)]
    tid: u64,
}

impl VfsNode for ProcPidTaskTidDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            // Inode: top nibble 0xA, next 32 bits = pid, bottom 28 bits = tid.
            ino: 0xA000_0000_0000_0000u64 | ((self.pid as u64) << 28) | (self.tid & 0x0FFF_FFFF),
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = ["name", "task_state", "task"];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/self — virtual directory for the calling process ───────────────────

struct ProcSelfDirNode;

impl VfsNode for ProcSelfDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 210, ..Default::default() })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = ["exe", "authority", "place", "presence", "inbox", "serviceloop"];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/self/exe — symlink to the current process's executable ──────────────

/// A symlink node that resolves to the calling process's executable path.
///
/// Reading returns the path; `readlink` returns it directly for VFS consumers.
struct ProcSelfExeNode;

impl VfsNode for ProcSelfExeNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let target = self.readlink()?;
        let data = target.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFLNK | 0o777, size: 0, ino: 211, ..Default::default() })
    }
    fn readlink(&self) -> SysResult<String> {
        let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let path = pinfo.lock().exec_path.clone();
        if path.is_empty() { Err(Errno::ENOENT) } else { Ok(path) }
    }
}

// ── /proc/self/authority — Authority for the calling process ─────────────────

/// A read-only node that reports the calling task's canonical Authority.
///
/// Reads the authority by calling [`crate::authority::bridge::authority_for_current`]
/// at read time; the result reflects the permission context of the task that
/// opened the file, formatted via [`thingos::authority::Authority::as_text`].
struct ProcSelfAuthorityNode;

impl VfsNode for ProcSelfAuthorityNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let authority = crate::authority::bridge::authority_for_current();
        let text = authority.as_text();
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 212, ..Default::default() })
    }
}

// ── /proc/self/place — Place for the calling process ─────────────────────────

/// A read-only node that reports the calling task's canonical Place.
///
/// Reads the place by calling [`crate::place::bridge::place_for_current`]
/// at read time; the result reflects the world-context of the task that
/// opened the file, formatted via [`thingos::place::Place::as_text`].
struct ProcSelfPlaceNode;

impl VfsNode for ProcSelfPlaceNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let place = crate::place::bridge::place_for_current();
        let text = place.as_text();
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 213, ..Default::default() })
    }
}

// ── /proc/self/presence — Presence for the calling process ────────────────────

/// A read-only node that reports the calling task's canonical Presence.
struct ProcSelfPresenceNode;
const PROC_SELF_PRESENCE_INO: u64 = 214;

impl VfsNode for ProcSelfPresenceNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let presence = crate::presence::bridge::presence_for_current();
        let text = presence.as_text();
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0,
            ino: PROC_SELF_PRESENCE_INO,
            ..Default::default()
        })
    }
}

// ── /proc/<pid>/exe — symlink to a specific process's executable ──────────────

/// A symlink node that resolves to a given process's executable path.
struct ProcPidExeNode {
    exec_path: String,
}

impl VfsNode for ProcPidExeNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let target = self.readlink()?;
        let data = target.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFLNK | 0o777,
            size: 0,
            ino: 0, // dynamic; caller doesn't rely on stable ino for exe nodes
            ..Default::default()
        })
    }
    fn readlink(&self) -> SysResult<String> {
        if self.exec_path.is_empty() { Err(Errno::ENOENT) } else { Ok(self.exec_path.clone()) }
    }
}

// ── Static text node ──────────────────────────────────────────────────────────

/// Returns a fixed byte slice on read.
struct StaticTextNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticTextNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticTextNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.data.len() {
            return Ok(0);
        }
        let avail = &self.data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
            ..Default::default()
        })
    }
}

// ── /proc/mounts ─────────────────────────────────────────────────────────────

/// A dynamic node that renders the current mount table as text.
///
/// The output mirrors a simplified `/proc/mounts` format:
/// ```text
/// <mount_point> ramfs rw 0 0
/// ```
struct MountsNode;

impl VfsNode for MountsNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Build the mounts listing dynamically from the global mount table.
        let listing = super::mount::mounts_text();
        let data = listing.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let avail = &data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0, // dynamic — size not known until read
            ino: 201,
            ..Default::default()
        })
    }
}

// ── Dynamic text node ─────────────────────────────────────────────────────────

/// Returns an owned byte vector on read.  Used for dynamically-generated
/// per-process text nodes such as `/proc/<pid>/status`.
struct DynamicTextNode {
    data: Vec<u8>,
    ino: u64,
}

impl DynamicTextNode {
    fn new(data: Vec<u8>, ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for DynamicTextNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.data.len() {
            return Ok(0);
        }
        let avail = &self.data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
            ..Default::default()
        })
    }
}

// ── /proc/meminfo ─────────────────────────────────────────────────────────────

/// Reports kernel heap statistics in a simplified `/proc/meminfo` format.
struct MemInfoNode;

impl VfsNode for MemInfoNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // We report the static heap reservation size; detailed used/free
        // accounting is not yet tracked in the global allocator.
        let total_kb = (crate::memory::layout::KHEAP_SIZE / 1024) as u64;
        let text = alloc::format!("MemTotal:    {:8} kB\nMemFree:     {:8} kB\n", total_kb, 0u64,);
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 202, ..Default::default() })
    }
}

// ── /proc/cpuinfo ─────────────────────────────────────────────────────────────

/// Reports a minimal CPU description.
struct CpuInfoNode;

impl VfsNode for CpuInfoNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let text = b"model name\t: Thing-OS virtual CPU\nprocessor\t: 0\n";
        let off = offset as usize;
        if off >= text.len() {
            return Ok(0);
        }
        let n = (text.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&text[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 203, ..Default::default() })
    }
}

// ── /proc/uptime ──────────────────────────────────────────────────────────────

/// Returns seconds since boot as a decimal string.
struct UptimeNode;

impl VfsNode for UptimeNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let secs = uptime_secs();
        let text = alloc::format!("{}.00 {}.00\n", secs, secs);
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 204, ..Default::default() })
    }
}

/// Return the number of seconds elapsed since boot using the runtime's
/// monotonic clock.  Returns 0 in test environments where the runtime hook
/// is not installed.
fn uptime_secs() -> u64 {
    // `runtime_base()` panics in test builds if the hook is not set up;
    // guard with a cfg flag so unit tests still pass.
    #[cfg(not(test))]
    {
        let rt = crate::runtime_base();
        let ticks = rt.mono_ticks();
        let freq = rt.mono_freq_hz();
        if freq == 0 {
            return 0;
        }
        ticks / freq
    }
    #[cfg(test)]
    {
        0
    }
}

// ── /proc/ipc/ directory ──────────────────────────────────────────────────────

/// Directory node for `/proc/ipc`.
struct IpcDirNode;

impl VfsNode for IpcDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 500, ..Default::default() })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = ["ports", "pipes", "vfs_rpc"];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

/// Dynamic text node that renders IPC diagnostic counters on demand.
struct IpcDiagNode {
    kind: IpcDiagKind,
    ino: u64,
}

enum IpcDiagKind {
    Ports,
    Pipes,
    VfsRpc,
}

impl IpcDiagNode {
    fn ports() -> Self {
        Self { kind: IpcDiagKind::Ports, ino: 501 }
    }
    fn pipes() -> Self {
        Self { kind: IpcDiagKind::Pipes, ino: 502 }
    }
    fn vfs_rpc() -> Self {
        Self { kind: IpcDiagKind::VfsRpc, ino: 503 }
    }

    fn render(&self) -> alloc::string::String {
        match self.kind {
            IpcDiagKind::Ports => crate::ipc::diag::ports_text(),
            IpcDiagKind::Pipes => crate::ipc::diag::pipes_text(),
            IpcDiagKind::VfsRpc => crate::ipc::diag::vfs_rpc_text(),
        }
    }
}

impl VfsNode for IpcDiagNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let text = self.render();
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0, // dynamic
            ino: self.ino,
            ..Default::default()
        })
    }
}

// ── /proc/sched/ — scheduler diagnostics ─────────────────────────────────────
//
// | Path                 | Contents                                        |
// |----------------------|-------------------------------------------------|
// | `/proc/sched`        | Directory listing                               |
// | `/proc/sched/stat`   | One block per online CPU with all counters      |
// | `/proc/sched/cpu<N>` | Single-CPU counters for CPU N                   |

/// Directory node for `/proc/sched`.
struct SchedDirNode;

impl VfsNode for SchedDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 600, ..Default::default() })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let diag = crate::sched::collect_sched_diag_current();
        let mut names: Vec<String> = Vec::with_capacity(diag.per_cpu.len() + 1);
        names.push(String::from("stat"));
        for cpu in &diag.per_cpu {
            names.push(alloc::format!("cpu{}", cpu.cpu_id));
        }
        super::write_readdir_entries(names.iter().map(|s: &String| s.as_str()), offset, buf)
    }
}

/// Render a single [`CpuSchedDiag`] into a key: value text block.
fn render_cpu_sched_diag(cpu: &crate::sched::CpuSchedDiag) -> String {
    alloc::format!(
        "cpu: {}\nrunnable_count: {}\ncontext_switches: {}\nwakeups: {}\nsteals_in: {}\nsteals_out: {}\ntimer_interrupts: {}\nidle_total_us: {}\nidle_episodes: {}\ndispatch_count: {}\nresched_ipi_received: {}\nmailbox_pushes: {}\nmailbox_tasks_drained: {}\n",
        cpu.cpu_id,
        cpu.runnable_count,
        cpu.context_switches,
        cpu.wakeups,
        cpu.steals_in,
        cpu.steals_out,
        cpu.timer_interrupts,
        cpu.idle_total_us,
        cpu.idle_episodes,
        cpu.dispatch_count,
        cpu.resched_ipi_received,
        cpu.mailbox_pushes,
        cpu.mailbox_tasks_drained,
    )
}

/// `/proc/sched/stat` — aggregate per-CPU scheduler metrics.
///
/// Emits one block per online CPU, each containing all tracked counters.
/// Blocks are separated by a blank line for easy `grep`/`awk` consumption.
struct SchedStatNode;

impl VfsNode for SchedStatNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let diag = crate::sched::collect_sched_diag_current();
        let mut text = alloc::format!("online_cpus: {}\n\n", diag.online_cpu_count);
        for cpu in &diag.per_cpu {
            text.push_str(&render_cpu_sched_diag(cpu));
            text.push('\n');
        }
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino: 601, ..Default::default() })
    }
}

/// `/proc/sched/cpu<N>` — per-CPU scheduler metrics for a single CPU.
struct SchedCpuNode {
    cpu_id: usize,
}

impl VfsNode for SchedCpuNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let diag = crate::sched::collect_sched_diag_current();
        // Try direct index first (cpu_id usually equals vector position for
        // contiguous online CPUs), then fall back to a linear search.
        let cpu = diag
            .per_cpu
            .get(self.cpu_id)
            .filter(|c| c.cpu_id == self.cpu_id)
            .or_else(|| diag.per_cpu.iter().find(|c| c.cpu_id == self.cpu_id))
            .ok_or(Errno::ENOENT)?;
        let text = render_cpu_sched_diag(cpu);
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        // Inode: top nibble 0x6, bottom 16 bits = cpu_id.
        let ino = 0x6000_0000_0000_0000u64 | (self.cpu_id as u64 & 0xFFFF);
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o444, size: 0, ino, ..Default::default() })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot_with_states(
        state: crate::task::TaskState,
        thread_states: Vec<crate::task::TaskState>,
    ) -> crate::sched::ProcessSnapshot {
        crate::sched::ProcessSnapshot {
            pid: 42,
            ppid: 7,
            tid: 42,
            name: String::from("demo"),
            state,
            argv: Vec::new(),
            exec_path: String::from("/bin/demo"),
            uid: 0,
            gid: 0,
            capability_mask: 0,
            exit_code: None,
            pgid: 42,
            sid: 42,
            session_leader: true,
            foreground_pgid: Some(42),
            cwd: String::from("/"),
            namespace_label: String::from("global"),
            root_path: String::from("/"),
            thread_states,
            space_id: thingos::space::SpaceId::NONE,
            space_mapping_count: 0,
            space_sharing_count: 1,
        }
    }

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        ProcFs::new().lookup(path)
    }

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_lookup_version() {
        let node = lookup("version").unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        assert!(core::str::from_utf8(&buf[..n]).unwrap().contains("Thing-OS"));
    }

    #[test]
    fn test_version_is_readonly() {
        let node = lookup("version").unwrap();
        assert!(matches!(node.write(0, b"hack"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("doesnotexist"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_readdir_root_lists_entries() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("version"));
        assert!(s.contains("mounts"));
        assert!(s.contains("meminfo"));
        assert!(s.contains("cpuinfo"));
        assert!(s.contains("uptime"));
        assert!(s.contains("ipc"));
    }

    #[test]
    fn test_lookup_meminfo() {
        let node = lookup("meminfo").unwrap();
        let mut buf = [0u8; 128];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("MemTotal"));
    }

    #[test]
    fn test_lookup_cpuinfo() {
        let node = lookup("cpuinfo").unwrap();
        let mut buf = [0u8; 128];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("processor"));
    }

    #[test]
    fn test_lookup_uptime() {
        let node = lookup("uptime").unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
    }

    #[test]
    fn test_lookup_pid_enoent_when_no_processes() {
        // In test environment there are no real processes so any PID should
        // return ENOENT.
        assert!(matches!(lookup("1/status"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_readdir_root_lists_ipc() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("ipc"));
    }

    #[test]
    fn test_lookup_ipc_dir_is_dir() {
        let node = lookup("ipc").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_lookup_ipc_ports() {
        let node = lookup("ipc/ports").unwrap();
        let mut buf = [0u8; 256];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("sends:"));
    }

    #[test]
    fn test_lookup_ipc_pipes() {
        let node = lookup("ipc/pipes").unwrap();
        let mut buf = [0u8; 256];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("writes:"));
    }

    #[test]
    fn test_lookup_ipc_vfs_rpc() {
        let node = lookup("ipc/vfs_rpc").unwrap();
        let mut buf = [0u8; 256];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("requests:"));
    }

    #[test]
    fn test_ipc_diag_nodes_are_readonly() {
        for path in &["ipc/ports", "ipc/pipes", "ipc/vfs_rpc"] {
            let node = lookup(path).unwrap();
            assert!(matches!(node.write(0, b"x"), Err(Errno::EROFS)));
        }
    }

    // ── /proc/self/authority ──────────────────────────────────────────────────

    #[test]
    fn test_lookup_self_authority_succeeds() {
        // /proc/self/authority is always resolvable — it uses authority_for_current()
        // which falls back to "kernel" when no process context exists.
        assert!(lookup("self/authority").is_ok());
    }

    #[test]
    fn test_self_authority_is_readable() {
        let node = lookup("self/authority").unwrap();
        let mut buf = [0u8; 256];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("name:"), "authority text must contain 'name:': {s}");
        assert!(s.contains("capabilities:"), "authority text must contain 'capabilities:': {s}");
    }

    #[test]
    fn test_self_authority_is_readonly() {
        let node = lookup("self/authority").unwrap();
        assert!(matches!(node.write(0, b"x"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_self_authority_is_regular_file() {
        let node = lookup("self/authority").unwrap();
        let stat = node.stat().unwrap();
        assert!(!stat.is_dir());
        // Readable by all; not writable.
        assert_eq!(stat.mode & 0o777, 0o444);
    }

    #[test]
    fn test_self_dir_readdir_includes_authority() {
        let node = lookup("self").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("authority"), "self readdir must list 'authority': {s}");
    }

    #[test]
    fn test_lookup_self_presence_succeeds() {
        assert!(lookup("self/presence").is_ok());
    }

    #[test]
    fn test_self_presence_is_readable() {
        let node = lookup("self/presence").unwrap();
        let mut buf = [0u8; 256];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("subject:"), "presence text must contain 'subject:': {s}");
        assert!(s.contains("mode:"), "presence text must contain 'mode:': {s}");
    }

    #[test]
    fn test_self_presence_is_readonly() {
        let node = lookup("self/presence").unwrap();
        assert!(matches!(node.write(0, b"x"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_self_dir_readdir_includes_presence() {
        let node = lookup("self").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("presence"), "self readdir must list 'presence': {s}");
    }

    #[test]
    fn test_unix_status_state_projection_matches_legacy_letters() {
        assert_eq!(
            unix_status_state_projection(
                thingos::task::TaskState::Running,
                thingos::job::JobState::Running
            ),
            "R"
        );
        assert_eq!(
            unix_status_state_projection(
                thingos::task::TaskState::Ready,
                thingos::job::JobState::Running
            ),
            "R"
        );
        assert_eq!(
            unix_status_state_projection(
                thingos::task::TaskState::Blocked,
                thingos::job::JobState::Running
            ),
            "S"
        );
        assert_eq!(
            unix_status_state_projection(
                thingos::task::TaskState::Exited,
                thingos::job::JobState::Exited
            ),
            "Z"
        );
    }

    #[test]
    fn test_render_proc_status_uses_job_bridge_for_exited_state() {
        let snapshot = snapshot_with_states(
            crate::task::TaskState::Running,
            vec![crate::task::TaskState::Dead],
        );
        let text = render_proc_status_text(&snapshot);
        assert!(text.contains("Name:\tdemo\n"), "unexpected text: {text}");
        assert!(text.contains("State:\tZ\n"), "unexpected text: {text}");
        assert!(text.contains("Pid:\t42\n"), "unexpected text: {text}");
        assert!(text.contains("PPid:\t7\n"), "unexpected text: {text}");
    }

    #[test]
    fn test_render_proc_status_handles_direct_exited_task_projection() {
        let snapshot = snapshot_with_states(crate::task::TaskState::Dead, Vec::new());
        let text = render_proc_status_text(&snapshot);
        assert!(text.contains("State:\tZ\n"), "unexpected text: {text}");
    }

    // ── /proc/sched ───────────────────────────────────────────────────────────

    #[test]
    fn test_lookup_sched_dir_succeeds() {
        let node = lookup("sched").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir(), "sched should be a directory");
        assert_eq!(stat.mode & 0o777, 0o555);
    }

    #[test]
    fn test_sched_dir_write_returns_eisdir() {
        let node = lookup("sched").unwrap();
        assert!(matches!(node.write(0, b"x"), Err(Errno::EISDIR)));
    }

    #[test]
    fn test_lookup_sched_stat_succeeds() {
        let node = lookup("sched/stat").unwrap();
        let stat = node.stat().unwrap();
        assert!(!stat.is_dir(), "sched/stat should be a regular file");
        assert_eq!(stat.mode & 0o777, 0o444);
    }

    #[test]
    fn test_sched_stat_is_readable() {
        let node = lookup("sched/stat").unwrap();
        let mut buf = [0u8; 512];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0, "sched/stat should produce output");
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("online_cpus:"), "sched/stat must contain 'online_cpus:': {s}");
    }

    #[test]
    fn test_sched_stat_is_readonly() {
        let node = lookup("sched/stat").unwrap();
        assert!(matches!(node.write(0, b"x"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_sched_dir_readdir_includes_stat() {
        let node = lookup("sched").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("stat"), "sched readdir must list 'stat': {s}");
    }

    #[test]
    fn test_root_readdir_includes_sched() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 512];
        let n = node.readdir(0, &mut buf).unwrap();
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("sched"), "root readdir must list 'sched': {s}");
    }

    #[test]
    fn test_sched_cpu_invalid_returns_enoent() {
        // CPU 9999 is never online so should return ENOENT.
        let node = lookup("sched/cpu9999").unwrap();
        let mut buf = [0u8; 256];
        assert!(matches!(node.read(0, &mut buf), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_sched_cpu_node_is_readonly() {
        let node = lookup("sched/cpu9999").unwrap();
        assert!(matches!(node.write(0, b"x"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_sched_unknown_subpath_returns_enoent() {
        assert!(matches!(lookup("sched/bogus"), Err(Errno::ENOENT)));
    }

    // ── ServiceLoop stats registry tests ─────────────────────────────────────

    #[test]
    fn test_service_loop_stats_from_text_round_trip() {
        let text = "last_enter_wait_ns: 100\nlast_exit_wait_ns: 200\nlast_dispatch_start_ns: 250\nlast_dispatch_end_ns: 300\ncurrent_event_kind: 0\n";
        let s = ServiceLoopStats::from_text(text).expect("should parse");
        assert_eq!(s.last_enter_wait_ns, 100);
        assert_eq!(s.last_exit_wait_ns, 200);
        assert_eq!(s.last_dispatch_start_ns, 250);
        assert_eq!(s.last_dispatch_end_ns, 300);
        assert_eq!(s.current_event_kind, 0);
    }

    #[test]
    fn test_service_loop_stats_from_text_missing_field_returns_none() {
        // Only 4 of the 5 required fields.
        let text = "last_enter_wait_ns: 1\nlast_exit_wait_ns: 2\nlast_dispatch_start_ns: 3\nlast_dispatch_end_ns: 4\n";
        assert!(ServiceLoopStats::from_text(text).is_none());
    }

    #[test]
    fn test_service_loop_stats_as_text_parseable() {
        let s = ServiceLoopStats {
            last_enter_wait_ns: 10,
            last_exit_wait_ns: 20,
            last_dispatch_start_ns: 30,
            last_dispatch_end_ns: 40,
            current_event_kind: 5,
        };
        let text = s.as_text();
        let parsed = ServiceLoopStats::from_text(&text).expect("as_text output must be parseable");
        assert_eq!(parsed.last_enter_wait_ns, 10);
        assert_eq!(parsed.last_dispatch_end_ns, 40);
        assert_eq!(parsed.current_event_kind, 5);
    }

    #[test]
    fn test_publish_and_get_service_loop_stats() {
        let stats = ServiceLoopStats {
            last_enter_wait_ns: 999,
            last_exit_wait_ns: 1000,
            last_dispatch_start_ns: 1001,
            last_dispatch_end_ns: 1002,
            current_event_kind: 42,
        };
        publish_service_loop_stats(77777, stats.clone());
        let retrieved = get_service_loop_stats(77777).expect("should retrieve published stats");
        assert_eq!(retrieved.last_enter_wait_ns, 999);
        assert_eq!(retrieved.current_event_kind, 42);
        // Cleanup.
        remove_service_loop_stats(77777);
        assert!(get_service_loop_stats(77777).is_none());
    }

    #[test]
    fn test_get_service_loop_stats_missing_returns_none() {
        // Use a PID unlikely to be registered.
        assert!(get_service_loop_stats(99998).is_none());
    }
}

//! Bridge layer: kernel coordination state → canonical `thingos::group::Group`.
//!
//! # Purpose
//!
//! This module is the **single conversion point** from the kernel's Unix-shaped
//! coordination structures (`Process::pgid`, `Process::sid`,
//! `ConsoleTtyState::foreground_pgid`) to the
//! schema-generated canonical `Group` and `GroupKind` types.
//!
//! All group-coordination-facing public paths (procfs, future session/TTY
//! boundary) should go through here rather than reading internal fields
//! directly.
//!
//! # Transitional mapping
//!
//! | Kernel source                               | Canonical `GroupKind`     |
//! |---------------------------------------------|---------------------------|
//! | `foreground_pgid == Some(process.pgid)`      | `Foreground`              |
//! | otherwise                                   | `Coordination`            |
//!
//! # What Process/Unix concepts are not (yet) replaced
//!
//! * Signal broadcast domains (`pgid` → `send_signal_to_group`)
//! * `TIOCSPGRP` / `TIOCGPGRP` ioctl group control
//! * `setsid` / session creation
//! * `SIGCONT`, `SIGSTOP`, `SIGHUP` routing
//!
//! These remain tied to Unix-shaped internals.  They become `Group` in
//! Phase 5 and beyond.
//!
//! # Future direction
//!
//! Signal routing is still Unix-shaped today (`pgid`-based broadcasts and tty
//! job-control wiring), but foreground-group determination now follows the
//! source-of-truth tty foreground pgid.

use thingos::group::{Group, GroupKind};

/// Derive the canonical `GroupKind` from a process snapshot.
///
/// Foreground membership is derived from source-of-truth TTY foreground
/// ownership: the process is foreground iff its `pgid` matches the current
/// console `foreground_pgid`.
pub fn group_kind_from_snapshot(
    snapshot: &crate::sched::hooks::ProcessSnapshot,
) -> GroupKind {
    if snapshot.foreground_pgid == Some(snapshot.pgid) {
        GroupKind::Foreground
    } else {
        GroupKind::Coordination
    }
}

/// Build a canonical `Group` from a process snapshot.
///
/// Convenience wrapper around [`group_kind_from_snapshot`].
pub fn group_from_snapshot(snapshot: &crate::sched::hooks::ProcessSnapshot) -> Group {
    Group { kind: group_kind_from_snapshot(snapshot) }
}

/// Return `true` when this process is in the foreground coordination group.
///
/// Derives foreground status entirely from [`group_kind_from_snapshot`] so
/// that all Group-bridge logic remains in one place.  Callers must not
/// inspect `pgid`, `sid`, or `foreground_pgid` directly.
///
/// # Usage
///
/// This function backs the `/proc/<pid>/foreground_group` procfs path.
/// It is the canonical Group-vocabulary answer to "is this process in the
/// foreground group?" and replaces any direct TTY/pgid inspection for new
/// introspection surfaces.
pub fn foreground_group_from_snapshot(
    snapshot: &crate::sched::hooks::ProcessSnapshot,
) -> bool {
    group_kind_from_snapshot(snapshot) == GroupKind::Foreground
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::hooks::ProcessSnapshot;
    use crate::task::TaskState;

    fn make_snapshot(
        pgid: u32,
        foreground_pgid: Option<u32>,
        session_leader: bool,
    ) -> ProcessSnapshot {
        ProcessSnapshot {
            pid: 1,
            ppid: 0,
            tid: 1,
            name: alloc::string::String::from("test"),
            state: TaskState::Runnable,
            argv: alloc::vec::Vec::new(),
            exec_path: alloc::string::String::new(),
            uid: 0,
            gid: 0,
            capability_mask: 0,
            exit_code: None,
            pgid,
            sid: 1,
            session_leader,
            foreground_pgid,
            cwd: alloc::string::String::from("/"),
            namespace_label: alloc::string::String::from("global"),
            root_path: alloc::string::String::from("/"),
            thread_states: alloc::vec![TaskState::Runnable],
            space_id: thingos::space::SpaceId::NONE,
            space_mapping_count: 0,
            space_sharing_count: 0,
        }
    }

    // ── group_kind_from_snapshot ─────────────────────────────────────────────

    #[test]
    fn test_matching_foreground_pgid_is_foreground() {
        let snap = make_snapshot(7, Some(7), false);
        assert_eq!(group_kind_from_snapshot(&snap), GroupKind::Foreground);
    }

    #[test]
    fn test_mismatched_foreground_pgid_is_coordination() {
        let snap = make_snapshot(7, Some(9), true);
        assert_eq!(group_kind_from_snapshot(&snap), GroupKind::Coordination);
    }

    // ── foreground_group_from_snapshot ──────────────────────────────────────

    #[test]
    fn test_foreground_group_from_snapshot_matching_pgid() {
        let snap = make_snapshot(3, Some(3), false);
        assert!(foreground_group_from_snapshot(&snap));
    }

    #[test]
    fn test_foreground_group_from_snapshot_mismatched_pgid() {
        let snap = make_snapshot(3, Some(4), true);
        assert!(!foreground_group_from_snapshot(&snap));
    }

    // ── group_from_snapshot ──────────────────────────────────────────────────

    #[test]
    fn test_group_from_snapshot_matching_pgid() {
        let snap = make_snapshot(11, Some(11), false);
        let group = group_from_snapshot(&snap);
        assert_eq!(group.kind, GroupKind::Foreground);
    }

    #[test]
    fn test_group_from_snapshot_mismatched_pgid() {
        let snap = make_snapshot(11, Some(12), true);
        let group = group_from_snapshot(&snap);
        assert_eq!(group.kind, GroupKind::Coordination);
    }

    #[test]
    fn test_group_from_snapshot_as_text_foreground() {
        let snap = make_snapshot(1, Some(1), false);
        let group = group_from_snapshot(&snap);
        let text = group.as_text();
        assert!(text.contains("kind: Foreground"), "unexpected: {}", text);
    }

    #[test]
    fn test_group_from_snapshot_as_text_coordination() {
        let snap = make_snapshot(1, None, true);
        let group = group_from_snapshot(&snap);
        let text = group.as_text();
        assert!(text.contains("kind: Coordination"), "unexpected: {}", text);
    }
}

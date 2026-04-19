//! Bridge layer: kernel `ProcessSnapshot` → canonical `thingos::authority::Authority`.
//!
//! # Purpose
//!
//! This module is the **single conversion point** from the kernel's
//! transitional `Process`-shaped permission context to the schema-generated
//! canonical `Authority` representation.  All authority-facing public paths
//! (procfs `/proc/<pid>/authority`, future introspection syscalls) should go
//! through here rather than reading internal fields directly.
//!
//! # Migration inventory — Process credential/permission responsibilities
//!
//! The table below inventories every field in the current `Process` struct that
//! answers authorization questions, and maps it to its intended future
//! `Authority` field.  Fields marked **provisional** remain in `Process`
//! pending fuller extraction.
//!
//! | `Process` / `ProcessSnapshot` field | Authorization role               | Intended `Authority` mapping          | Status       |
//! |--------------------------------------|----------------------------------|---------------------------------------|--------------|
//! | `name` (thread/process name)         | Human-readable identity label    | `Authority::name`                     | **Bridged**  |
//! | `exec_path`                          | Executable identity              | `Authority::name` (fallback if empty) | Provisional  |
//! | `pid` / `ppid`                       | Process identity                 | Future principal identifier           | Provisional  |
//! | `pgid` / `sid`                       | Coordination group identity      | `Group` domain (Phase 4)              | Provisional  |
//! | `session_leader`                     | TTY foreground ownership         | `Group::kind` (Phase 4)               | Provisional  |
//! | `authority.uid` / `authority.gid`    | Principal identity               | `Authority::{uid,gid}`                | **Bridged**  |
//! | `authority.capability_mask`          | Fine-grained privilege bits      | `Authority::capability_mask`          | **Bridged**  |
//! | `handle_table` (open files)              | Resource access rights           | Out of `Authority` scope (Phase 8+)   | Provisional  |
//! | `namespace`                          | VFS visibility                   | Future `Place` context (Phase 8)      | Provisional  |
//! | `signals` (signal dispositions)      | Signal delivery permissions      | Future authority concern (Phase 9+)   | Provisional  |
//!
//! # Transitional mapping
//!
//! | `ProcessSnapshot` field | `Authority` field     | Notes                                      |
//! |-------------------------|-----------------------|--------------------------------------------|
//! | `authority.uid`         | `uid`                 | Principal user id                              |
//! | `authority.gid`         | `gid`                 | Principal group id                             |
//! | `authority.capability_mask` | `capability_mask` | Capability bits used for enforcement           |
//! | `name`/`exec_path`      | `name`                | Human-readable label only (non-authoritative)  |
//!
//! # What is not yet replaced
//!
//! * uid/gid-like identity — no such field in `Process` yet
//! * capability masks — no such field in `Process` yet
//! * service-account / principal binding — not yet introduced
//! * signal permissions — remain in `Process::signals` (provisional)
//! * namespace / VFS visibility — will become `Place` context in Phase 8
//!
//! # Future direction
//!
//! When a uid/gid field or capability mask is added to `Process`, this bridge
//! will be the **only** place that needs updating to surface those fields
//! through the canonical `Authority` type.  New access-control code must not
//! read those fields from `Process` directly; all public paths must go through
//! this bridge.
//!
//! # Guardrail: new authorization code entry points
//!
//! **New authorization checks must use the helpers in this module**, not raw
//! `Process` fields.  The canonical call sequence for a privileged syscall is:
//!
//! ```text
//! let authority = authority_for_current();
//! check_privilege(&authority, "reboot")?;
//! ```
//!
//! This keeps all authorization decision points visible and in one place,
//! making future migration to a fully-extracted `Authority` struct trivial.

use abi::errors::{Errno, SysResult};
use thingos::authority::Authority;

/// Capability bit: reboot/power control operations.
pub const CAP_REBOOT: u64 = 1 << 0;
/// Capability bit: signal delivery to other processes/groups.
pub const CAP_SIGNAL: u64 = 1 << 1;
/// Capability bit: thread/process termination operations.
pub const CAP_KILL: u64 = 1 << 2;
/// Capability bit: realtime priority escalation.
pub const CAP_REALTIME_PRIORITY: u64 = 1 << 3;

fn capability_name(bit: u64) -> Option<&'static str> {
    match bit {
        CAP_REBOOT => Some("reboot"),
        CAP_SIGNAL => Some("signal"),
        CAP_KILL => Some("kill"),
        CAP_REALTIME_PRIORITY => Some("realtime_priority"),
        _ => None,
    }
}

fn capabilities_from_mask(mask: u64) -> alloc::vec::Vec<alloc::string::String> {
    let mut out = alloc::vec::Vec::new();
    for bit in [
        CAP_REBOOT,
        CAP_SIGNAL,
        CAP_KILL,
        CAP_REALTIME_PRIORITY,
    ] {
        if (mask & bit) != 0 {
            if let Some(name) = capability_name(bit) {
                out.push(alloc::string::String::from(name));
            }
        }
    }
    out
}

fn required_capability(privilege: &str) -> Option<u64> {
    match privilege {
        "reboot" => Some(CAP_REBOOT),
        "signal" => Some(CAP_SIGNAL),
        "kill" => Some(CAP_KILL),
        "realtime_priority" => Some(CAP_REALTIME_PRIORITY),
        _ => None,
    }
}

/// Build a canonical `Authority` from a [`crate::sched::hooks::ProcessSnapshot`].
///
/// # Transitional mapping
///
/// The principal (`uid`/`gid`) and capability bits are sourced from
/// `ProcessSnapshot` authority backing fields. `name` remains a
/// human-readable label (`snapshot.name` with `exec_path` fallback) and is not
/// used for privilege decisions.
///
/// # Note on provisional credential state
///
/// The current `Process` struct does not carry uid/gid, capability masks, or
/// service-account fields.  All credential/permission state in `Process` is
/// therefore **provisional** — it backs the canonical `Authority` through this
/// bridge but has not yet been fully extracted into `Authority`-shaped storage.
pub fn authority_from_snapshot(
    snapshot: &crate::sched::hooks::ProcessSnapshot,
) -> Authority {
    // Prefer the human-readable thread/process name; fall back to exec_path when
    // the name has not been set (i.e., it is empty).
    //
    // PROVISIONAL: This derives the authority name from the current process/thread
    // name.  Future phases will replace this with a stable principal identifier
    // once uid/gid-like fields or a service-account concept are introduced into
    // the Process struct.
    let name = if snapshot.name.is_empty() {
        snapshot.exec_path.clone()
    } else {
        snapshot.name.clone()
    };

    // PROVISIONAL: capabilities is always empty in Phase 7.  The current
    // `Process` struct carries no capability mask.  When a capability field is
    // added to `Process` (or its Authority-shaped substructure from Phase 5),
    // this is the sole site that must be updated to surface those capabilities
    // through the canonical `Authority` type.  New access-control code must NOT
    // read capability state from `Process` directly.
    Authority {
        uid: snapshot.uid,
        gid: snapshot.gid,
        name,
        capability_mask: snapshot.capability_mask,
        capabilities: capabilities_from_mask(snapshot.capability_mask),
    }
}

/// Return the canonical [`Authority`] for the **currently running task**.
///
/// This is the **preferred entry point** for any new authorization check that
/// needs to know "under what power is this action occurring?".  Callers should
/// use this function instead of reaching into `Process` fields directly.
///
/// # Transitional behaviour
///
/// This returns principal (`uid`/`gid`) and capability bits from the current
/// `Process` authority backing. When no process context is available (kernel
/// threads), the returned `Authority` uses root principal and full capability
/// mask.
///
/// # Migration note
///
/// Once uid/gid-like fields or a capability mask are added to `Process`, this
/// function remains the **single entry point** — callers will transparently
/// receive a richer `Authority` without code changes at call sites.
pub fn authority_for_current() -> Authority {
    if let Some(p) = crate::sched::process_info_current() {
        let p = p.lock();
        let name = if p.exec_path.is_empty() {
            alloc::format!("pid:{}", p.pid)
        } else {
            p.exec_path.clone()
        };
        let capability_mask = p.authority.capability_mask;
        Authority {
            uid: p.authority.uid,
            gid: p.authority.gid,
            name,
            capability_mask,
            capabilities: capabilities_from_mask(capability_mask),
        }
    } else {
        // Kernel threads execute with kernel authority.
        Authority {
            uid: 0,
            gid: 0,
            name: alloc::string::String::from("kernel"),
            capability_mask: u64::MAX,
            capabilities: capabilities_from_mask(u64::MAX),
        }
    }
}

/// Check whether an [`Authority`] holds a named privilege.
///
/// This is the **canonical gate** for privileged operations.  New
/// authorization checks must call this function instead of reading `Process`
/// fields directly.
///
/// # Transitional behaviour
///
/// Non-root callers must hold the capability bit mapped to `privilege`.
/// Unknown privilege names fail closed with `Err(Errno::EPERM)`.
///
/// # Usage
///
/// ```text
/// let authority = authority_for_current();
/// check_privilege(&authority, "reboot")?;
/// // ... proceed with privileged operation
/// ```
///
/// # Future direction
///
/// When a capability mask or role model is added to `Process` (or its
/// Authority-shaped substructure), this function will be updated to enforce
/// the check.  All call sites will automatically gain real enforcement without
/// code changes.
///
pub fn check_privilege(authority: &Authority, privilege: &str) -> SysResult<()> {
    // uid 0 is the privileged principal.
    if authority.uid == 0 {
        return Ok(());
    }

    let required = required_capability(privilege).ok_or(Errno::EPERM)?;
    if (authority.capability_mask & required) != 0 {
        Ok(())
    } else {
        Err(Errno::EPERM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::hooks::ProcessSnapshot;
    use crate::task::TaskState;

    fn make_snapshot(name: &str, exec_path: &str) -> ProcessSnapshot {
        ProcessSnapshot {
            pid: 1,
            ppid: 0,
            tid: 1,
            name: alloc::string::String::from(name),
            state: TaskState::Runnable,
            argv: alloc::vec::Vec::new(),
            exec_path: alloc::string::String::from(exec_path),
            uid: 1000,
            gid: 1000,
            capability_mask: 0,
            exit_code: None,
            pgid: 1,
            sid: 1,
            session_leader: false,
            foreground_pgid: None,
            cwd: alloc::string::String::from("/"),
            namespace_label: alloc::string::String::from("global"),
            root_path: alloc::string::String::from("/"),
            thread_states: alloc::vec![TaskState::Runnable],
            space_id: thingos::space::SpaceId::NONE,
            space_mapping_count: 0,
            space_sharing_count: 0,
        }
    }

    // ── authority_from_snapshot ──────────────────────────────────────────────

    #[test]
    fn test_authority_name_from_snapshot_uses_name() {
        let snap = make_snapshot("my-service", "/bin/my-service");
        let auth = authority_from_snapshot(&snap);
        assert_eq!(auth.name, "my-service");
    }

    #[test]
    fn test_authority_name_falls_back_to_exec_path_when_name_is_empty() {
        let snap = make_snapshot("", "/bin/fallback");
        let auth = authority_from_snapshot(&snap);
        assert_eq!(auth.name, "/bin/fallback");
    }

    #[test]
    fn test_authority_capabilities_are_empty_in_phase7() {
        let snap = make_snapshot("svc", "/bin/svc");
        let auth = authority_from_snapshot(&snap);
        assert!(auth.capabilities.is_empty());
    }

    #[test]
    fn test_authority_from_snapshot_carries_uid_gid_and_mask() {
        let mut snap = make_snapshot("svc", "/bin/svc");
        snap.uid = 42;
        snap.gid = 84;
        snap.capability_mask = CAP_REBOOT | CAP_SIGNAL;
        let auth = authority_from_snapshot(&snap);
        assert_eq!(auth.uid, 42);
        assert_eq!(auth.gid, 84);
        assert_eq!(auth.capability_mask, CAP_REBOOT | CAP_SIGNAL);
        assert_eq!(
            auth.capabilities,
            alloc::vec![
                alloc::string::String::from("reboot"),
                alloc::string::String::from("signal")
            ]
        );
    }

    #[test]
    fn test_authority_as_text_contains_name() {
        let snap = make_snapshot("bristle", "/bin/bristle");
        let auth = authority_from_snapshot(&snap);
        let text = auth.as_text();
        assert!(text.contains("name: bristle"), "unexpected text: {text}");
    }

    #[test]
    fn test_authority_as_text_contains_empty_capabilities() {
        let snap = make_snapshot("bristle", "/bin/bristle");
        let auth = authority_from_snapshot(&snap);
        let text = auth.as_text();
        assert!(text.contains("capabilities: []"), "unexpected text: {text}");
    }

    #[test]
    fn test_authority_as_text_ends_with_newline() {
        let snap = make_snapshot("x", "/bin/x");
        let auth = authority_from_snapshot(&snap);
        assert!(auth.as_text().ends_with('\n'));
    }

    // ── check_privilege ──────────────────────────────────────────────────────

    #[test]
    fn test_check_privilege_root_uid_is_allowed() {
        let snap = make_snapshot("svc", "/bin/svc");
        let mut auth = authority_from_snapshot(&snap);
        auth.uid = 0;
        assert!(check_privilege(&auth, "reboot").is_ok());
        assert!(check_privilege(&auth, "any_privilege").is_ok());
    }

    #[test]
    fn test_check_privilege_non_root_requires_matching_capability() {
        let mut auth = authority_from_snapshot(&make_snapshot("svc", "/bin/svc"));
        auth.uid = 1000;
        auth.capability_mask = CAP_SIGNAL;
        assert!(check_privilege(&auth, "signal").is_ok());
        assert_eq!(check_privilege(&auth, "reboot"), Err(Errno::EPERM));
    }

    #[test]
    fn test_check_privilege_unknown_privilege_fails_closed() {
        let auth = authority_from_snapshot(&make_snapshot("svc", "/bin/svc"));
        assert_eq!(check_privilege(&auth, "unknown_privilege"), Err(Errno::EPERM));
    }
}

//! Bridge layer: runtime/session state → canonical `thingos::presence::Presence`.

use thingos::presence::{EmbodimentKind, EntityRef, GroupRef, PlaceRef, Presence, PresenceMode};

fn tagged_ref(tag: [u8; 4], value: u32) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&tag);
    out[4..8].copy_from_slice(&value.to_le_bytes());
    out
}

const CONSOLE_PLACE_REF_BYTES: [u8; 16] =
    [b't', b't', b'y', 0, b'c', b'o', b'n', b's', b'o', b'l', b'e', 0, 0, 0, 0, 1];

/// Build canonical Presence from a process snapshot and runtime console
/// attachment state.
pub fn presence_from_snapshot(snapshot: &crate::sched::hooks::ProcessSnapshot) -> Presence {
    let tty_presence = crate::presence::console_presence_state();
    let is_attached_session = tty_presence.controlling_sid == Some(snapshot.sid);
    let foreground_pgid = tty_presence.foreground_pgid;

    let mode = if is_attached_session {
        if foreground_pgid == Some(snapshot.pgid) {
            PresenceMode::Active
        } else {
            PresenceMode::Latent
        }
    } else {
        PresenceMode::Deferred
    };

    Presence {
        subject: EntityRef(tagged_ref(*b"pid\0", snapshot.pid)),
        place: if is_attached_session { Some(PlaceRef(CONSOLE_PLACE_REF_BYTES)) } else { None },
        group: foreground_pgid.map(|pgid| GroupRef(tagged_ref(*b"pgid", pgid))),
        mode,
        embodiment: if is_attached_session { Some(EmbodimentKind::Direct) } else { None },
        observed_at: None,
    }
}

/// Return canonical Presence for the currently running task.
pub fn presence_for_current() -> Presence {
    if let Some(pinfo) = crate::sched::process_info_current() {
        let p = pinfo.lock();
        let tty_presence = crate::presence::console_presence_state();
        let is_attached_session = tty_presence.controlling_sid == Some(p.unix_compat.sid);
        let foreground_pgid = tty_presence.foreground_pgid;

        let mode = if is_attached_session {
            if foreground_pgid == Some(p.unix_compat.pgid) {
                PresenceMode::Active
            } else {
                PresenceMode::Latent
            }
        } else {
            PresenceMode::Deferred
        };

        return Presence {
            subject: EntityRef(tagged_ref(*b"pid\0", p.pid)),
            place: if is_attached_session { Some(PlaceRef(CONSOLE_PLACE_REF_BYTES)) } else { None },
            group: foreground_pgid.map(|pgid| GroupRef(tagged_ref(*b"pgid", pgid))),
            mode,
            embodiment: if is_attached_session { Some(EmbodimentKind::Direct) } else { None },
            observed_at: None,
        };
    }

    Presence {
        subject: EntityRef([0u8; 16]),
        place: None,
        group: None,
        mode: PresenceMode::Unavailable,
        embodiment: None,
        observed_at: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::hooks::ProcessSnapshot;
    use crate::task::TaskState;

    fn snapshot_with_session(pid: u32, sid: u32, pgid: u32) -> ProcessSnapshot {
        ProcessSnapshot {
            pid,
            ppid: 0,
            tid: pid as u64,
            name: alloc::string::String::from("test"),
            state: TaskState::Runnable,
            argv: alloc::vec::Vec::new(),
            exec_path: alloc::string::String::new(),
            uid: 0,
            gid: 0,
            capability_mask: 0,
            exit_code: None,
            pgid,
            sid,
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

    #[test]
    fn test_presence_from_snapshot_active_for_foreground_attached_group() {
        crate::presence::set_console_presence_for_test(Some(7), Some(11));
        let snap = snapshot_with_session(42, 7, 11);
        let p = presence_from_snapshot(&snap);
        assert_eq!(p.mode, PresenceMode::Active);
        assert_eq!(p.embodiment, Some(EmbodimentKind::Direct));
        assert!(p.place.is_some());
        assert_eq!(p.group, Some(GroupRef(tagged_ref(*b"pgid", 11))));
    }

    #[test]
    fn test_presence_from_snapshot_latent_for_background_attached_group() {
        crate::presence::set_console_presence_for_test(Some(7), Some(11));
        let snap = snapshot_with_session(42, 7, 99);
        let p = presence_from_snapshot(&snap);
        assert_eq!(p.mode, PresenceMode::Latent);
        assert!(p.place.is_some());
    }

    #[test]
    fn test_presence_from_snapshot_deferred_when_session_not_attached() {
        crate::presence::set_console_presence_for_test(Some(7), Some(11));
        let snap = snapshot_with_session(42, 8, 11);
        let p = presence_from_snapshot(&snap);
        assert_eq!(p.mode, PresenceMode::Deferred);
        assert_eq!(p.place, None);
        assert_eq!(p.embodiment, None);
    }
}

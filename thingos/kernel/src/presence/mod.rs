//! Presence module: first-class runtime ownership for TTY person-in-place state.
//!
//! This module owns controlling-terminal attachment and foreground-group
//! attachment for `/dev/console`.  Devfs and procfs use this module rather than
//! keeping these semantics as ad-hoc fields.

use spin::Mutex;

pub mod bridge;

/// Runtime Presence state for `/dev/console`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConsolePresenceState {
    /// Session attached as controlling owner of `/dev/console`.
    pub controlling_sid: Option<u32>,
    /// Foreground process group within the controlling session.
    pub foreground_pgid: Option<u32>,
}

static CONSOLE_PRESENCE_STATE: Mutex<ConsolePresenceState> =
    Mutex::new(ConsolePresenceState { controlling_sid: None, foreground_pgid: None });

/// Snapshot console presence state.
pub fn console_presence_state() -> ConsolePresenceState {
    *CONSOLE_PRESENCE_STATE.lock()
}

/// Return foreground PGID of the current console presence (if attached).
pub fn console_foreground_pgid() -> Option<u32> {
    CONSOLE_PRESENCE_STATE.lock().foreground_pgid
}

/// Attach a session leader to the console Presence when no controlling
/// attachment exists yet.
pub fn maybe_attach_console_presence(
    caller_sid: u32,
    caller_pgid: u32,
    caller_is_session_leader: bool,
) {
    if !caller_is_session_leader {
        return;
    }
    let mut state = CONSOLE_PRESENCE_STATE.lock();
    if state.controlling_sid.is_none() {
        state.controlling_sid = Some(caller_sid);
        state.foreground_pgid = Some(caller_pgid);
    }
}

/// Returns true when caller is in the attached session but not in its
/// foreground group.
pub fn is_background_console_presence(caller_sid: u32, caller_pgid: u32) -> bool {
    let state = CONSOLE_PRESENCE_STATE.lock();
    match (state.controlling_sid, state.foreground_pgid) {
        (Some(sid), Some(fg_pgid)) => caller_sid == sid && caller_pgid != fg_pgid,
        _ => false,
    }
}

/// Returns true when caller is in the controlling session for `/dev/console`.
pub fn caller_in_controlling_console_session(caller_sid: u32) -> bool {
    CONSOLE_PRESENCE_STATE.lock().controlling_sid == Some(caller_sid)
}

/// Update the foreground process group of the attached console presence.
pub fn set_console_foreground_pgid(new_pgid: u32) {
    CONSOLE_PRESENCE_STATE.lock().foreground_pgid = Some(new_pgid);
}

#[cfg(test)]
pub fn set_console_presence_for_test(controlling_sid: Option<u32>, foreground_pgid: Option<u32>) {
    let mut st = CONSOLE_PRESENCE_STATE.lock();
    st.controlling_sid = controlling_sid;
    st.foreground_pgid = foreground_pgid;
}

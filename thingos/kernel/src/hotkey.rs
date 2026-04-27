//! Deferred hotkey actions that must not run directly in interrupt context.

use alloc::collections::BTreeMap;
use alloc::string::ToString;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::BootRuntime;
use crate::task::{Affinity, StartupArg, TaskPriority, TaskState};

const HOTKEY_SHELL_LOCKED: u64 = u64::MAX;
const HOTKEY_WORKER_SLEEP_MS: u64 = 10;

static TERMINAL_SHELL_REQUESTED: AtomicBool = AtomicBool::new(false);
static HOTKEY_SHELL_TID: AtomicU64 = AtomicU64::new(0);
static HOTKEY_WORKER_STARTED: AtomicBool = AtomicBool::new(false);

/// Request that the framebuffer terminal shell be spawned from task context.
///
/// This is safe to call from IRQ/NMI paths: it only updates an atomic flag.
pub fn request_terminal_shell() {
    TERMINAL_SHELL_REQUESTED.store(true, Ordering::Release);
}

/// Start the kernel worker that handles deferred hotkey actions.
pub fn spawn_worker<R: BootRuntime>() {
    if HOTKEY_WORKER_STARTED.swap(true, Ordering::AcqRel) {
        return;
    }

    crate::task::spawn::<R>(
        terminal_hotkey_worker::<R>,
        StartupArg::None,
        TaskPriority::High,
        Affinity::Any,
    );
}

extern "C" fn terminal_hotkey_worker<R: BootRuntime>(_arg: usize) -> ! {
    loop {
        if TERMINAL_SHELL_REQUESTED.swap(false, Ordering::AcqRel) {
            handle_terminal_shell_request::<R>();
        }
        crate::sched::sleep_ms::<R>(HOTKEY_WORKER_SLEEP_MS);
    }
}

fn handle_terminal_shell_request<R: BootRuntime>() {
    crate::runtime_base().activate_onscreen_terminal();
    crate::irq::ps2::set_fb_input_enabled(true);

    if !try_lock_hotkey_shell_spawn::<R>() {
        let existing = HOTKEY_SHELL_TID.load(Ordering::Acquire);
        if existing != 0 && existing != HOTKEY_SHELL_LOCKED {
            crate::kdebug!("F12 hotkey: shell already running (tid={})", existing);
        }
        return;
    }

    let spawned_tid = try_spawn_shell();
    unlock_hotkey_shell_spawn(spawned_tid);

    if let Some(tid) = spawned_tid {
        crate::kdebug!("F12 hotkey: spawned /bin/sh as tid {}", tid);
    } else {
        crate::kdebug!("F12 hotkey: failed to spawn /bin/sh");
    }
}

fn try_spawn_shell() -> Option<u64> {
    let tty_path = "/dev/tty0".to_string();
    let res = unsafe {
        crate::sched::spawn_process_from_path_current(
            "/bin/sh",
            alloc::vec![b"/bin/sh".to_vec()],
            BTreeMap::new(),
            crate::sched::StdioSpec::Path(tty_path.clone()),
            crate::sched::StdioSpec::Path(tty_path.clone()),
            crate::sched::StdioSpec::Path(tty_path),
            0,
            alloc::vec![],
            Some("/".to_string()),
            alloc::vec![],
            None,
        )
    };
    res.ok().map(|r| r.child_tid)
}

fn hotkey_shell_is_alive<R: BootRuntime>(tid: u64) -> bool {
    match unsafe { crate::sched::task_status_current(tid) } {
        Some((state, _)) => state != TaskState::Dead,
        None => false,
    }
}

fn try_lock_hotkey_shell_spawn<R: BootRuntime>() -> bool {
    loop {
        let current = HOTKEY_SHELL_TID.load(Ordering::Acquire);

        if current == HOTKEY_SHELL_LOCKED {
            return false;
        }

        if current != 0 {
            if hotkey_shell_is_alive::<R>(current) {
                return false;
            }

            if HOTKEY_SHELL_TID
                .compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                continue;
            }
        }

        if HOTKEY_SHELL_TID
            .compare_exchange(0, HOTKEY_SHELL_LOCKED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return true;
        }
    }
}

fn unlock_hotkey_shell_spawn(spawned_tid: Option<u64>) {
    HOTKEY_SHELL_TID.store(spawned_tid.unwrap_or(0), Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_terminal_shell_sets_pending_flag() {
        TERMINAL_SHELL_REQUESTED.store(false, Ordering::Release);

        request_terminal_shell();

        assert!(TERMINAL_SHELL_REQUESTED.swap(false, Ordering::AcqRel));
    }
}

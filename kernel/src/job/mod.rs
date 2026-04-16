//! Job module: canonical lifecycle container bridging the current `Process`
//! model toward the future `Job` ontology.
//!
//! # Transitional mapping
//!
//! | Canonical concept | Current internal backing  | Future direction             |
//! |-------------------|---------------------------|------------------------------|
//! | `thingos::job::Job` | kernel `Process` (partial) | hollowed out by further phases |
//!
//! The `Process` struct is retained as-is for now.  This module introduces
//! the public `Job` shape at the edges of the system, allowing the new
//! ontology to appear externally while the internal machinery is migrated
//! gradually (Phase 3 onwards).

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::task::TaskId;

/// First-class kernel Job object.
///
/// This object owns lifecycle truth for a thread group. `Process` keeps this as
/// a field for transitional compatibility, but lifecycle semantics flow through
/// this object rather than top-level `Process` fields.
///
/// Fields remain `pub` so existing scheduler/signal/syscall paths can migrate
/// incrementally without forcing broad accessor churn in this extraction phase.
pub struct Job {
    /// Parent process/job ID used for parent-child wait linkage.
    pub ppid: u32,
    /// Runtime TIDs currently associated with this job's thread group.
    pub thread_ids: Vec<TaskId>,
    /// Exec gate flag; true while exec collapse is in progress.
    pub exec_in_progress: bool,
    /// Parent-facing queue of child completion statuses for `waitpid`.
    pub children_done: VecDeque<(u32, i32)>,
    /// Optional observer inbox for canonical `JobExit` notifications.
    pub exit_observer_inbox: Option<crate::inbox::InboxId>,
    /// Leader exit code once the thread-group leader has exited.
    pub leader_exit_code: Option<i32>,
    /// Waiters blocked on leader-exit observation.
    pub leader_exit_waiters: crate::sched::WaitQueue,
}

impl Job {
    pub fn new(ppid: u32, leader_tid: TaskId) -> Self {
        Self {
            ppid,
            thread_ids: alloc::vec![leader_tid],
            exec_in_progress: false,
            children_done: VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        }
    }

    /// Record leader exit and drain any registered leader-exit waiters.
    ///
    /// Returns waiter task IDs that should be woken by the scheduler.
    pub fn complete_leader_exit(&mut self, code: i32) -> Vec<u64> {
        self.leader_exit_code = Some(code);
        self.leader_exit_waiters.drain()
    }
}

pub mod bridge;
pub mod notify;

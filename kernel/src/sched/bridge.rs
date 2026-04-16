//! Bridge layer: canonical generated kinds → scheduler runtime/cache projections.
//!
//! # Ownership boundary
//!
//! - **Generated kinds own semantic truth** (`thingos::task::Task` and friends).
//! - **Scheduler cache structs own dispatch/runtime mechanics**
//!   (`crate::sched::state::TaskSchedFields` and run-queue metadata).
//!
//! This module is intentionally one-way for now: canonical kind → runtime/cache.

use alloc::string::String;

use crate::sched::state::{self, ThreadId};

/// Runtime-facing projection of a canonical `thingos::task::Task`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskRuntime {
    pub state: state::TaskState,
    pub job: JobRuntimeBridge,
    pub name: Option<String>,
}

/// Narrow runtime contract for scheduler dispatch decisions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulableRuntime {
    pub state: state::TaskState,
    pub priority: crate::task::TaskPriority,
    pub affinity: crate::task::Affinity,
    pub current_cpu: Option<usize>,
    pub last_cpu: Option<usize>,
    pub slice_remaining: u32,
    pub enqueued_at_tick: u64,
    pub wake_pending: bool,
}

/// Scheduler-cache projection from canonical task state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskSchedCache {
    pub state: state::TaskState,
    pub priority: crate::task::TaskPriority,
    pub affinity: crate::task::Affinity,
    pub last_cpu: Option<usize>,
    pub wake_cpu: Option<usize>,
    pub run_cpu: Option<usize>,
    pub timeslice_remaining: u32,
    pub enqueued_at_tick: u64,
    pub wake_pending: bool,
}

/// Runtime bridge for task→job association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JobRuntimeBridge {
    pub id: Option<u32>,
}

impl JobRuntimeBridge {
    #[inline]
    pub fn is_bound(self) -> bool {
        self.id.is_some()
    }
}

impl TaskRuntime {
    /// Load runtime projection from canonical generated Task.
    pub fn from_generated(task: &thingos::task::Task) -> Self {
        Self {
            state: sched_state_from_generated(task.state),
            job: JobRuntimeBridge { id: task.job },
            name: task.name.clone(),
        }
    }
}

impl SchedulableRuntime {
    /// Project dispatch-relevant runtime fields from a concrete task record.
    pub fn from_thread<R: crate::BootRuntime>(thread: &crate::task::Thread<R>) -> Self {
        let current_cpu =
            if thread.state == state::TaskState::Running { thread.last_cpu } else { None };
        Self {
            state: thread.state,
            priority: thread.priority,
            affinity: thread.affinity,
            current_cpu,
            last_cpu: thread.last_cpu,
            slice_remaining: thread.timeslice_remaining,
            enqueued_at_tick: thread.enqueued_at_tick,
            wake_pending: thread.wake_pending,
        }
    }
}

impl TaskSchedCache {
    /// Load scheduler cache projection from canonical generated Task.
    pub fn from_generated(task: &thingos::task::Task) -> Self {
        let runtime = TaskRuntime::from_generated(task);
        Self::from_runtime(&runtime)
    }

    /// Load scheduler cache projection from runtime projection.
    pub fn from_runtime(runtime: &TaskRuntime) -> Self {
        let exited = runtime.state == state::TaskState::Dead;
        let schedulable = SchedulableRuntime {
            state: runtime.state,
            priority: crate::task::TaskPriority::Normal,
            affinity: crate::task::Affinity::Any,
            current_cpu: None,
            last_cpu: None,
            slice_remaining: if exited { 0 } else { crate::sched::DEFAULT_TIMESLICE },
            enqueued_at_tick: 0,
            wake_pending: false,
        };
        Self::from_schedulable_runtime(&schedulable)
    }

    /// Load scheduler cache projection from the schedulable runtime boundary.
    pub fn from_schedulable_runtime(runtime: &SchedulableRuntime) -> Self {
        Self {
            state: runtime.state,
            priority: runtime.priority,
            affinity: runtime.affinity,
            last_cpu: runtime.last_cpu,
            wake_cpu: None,
            run_cpu: runtime.current_cpu,
            timeslice_remaining: runtime.slice_remaining,
            enqueued_at_tick: runtime.enqueued_at_tick,
            wake_pending: runtime.wake_pending,
        }
    }

    /// Load scheduler cache projection from a concrete runtime thread record.
    pub fn from_thread<R: crate::BootRuntime>(thread: &crate::task::Thread<R>) -> Self {
        let runtime = SchedulableRuntime::from_thread(thread);
        Self::from_schedulable_runtime(&runtime)
    }

    #[inline]
    pub fn with_wake_cpu(mut self, wake_cpu: Option<usize>) -> Self {
        self.wake_cpu = wake_cpu;
        self
    }

    #[inline]
    pub fn with_run_cpu(mut self, run_cpu: Option<usize>) -> Self {
        self.run_cpu = run_cpu;
        self
    }

    /// Materialize concrete scheduler hot-cache fields for a specific `tid`.
    pub fn into_sched_fields(self, tid: ThreadId) -> state::TaskSchedFields {
        state::TaskSchedFields {
            tid,
            runq_location: None,
            state: self.state,
            priority: self.priority,
            affinity: self.affinity,
            last_cpu: self.last_cpu,
            wake_cpu: self.wake_cpu,
            run_cpu: self.run_cpu,
            timeslice_remaining: self.timeslice_remaining,
            enqueued_at_tick: self.enqueued_at_tick,
            wake_pending: self.wake_pending,
        }
    }
}

/// Map canonical generated `TaskState` into scheduler internal task state.
#[inline]
pub fn sched_state_from_generated(state: thingos::task::TaskState) -> state::TaskState {
    match state {
        thingos::task::TaskState::New => state::TaskState::Blocked,
        thingos::task::TaskState::Ready => state::TaskState::Runnable,
        thingos::task::TaskState::Running => state::TaskState::Running,
        thingos::task::TaskState::Blocked => state::TaskState::Blocked,
        thingos::task::TaskState::Exited => state::TaskState::Dead,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task(state: thingos::task::TaskState, job: Option<u32>) -> thingos::task::Task {
        thingos::task::Task { state, job, name: Some(alloc::string::String::from("demo")) }
    }

    #[test]
    fn generated_task_loads_runtime_projection() {
        let task = make_task(thingos::task::TaskState::Ready, Some(7));
        let runtime = TaskRuntime::from_generated(&task);
        assert_eq!(runtime.state, state::TaskState::Runnable);
        assert_eq!(runtime.job, JobRuntimeBridge { id: Some(7) });
        assert_eq!(runtime.name.as_deref(), Some("demo"));
    }

    #[test]
    fn generated_task_defaults_cache_dispatch_fields() {
        let task = make_task(thingos::task::TaskState::Running, None);
        let cache = TaskSchedCache::from_generated(&task);
        assert_eq!(cache.state, state::TaskState::Running);
        assert_eq!(cache.priority, crate::task::TaskPriority::Normal);
        assert_eq!(cache.affinity, crate::task::Affinity::Any);
        assert_eq!(cache.timeslice_remaining, crate::sched::DEFAULT_TIMESLICE);
        assert_eq!(cache.last_cpu, None);
        assert_eq!(cache.wake_cpu, None);
        assert_eq!(cache.run_cpu, None);
        assert!(!cache.wake_pending);
    }

    #[test]
    fn exited_task_enforces_cache_invariant_timeslice_zero() {
        let task = make_task(thingos::task::TaskState::Exited, Some(1));
        let cache = TaskSchedCache::from_generated(&task);
        assert_eq!(cache.state, state::TaskState::Dead);
        assert_eq!(cache.timeslice_remaining, 0);
    }

    #[test]
    fn new_task_defaults_to_blocked_until_scheduler_admission() {
        let task = make_task(thingos::task::TaskState::New, None);
        let cache = TaskSchedCache::from_generated(&task);
        assert_eq!(cache.state, state::TaskState::Blocked);
    }

    #[test]
    fn cache_projection_materializes_task_sched_fields() {
        let task = make_task(thingos::task::TaskState::Ready, Some(11));
        let fields = TaskSchedCache::from_generated(&task).into_sched_fields(42);
        assert_eq!(fields.tid, 42);
        assert_eq!(fields.runq_location, None);
        assert_eq!(fields.state, state::TaskState::Runnable);
        assert_eq!(fields.priority, crate::task::TaskPriority::Normal);
        assert_eq!(fields.affinity, crate::task::Affinity::Any);
    }

    #[test]
    fn schedulable_runtime_projection_keeps_dispatch_fields() {
        let runtime = SchedulableRuntime {
            state: state::TaskState::Runnable,
            priority: crate::task::TaskPriority::High,
            affinity: crate::task::Affinity::Pinned(2),
            current_cpu: None,
            last_cpu: Some(2),
            slice_remaining: 17,
            enqueued_at_tick: 99,
            wake_pending: true,
        };
        let cache = TaskSchedCache::from_schedulable_runtime(&runtime)
            .with_wake_cpu(Some(1))
            .into_sched_fields(7);
        assert_eq!(cache.tid, 7);
        assert_eq!(cache.state, state::TaskState::Runnable);
        assert_eq!(cache.priority, crate::task::TaskPriority::High);
        assert_eq!(cache.affinity, crate::task::Affinity::Pinned(2));
        assert_eq!(cache.last_cpu, Some(2));
        assert_eq!(cache.wake_cpu, Some(1));
        assert_eq!(cache.timeslice_remaining, 17);
        assert_eq!(cache.enqueued_at_tick, 99);
        assert!(cache.wake_pending);
    }

    #[test]
    fn job_runtime_bridge_reports_binding() {
        assert!(!JobRuntimeBridge { id: None }.is_bound());
        assert!(JobRuntimeBridge { id: Some(9) }.is_bound());
    }
}

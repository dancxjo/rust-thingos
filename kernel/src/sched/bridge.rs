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

impl TaskSchedCache {
    /// Load scheduler cache projection from canonical generated Task.
    pub fn from_generated(task: &thingos::task::Task) -> Self {
        let runtime = TaskRuntime::from_generated(task);
        Self::from_runtime(&runtime)
    }

    /// Load scheduler cache projection from runtime projection.
    pub fn from_runtime(runtime: &TaskRuntime) -> Self {
        let exited = runtime.state == state::TaskState::Dead;
        Self {
            state: runtime.state,
            priority: crate::task::TaskPriority::Normal,
            affinity: crate::task::Affinity::Any,
            last_cpu: None,
            wake_cpu: None,
            run_cpu: None,
            timeslice_remaining: if exited { 0 } else { crate::sched::DEFAULT_TIMESLICE },
            enqueued_at_tick: 0,
            wake_pending: false,
        }
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
        thingos::task::Task {
            state,
            job,
            name: Some(alloc::string::String::from("demo")),
        }
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
    fn job_runtime_bridge_reports_binding() {
        assert!(!JobRuntimeBridge { id: None }.is_bound());
        assert!(JobRuntimeBridge { id: Some(9) }.is_bound());
    }
}

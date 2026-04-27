//! Deferred synchronization of scheduler state back to the canonical task registry.
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use super::types;
use super::profiling::*;
use crate::BootRuntime;
use crate::task::registry;

/// Apply a batch of deferred state updates to the canonical REGISTRY.
///
/// This must be called after releasing the SCHEDULER lock to maintain the
/// `SCHEDULER -> REGISTRY` lock order and prevent deadlocks on SMP.
pub(crate) fn apply_deferred_registry_syncs<R: BootRuntime>(
    syncs: Vec<types::DeferredRegistrySync>,
) {
    if syncs.is_empty() {
        return;
    }
    let mut reg = registry::get_registry::<R>();
    for sync in syncs {
        if let Some(task) = reg.get_mut(sync.tid) {
            if let Some(state) = sync.new_state {
                task.state = state;
            }
            if let Some(tick) = sync.new_enqueued_at_tick {
                task.enqueued_at_tick = tick;
            }
            if let Some(cpu) = sync.new_last_cpu {
                task.last_cpu = Some(cpu);
            }
        }
    }
}

/// Insert a batch of new tasks into the canonical REGISTRY.
///
/// This must be called after releasing the SCHEDULER lock.
pub(crate) fn apply_deferred_registry_inserts<R: BootRuntime>(
    inserts: Vec<alloc::boxed::Box<crate::task::Task<R>>>,
) {
    if inserts.is_empty() {
        return;
    }
    let mut reg = registry::get_registry::<R>();
    for task in inserts {
        PROF_RUNNABLE_TRANSITIONS.fetch_add(1, Ordering::Relaxed);
        reg.insert(task);
    }
}

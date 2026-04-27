//! Preemptive priority-based scheduler
//!
//! This module is split into focused submodules:
//! - `types`: Core data structures and enums
//! - `blocking`: Task blocking and wake primitives
//! - `hooks`: Type-erased hook system for callers without generic params
//! - `spawn`: Task and thread spawning
//! - `stack`: User stack allocation and fault handling
//! - `sleep`: Timing and yield functions
//! - `events`: Lock-free scheduler event types
//! - `metrics`: Telemetry and performance monitoring
//! - `balancing`: Load balancing and CPU selection logic
//! - `mailbox`: Cross-CPU wakeup mailboxes
//! - `registry_sync`: Deferred task registry updates
//! - `dispatch`: Main interrupt handlers and core dispatch loop
//!
//! ## Per-CPU Ownership Model
//!
//! Each logical CPU has an exclusive [`state::CpuScheduler`] that owns:
//! - `current` — the task currently executing on this CPU.
//! - `idle_task` — the CPU-local idle task.
//! - `runq` — the local priority-indexed run queue (see [`state::RunQueue`]).
//! - `need_resched` — the reschedule-pending flag for this CPU.
//! - `stats` — per-CPU scheduling counters.
//!
//! **Invariant**: A CPU owns its [`state::CpuScheduler`] state. Other CPUs
//! **must not** directly mutate another CPU's local run queue. Cross-CPU
//! scheduling effects go through explicit delivery mechanisms (remote-wake
//! mailboxes, IPIs). See [`mailbox::REMOTE_WAKE_MAILBOXES`] and later issues for the
//! mailbox path.
//!
//! Normal scheduling on CPU *N* reads and writes through
//! `sched.state.per_cpu[N]` (a [`state::CpuScheduler`] value). The global
//! [`Scheduler`] coordinates cross-CPU policy — task placement, load
//! balancing, diagnostics — without owning CPU-local execution state directly.
//!
//! ## Per-CPU Preemption (No Global Preemption Lock)
//!
//! Preemption decisions are made **independently per CPU** without a global
//! preemption lock:
//!
//! * The per-CPU `need_resched` flag (mirrored to the lockless
//!   [`GLOBAL_NEED_RESCHED`] array) is set by:
//!   - the timer tick handler when the current task's timeslice expires,
//!   - the remote-wake mailbox drain when a higher-priority task is woken,
//!   - cross-CPU wakeup/IPI delivery paths.
//! * Safe-point callers (e.g. `task::resched_if_needed`,
//!   `task::preempt_enable`) first check [`need_resched_pending`] atomically
//!   — **without** acquiring the scheduler lock — and only acquire the lock
//!   when a reschedule is actually needed.
//! * Each CPU therefore makes its own preemption decision based solely on
//!   its own per-CPU atomic flag, scaling independently with CPU count.
//!
//! Lock-order policy:
//! - `SCHEDULER` must never take `task::registry::REGISTRY` or
//!   `device_registry::REGISTRY`.
//! - Scheduler-owned paths defer registry/device work and apply it after
//!   releasing `SCHEDULER`.
//!
//! ## Architecture Guardrails
//!
//! Thing-OS follows a scheduler-first design where every unit of execution is a
//! kernel-scheduled task. This module implements that core guarantee.

pub(crate) mod balancing;
pub(crate) mod blocking;
pub mod bridge;
pub(crate) mod dispatch;
pub mod hooks;
pub use hooks::protect_user_range_current;
pub mod lifecycle;
pub use lifecycle::*;
pub(crate) mod mailbox;
pub(crate) mod metrics;
pub mod policy;
pub mod profiling;
pub use profiling::*;
pub(crate) use profiling::{is_task_on_other_cpu, set_cpu_current_task};
pub(crate) mod registry_sync;
mod sleep;
mod spawn;
mod stack;
pub mod state;
pub(crate) mod types;
mod vm;
pub(crate) mod wait_queue;

// Re-export all public items
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicIsize, AtomicU64, Ordering};

pub use blocking::{
    block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased,
};
pub use hooks::{
    CpuSchedDiag, ProcessSnapshot, SchedDiag, add_user_mapping_current, alloc_user_stack_current,
    available_parallelism_current, check_user_mapping_current, collect_sched_diag_current,
    current_priority_current, current_task_name_current, current_task_resource_id,
    current_tid_current, current_user_fs_base_current, dump_stats_current, exit_current,
    get_signal_mask_current, get_thread_pending_current, get_user_mapping_at_current,
    handle_user_stack_fault_current, interrupt_task_current, kill_by_tid_current,
    list_process_ids_by_pgid_current, list_processes_current, poll_task_exit_current,
    process_info_current, process_info_for_pid_current, process_info_for_tid_current,
    register_task_exit_waiter_current, register_timeout_wake_current, remove_user_mappings_current,
    set_current_task_name_current, set_current_user_fs_base_current, set_priority_current,
    set_signal_mask_current, set_thread_pending_current, sleep_ticks_current,
    spawn_process_current, spawn_process_ex_current, spawn_process_from_path_current,
    spawn_user_thread_current, take_pending_interrupt_current, task_exec_current,
    task_status_current, task_wait_current, unregister_task_exit_waiter_current,
    unregister_timeout_wake_current, waitpid_current, yield_now_current,
};
pub use policy::{DefaultPolicy, SchedPolicy};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    SpawnExResult, StdioSpec, boot_spawn_process, spawn, spawn_user_task_full, spawn_user_thread,
    spawn_user_thread_ex, spawn_with_priority, user_thread_trampoline,
};
use spin::Mutex;
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use state::{
    CpuSchedStats, CpuScheduler, MigrationState, RunQueue, WakeMailbox, WakeMailboxEntry,
};
pub use types::{
    DEFAULT_TIMESLICE, ScheduleReason, Scheduler, StackFaultResult, SwitchDecision, SwitchParams,
};
pub use wait_queue::WaitQueue;

pub use metrics::*;
pub use dispatch::{DispatchTrigger, emit_debug_summary, on_resched_ipi, on_tick};
pub(crate) use dispatch::{resolve_switch_params, send_deferred_prepare_schedule_ipis};
pub use mailbox::enqueue_remote_wake_mailbox;
pub(crate) use mailbox::claim_remote_wake_mailbox_ipi_epoch;
pub(crate) use balancing::{choose_wake_cpu, cross_cpu_runq_migration_enabled};
pub(crate) use registry_sync::{apply_deferred_registry_inserts, apply_deferred_registry_syncs};

use crate::task::{StartupArg, Task, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);
pub static SCHEDULER_LOCK_OWNER: AtomicIsize = AtomicIsize::new(-1);
pub static SCHEDULER_LOCK_ACQUIRED_AT: AtomicU64 = AtomicU64::new(0);
pub(super) const RESCHED_IPI_NEVER_SENT: u64 = u64::MAX;
const RESCHED_IPI_MIN_TICK_DELTA: u64 = 2;

pub struct SchedLockTrackingGuard<R: BootRuntime> {
    owner_cpu: isize,
    _phantom: PhantomData<R>,
}

#[inline]
fn clear_sched_lock_tracking_owner(owner_cpu: isize) {
    if SCHEDULER_LOCK_OWNER
        .compare_exchange(owner_cpu, -1, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        SCHEDULER_LOCK_ACQUIRED_AT.store(0, Ordering::Release);
    }
}

#[inline]
pub fn set_sched_lock_tracking<R: BootRuntime>(cpu_idx: usize) {
    SCHEDULER_LOCK_OWNER.store(cpu_idx as isize, Ordering::Release);
    // Use TICK_COUNT instead of mono_ticks to avoid false-positive deadlocks caused by TSC
    // desynchronization across CPUs (especially between BSP and APs under KVM).
    // Store TICK_COUNT + 1 so that a tick count of 0 does not disable the watchdog.
    SCHEDULER_LOCK_ACQUIRED_AT.store(TICK_COUNT.load(Ordering::Relaxed) + 1, Ordering::Release);
}

#[inline]
pub fn sched_lock_tracking_guard<R: BootRuntime>(cpu_idx: usize) -> SchedLockTrackingGuard<R> {
    set_sched_lock_tracking::<R>(cpu_idx);
    SchedLockTrackingGuard { owner_cpu: cpu_idx as isize, _phantom: PhantomData }
}

#[inline]
pub fn clear_sched_lock_tracking<R: BootRuntime>() {
    let cpu_owner = crate::runtime::<R>().current_cpu_index() as isize;
    clear_sched_lock_tracking_owner(cpu_owner);
}

impl<R: BootRuntime> Drop for SchedLockTrackingGuard<R> {
    fn drop(&mut self) {
        clear_sched_lock_tracking_owner(self.owner_cpu);
    }
}

#[inline]
fn debug_assert_scheduler_not_held_by_this_cpu<R: BootRuntime>(context: &str) {
    let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
    let cpu = crate::runtime::<R>().current_cpu_index() as isize;
    // Keep lightweight telemetry in all builds; debug builds also assert.
    if owner == cpu {
        PROF_LOCK_ORDER_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
    }
    #[cfg(debug_assertions)]
    {
        debug_assert_ne!(
            owner, cpu,
            "scheduler lock-order violation: {} attempted while SCHEDULER is held on CPU {}",
            context, cpu
        );
    }
}

/// Assert that a run-queue enqueue targets the **calling CPU** only.
///
/// Per-CPU run queues are **owned** by their CPU.  Only the owning CPU must
/// enqueue tasks directly into its own run queue; all other CPUs must route
/// through the per-CPU [`WakeMailbox`][crate::sched::state::WakeMailbox] (via
/// [`enqueue_remote_wake_mailbox`]).
///
/// This function increments [`PROF_CROSS_CPU_RUNQ_DIRECT_ENQUEUE`] in all
/// build configurations whenever a violation is detected, and additionally
/// fires a [`debug_assert`] in debug builds.
#[inline]
fn debug_assert_runq_cpu_is_local<R: BootRuntime>(target_cpu: usize) {
    let current = crate::runtime::<R>().current_cpu_index();
    if target_cpu != current {
        PROF_CROSS_CPU_RUNQ_DIRECT_ENQUEUE.fetch_add(1, Ordering::Relaxed);
        #[cfg(all(debug_assertions, not(test)))]
        debug_assert_eq!(
            target_cpu, current,
            "SCHED ownership violation: CPU {} attempted direct enqueue into CPU {}'s run queue; \
             use enqueue_remote_wake_mailbox() for cross-CPU operations",
            current, target_cpu
        );
    }
}

#[inline]
pub(crate) fn scheduler_lock_held_by_this_cpu<R: BootRuntime>() -> bool {
    let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
    owner == crate::runtime::<R>().current_cpu_index() as isize
}

/// Lock-skip self-healing: when try_resched_if_needed() fails to acquire
/// the scheduler lock, set this flag so the next safe-point yields.
///
/// This is per-CPU to prevent one CPU from accidentally consuming another's
/// reschedule request.
pub(super) static GLOBAL_NEED_RESCHED: [AtomicBool; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
    [ATOMIC_FALSE; types::MAX_CPUS]
};

/// Per-CPU start tick for the current try-lock miss warning window.
pub(super) static TRYLOCK_MISS_WINDOW_START: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses within the current warning window.
pub(super) static TRYLOCK_MISS_WINDOW_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of timer-triggered try-lock misses within the current warning window.
pub(super) static TRYLOCK_MISS_WINDOW_TIMER_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of IPI-triggered try-lock misses within the current warning window.
pub(super) static TRYLOCK_MISS_WINDOW_IPI_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses that happened on timer ticks while this CPU
/// was already running its idle task and had no pending reschedule request.
pub(super) static TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses observed while a reschedule request was
/// already pending for this CPU.
pub(super) static TRYLOCK_MISS_WINDOW_PENDING_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU tick timestamp when the try-lock miss threshold warning was last emitted.
pub(super) static TRYLOCK_MISS_LAST_WARN_TICK: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

#[inline]
pub fn current_cpu_index<R: BootRuntime>() -> usize {
    crate::runtime::<R>().current_cpu_index()
}

#[inline]
pub fn set_global_need_resched(cpu: usize) -> bool {
    if cpu < types::MAX_CPUS {
        GLOBAL_NEED_RESCHED[cpu].swap(true, Ordering::AcqRel)
    } else {
        false
    }
}

#[inline]
pub fn clear_global_need_resched(cpu: usize, ordering: Ordering) {
    if cpu < types::MAX_CPUS {
        GLOBAL_NEED_RESCHED[cpu].store(false, ordering);
    }
}

#[inline]
pub fn global_need_resched_swap(cpu: usize, val: bool, ordering: Ordering) -> bool {
    if cpu < types::MAX_CPUS {
        GLOBAL_NEED_RESCHED[cpu].swap(val, ordering)
    } else {
        false
    }
}

#[inline]
pub fn global_need_resched_load(cpu: usize, ordering: Ordering) -> bool {
    if cpu < types::MAX_CPUS {
        GLOBAL_NEED_RESCHED[cpu].load(ordering)
    } else {
        false
    }
}

#[inline]
pub fn need_resched_pending(cpu: usize) -> bool {
    global_need_resched_load(cpu, Ordering::Acquire)
}

#[inline]
pub(crate) fn should_send_remote_resched_ipi(target_cpu: usize) -> bool {
    if target_cpu >= types::MAX_CPUS {
        return true;
    }
    let now_tick = TICK_COUNT.load(Ordering::Relaxed);
    let last_tick = profiling::LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].load(Ordering::Relaxed);
    if last_tick != RESCHED_IPI_NEVER_SENT
        && now_tick.saturating_sub(last_tick) < RESCHED_IPI_MIN_TICK_DELTA
    {
        PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    profiling::LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].store(now_tick, Ordering::Relaxed);
    true
}

pub fn init<R: BootRuntime>() {
    let mut lock = SCHEDULER.lock();
    set_sched_lock_tracking::<R>(0);
    if lock.is_none() {
        let sched = alloc::boxed::Box::new(types::Scheduler::<R>::new());
        let s = alloc::boxed::Box::leak(sched);
        init_boot_task::<R>(s);
        *lock = Some(s as *mut types::Scheduler<R> as usize);
        unsafe {
            hooks::YIELD_HOOK = Some(sleep::yield_now::<R>);
            hooks::EXIT_HOOK = Some(exit::<R>);
            hooks::SPAWN_USER_HOOK = Some(spawn::spawn_user_thread_ex::<R>);
            hooks::SPAWN_PROCESS_HOOK = Some(spawn::boot_spawn_process::<R>);
            hooks::CURRENT_TID_HOOK = Some(current_tid::<R>);
            hooks::INTERRUPT_TASK_HOOK = Some(lifecycle::interrupt_task::<R>);
            hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(lifecycle::take_pending_interrupt::<R>);
            hooks::TASK_STATUS_HOOK = Some(task_status::<R>);
            hooks::TASK_WAIT_HOOK = Some(wait_task::<R>);
            hooks::SET_PRIORITY_HOOK = Some(set_priority::<R>);
            hooks::CURRENT_PRIORITY_HOOK = Some(current_priority::<R>);
            hooks::AVAILABLE_PARALLELISM_HOOK = Some(available_parallelism::<R>);
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            hooks::RUN_SCHEDULER_HOOK = Some(crate::task::run_scheduler::<R>);
            hooks::KILL_BY_TID_HOOK = Some(kill_by_tid::<R>);
            hooks::DUMP_STATS_HOOK = Some(crate::task::dump_stats::<R>);
            hooks::COLLECT_SCHED_DIAG_HOOK = Some(collect_sched_diag::<R>);
            crate::memory::set_map_user_page_hook(stack::map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(stack::map_user_page_perms::<R>);
            crate::memory::set_unmap_user_page_hook(stack::unmap_user_page::<R>);
            crate::memory::set_protect_user_page_hook(stack::protect_user_page::<R>);
            hooks::STACK_FAULT_HOOK = Some(stack::handle_stack_fault::<R>);
            hooks::SLEEP_TICKS_HOOK = Some(sleep::sleep_ticks::<R>);
            hooks::ADD_USER_MAPPING_HOOK = Some(vm::add_user_mapping::<R>);
            hooks::REMOVE_USER_MAPPINGS_HOOK = Some(vm::remove_user_mappings::<R>);
            hooks::CHECK_USER_MAPPING_HOOK = Some(vm::check_user_mapping::<R>);
            hooks::GET_USER_MAPPING_AT_HOOK = Some(vm::get_user_mapping_at::<R>);
            hooks::PROTECT_USER_RANGE_HOOK = Some(vm::protect_user_range::<R>);
            hooks::PROCESS_INFO_HOOK = Some(process_info::<R>);
            hooks::PROCESS_INFO_FOR_TID_HOOK = Some(process_info_for_tid::<R>);
            hooks::PROCESS_INFO_FOR_PID_HOOK = Some(process_info_for_pid::<R>);
            hooks::SPAWN_PROCESS_EX_HOOK = Some(spawn::boot_spawn_process_ex::<R>);
            hooks::SPAWN_PROCESS_FROM_PATH_HOOK = Some(spawn::spawn_process_from_path::<R>);
            hooks::CURRENT_RESOURCE_HOOK = Some(lifecycle::current_task_resource_id_impl::<R>);
            hooks::POLL_TASK_EXIT_HOOK = Some(poll_task_exit::<R>);
            hooks::REGISTER_TASK_EXIT_WAITER_HOOK = Some(register_task_exit_waiter_public::<R>);
            hooks::UNREGISTER_TASK_EXIT_WAITER_HOOK = Some(unregister_task_exit_waiter::<R>);
            hooks::REGISTER_TIMEOUT_WAKE_HOOK = Some(register_timeout_wake::<R>);
            hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = Some(unregister_timeout_wake::<R>);
            hooks::LIST_PROCESSES_HOOK = Some(list_processes::<R>);
            hooks::LIST_PROCESS_IDS_BY_PGID_HOOK = Some(list_process_ids_by_pgid::<R>);
            hooks::CURRENT_TASK_NAME_HOOK = Some(lifecycle::current_task_name_impl::<R>);
            hooks::TASK_EXEC_HOOK = Some(crate::task::exec::task_exec_current::<R>);
            hooks::SET_CURRENT_USER_FS_BASE_HOOK = Some(lifecycle::set_current_user_fs_base::<R>);
            hooks::CURRENT_USER_FS_BASE_HOOK = Some(lifecycle::current_user_fs_base::<R>);
            hooks::SET_CURRENT_TASK_NAME_HOOK = Some(lifecycle::set_current_task_name::<R>);
            hooks::WAITPID_HOOK = Some(waitpid::<R>);
            hooks::GET_SIGNAL_MASK_HOOK = Some(lifecycle::get_signal_mask::<R>);
            hooks::SET_SIGNAL_MASK_HOOK = Some(lifecycle::set_signal_mask::<R>);
            hooks::GET_THREAD_PENDING_HOOK = Some(lifecycle::get_thread_pending::<R>);
            hooks::SET_THREAD_PENDING_HOOK = Some(lifecycle::set_thread_pending::<R>);
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    }
    clear_sched_lock_tracking::<R>();
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();
    let cpu_total = rt.cpu_total_count();

    for cpu_id in 0..cpu_total {
        let cpu_sched = state::CpuScheduler::new_for_cpu(cpu_id);
        sched.state.per_cpu.push(cpu_sched);
    }

    sched.total_cpu_count = cpu_total;
    sched.state.set_boot_cpu_online();
    sched.bringup_in_progress = true;

    let layout = alloc::alloc::Layout::from_size_align(16384, 8).unwrap();
    let stack_base = unsafe { alloc::alloc::alloc(layout) };
    if stack_base.is_null() {
        panic!("Failed to allocate stack for boot task");
    }
    let stack_top = (stack_base as u64) + 16384;

    let task: Task<R> = Task {
        id: 0,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        kstack_base: stack_base,
        kstack_size: 16384,
        kstack_top: stack_top,
        ctx: Default::default(),
        aspace: rt.tasking().active_address_space(),
        simd: crate::simd::SimdState::new(rt),
        exit_code: None,
        exit_waiters: WaitQueue::new(),
        is_user: false,
        wake_pending: false,
        pending_interrupt: false,
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        affinity: crate::task::Affinity::Any,
        last_cpu: Some(0),
        name: {
            let mut n = [0u8; 32];
            n[0] = b'b'; n[1] = b'o'; n[2] = b'o'; n[3] = b't';
            n
        },
        name_len: 4,
        process_info: None,
        enqueued_at_tick: TICK_COUNT.load(Ordering::Relaxed),
        base_priority: TaskPriority::Normal,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    let sched_fields = bridge::TaskSchedCache::from_thread(&task)
        .with_wake_cpu(Some(0))
        .with_run_cpu(Some(0))
        .into_sched_fields(task.id);
    sched.state.insert_task(sched_fields);
    crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));

    sched.state.per_cpu[0].current = Some(0);
    set_cpu_current_task(0, 0);

    // Create idle task for CPU 0 initially
    {
        let i = 0;
        let idle_id = sched.spawn(
            lifecycle::idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        sched.state.remove_task_from_runq(idle_id);
        sched.state.per_cpu[i].idle_task = Some(idle_id);

        if let Some(mut t) = crate::task::registry::get_task_mut::<R>(idle_id) {
            t.affinity = crate::task::Affinity::Pinned(i);
        }
    }
}

/// Legacy pick-candidate comparison helper.
#[allow(dead_code)]
#[inline]
pub(crate) fn better_fair_pick_candidate(
    eff: usize,
    vruntime: u64,
    queue_idx: usize,
    best_eff: usize,
    best_vruntime: u64,
    best_q: Option<usize>,
) -> bool {
    if eff > best_eff {
        return true;
    }
    if eff != best_eff {
        return false;
    }
    if vruntime < best_vruntime {
        return true;
    }
    vruntime == best_vruntime && best_q.is_some_and(|best_queue_idx| queue_idx < best_queue_idx)
}

#[inline]
pub(super) fn sample_runq_len<R: BootRuntime>(sched: &types::Scheduler<R>, cpu_idx: usize) {
    let now = TICK_COUNT.load(Ordering::Relaxed);
    if now % 64 == 0 {
        if let Some(pc) = sched.state.per_cpu.get(cpu_idx) {
            let len = pc.runq.total_len() as u64;
            PROF_RUNQ_LEN_LAST[cpu_idx].store(len, Ordering::Relaxed);
            let old_max = PROF_RUNQ_LEN_MAX[cpu_idx].load(Ordering::Relaxed);
            if len > old_max {
                PROF_RUNQ_LEN_MAX[cpu_idx].store(len, Ordering::Relaxed);
            }
        }
    }
}

#[cfg(test)]
mod tests;

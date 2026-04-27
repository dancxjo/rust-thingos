use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

// ── IRQ balance regression tests ──────────────────────────────────────────

/// `block_current` must restore IRQ state on the early-return path that
/// triggers when there is no current task on the calling CPU (`current_id
/// == None`).  Previously this path returned without calling `irq_restore`,
/// leaving the CPU with interrupts permanently disabled.
#[test]
fn test_block_current_restores_irq_when_no_current_task() {
    let _g = init_test_env();
    reset_mock_irq_depth();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // CPU 0 with no current task.
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = None;

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    block_current::<MockRuntime>();

    // IRQ depth must be back to 0 — irq_disable was paired with irq_restore.
    assert_eq!(mock_irq_depth(), 0, "block_current left IRQs disabled (depth != 0)");

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_block_current_recovers_when_prepare_schedule_returns_none() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    reset_mock_irq_depth();

    let tid = 9901;
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(tid);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        crate::task::Task {
            id: tid,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        },
    ));

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    block_current::<MockRuntime>();

    assert_eq!(mock_irq_depth(), 0, "block_current left IRQs disabled (depth != 0)");
    assert!(
        !sched.state.wait_queue.contains(&tid),
        "block_current recovery must remove waiter membership when no switch occurs"
    );
    assert_eq!(
        sched.state.get_task(tid).map(|sf| sf.state),
        Some(TaskState::Running),
        "scheduler cache must restore the task to Running when no switch occurs"
    );
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(tid).unwrap().state,
        TaskState::Running,
        "registry state must be restored when no switch occurs"
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

/// `sleep_ticks` must restore IRQ state on the early-return path that
/// triggers when there is no current task on the calling CPU (`current_id
/// == None`).  Without the fix the `None` arm returned without calling
/// `irq_restore`, leaving interrupts disabled.
#[test]
fn test_sleep_ticks_restores_irq_when_no_current_task() {
    let _g = init_test_env();
    reset_mock_irq_depth();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // CPU 0 with no current task.
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = None;

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    sleep_ticks::<MockRuntime>(5);

    // IRQ depth must be back to 0 — irq_disable was paired with irq_restore.
    assert_eq!(mock_irq_depth(), 0, "sleep_ticks left IRQs disabled (depth != 0)");

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

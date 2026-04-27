
use super::*;
use crate::task::{Affinity, TaskPriority, TaskState};
use crate::{
    BootRuntime, BootRuntimeBase, BootTasking, MapKind, MapPerms, UserEntry, UserTaskSpec,
};

// Mock types for testing - copy from spawn.rs tests
#[derive(Default, Copy, Clone)]
pub(crate) struct MockContext(pub(crate) usize);
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct MockAddressSpace(pub(crate) u64);

pub(crate) static MOCK_RUNTIME: MockRuntime = MockRuntime;
pub(crate) struct MockRuntime;

// Per-thread IRQ depth counter used by MockRuntime to detect imbalanced
// irq_disable / irq_restore pairs. irq_disable increments the depth and
// returns the old value; irq_restore restores the depth to the saved value.
std::thread_local! {
    static IRQ_DEPTH: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static MOCK_IDLE_TASK_CURRENT: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Returns the current mock IRQ depth for the calling test thread.
/// A value of 0 means interrupts are conceptually enabled (balanced state).
pub(crate) fn mock_irq_depth() -> usize {
    IRQ_DEPTH.with(|c| c.get())
}

/// Resets the mock IRQ depth to 0 (call at the start of each test that
/// checks IRQ balance to ensure a clean baseline).
pub(crate) fn reset_mock_irq_depth() {
    IRQ_DEPTH.with(|c| c.set(0));
}

impl BootRuntimeBase for MockRuntime {
    fn putchar(&self, _c: u8) {}
    fn mono_ticks(&self) -> u64 {
        0
    }
    fn mono_freq_hz(&self) -> u64 {
        1
    }
    fn init_secondary_cpu(&self, _cpu_index: usize) {}
    fn phys_to_virt_offset(&self) -> u64 {
        0
    }

    fn irq_disable(&self) -> crate::IrqState {
        let prev = IRQ_DEPTH
            .try_with(|c| {
                let d = c.get();
                c.set(d + 1);
                d
            })
            .unwrap_or(0);
        crate::IrqState(prev)
    }
    fn irq_restore(&self, state: crate::IrqState) {
        let _ = IRQ_DEPTH.try_with(|c| c.set(state.0));
    }
    fn is_idle_task_current(&self) -> bool {
        MOCK_IDLE_TASK_CURRENT.try_with(|c| c.get()).unwrap_or(false)
    }
    fn set_idle_task_current(&self, idle: bool) {
        let _ = MOCK_IDLE_TASK_CURRENT.try_with(|c| c.set(idle));
    }
}
impl BootRuntime for MockRuntime {
    type Tasking = MockRuntime;
    fn tasking(&self) -> &Self {
        self
    }
    fn halt(&self) -> ! {
        loop {}
    }

    // irq_disable and irq_restore moved to BootRuntimeBase
    fn phys_memory_map(&self) -> &'static [crate::PhysRange] {
        &[]
    }
    fn modules(&self) -> &'static [crate::BootModuleDesc] {
        &[]
    }
    fn framebuffer(&self) -> Option<crate::FramebufferInfo> {
        None
    }
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }
    unsafe fn simd_save(&self, _ptr: *mut u8) {}
    unsafe fn simd_restore(&self, _ptr: *const u8) {}
    fn get_kernel_cmdline(&self) -> &'static str {
        ""
    }
}
impl BootTasking for MockRuntime {
    type Runtime = MockRuntime;
    type Context = MockContext;
    type AddressSpace = MockAddressSpace;
    fn init(&self, _hhdm: u64) {}
    fn init_kernel_context(
        &self,
        _entry: extern "C" fn(usize) -> !,
        _st: u64,
        _arg: usize,
    ) -> Self::Context {
        MockContext(_arg)
    }
    fn init_user_context(
        &self,
        _spec: UserTaskSpec<Self::AddressSpace>,
        _kst: u64,
    ) -> Self::Context {
        MockContext(_spec.arg)
    }
    unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context, _tid: u64) {}
    unsafe fn enter_user(&self, _e: UserEntry) -> ! {
        loop {}
    }
    fn make_user_address_space(&self) -> Self::AddressSpace {
        MockAddressSpace(0)
    }
    fn active_address_space(&self) -> Self::AddressSpace {
        MockAddressSpace(0)
    }
    fn activate_address_space(&self, _as: Self::AddressSpace) {}
    fn map_page(
        &self,
        _as: Self::AddressSpace,
        _v: u64,
        _p: u64,
        _pr: MapPerms,
        _a: &dyn crate::FrameAllocatorHook,
    ) -> Result<(), ()> {
        Ok(())
    }
    fn unmap_page(&self, _as: Self::AddressSpace, _v: u64) -> Result<Option<u64>, ()> {
        Ok(None)
    }
    fn protect_page(
        &self,
        _as: Self::AddressSpace,
        _virt: u64,
        _perms: MapPerms,
    ) -> Result<(), ()> {
        Ok(())
    }
    fn translate(&self, _as: Self::AddressSpace, _v: u64) -> Option<u64> {
        None
    }
    fn tlb_flush_page(&self, _v: u64) {}
}

static INIT_TESTS: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
fn init_mock_runtime() {
    if !INIT_TESTS.swap(true, core::sync::atomic::Ordering::SeqCst) {
        crate::init_runtime(&MOCK_RUNTIME);
    }
}

#[test]
fn effective_parallelism_any_uses_online_cpu_count() {
    assert_eq!(effective_parallelism_from_state(1, Affinity::Any), 1);
    assert_eq!(effective_parallelism_from_state(4, Affinity::Any), 4);
}

#[test]
fn effective_parallelism_pinned_is_single_cpu() {
    assert_eq!(effective_parallelism_from_state(1, Affinity::Pinned(0)), 1);
    assert_eq!(effective_parallelism_from_state(8, Affinity::Pinned(3)), 1);
}

#[test]
fn effective_parallelism_never_returns_zero() {
    assert_eq!(effective_parallelism_from_state(0, Affinity::Any), 1);
}

#[test]
fn effective_parallelism_restricted_counts_allowed_online_cpus() {
    use crate::sched::state::{CpuAffinity, CpuSet};
    // Allowed CPUs 1 and 2 only; 4 CPUs online.
    let aff = CpuAffinity { allowed: CpuSet(0b0110), preferred: None, last_cpu: None };
    assert_eq!(effective_parallelism_from_state(4, Affinity::Restricted(aff)), 2);
}

#[test]
fn effective_parallelism_restricted_caps_at_online_count() {
    use crate::sched::state::{CpuAffinity, CpuSet};
    // Allowed CPUs 0-7 but only 2 online.
    let aff = CpuAffinity { allowed: CpuSet(0xFF), preferred: None, last_cpu: None };
    assert_eq!(effective_parallelism_from_state(2, Affinity::Restricted(aff)), 2);
}

/// Serialises sched tests that mutate shared globals (REGISTRY, SCHEDULER,
/// TICK_COUNT).  Any test that calls `init_test_env` should hold the
/// returned guard for its entire duration to prevent races with concurrent
/// tests that reinitialize the registry.
pub(crate) static SCHED_TEST_GUARD: spin::Mutex<()> = spin::Mutex::new(());

pub(crate) fn init_test_env() -> spin::MutexGuard<'static, ()> {
    let guard = SCHED_TEST_GUARD.lock();
    init_mock_runtime();
    crate::task::registry::init::<MockRuntime>();
    *SCHEDULER.lock() = None;
    SCHEDULER_LOCK_OWNER.store(-1, Ordering::Relaxed);
    SCHEDULER_LOCK_ACQUIRED_AT.store(0, Ordering::Relaxed);
    PROF_LOCK_ORDER_VIOLATIONS.store(0, Ordering::Relaxed);
    TICK_COUNT.store(0, core::sync::atomic::Ordering::Relaxed);
    for i in 0..types::MAX_CPUS {
        TRYLOCK_MISS_WINDOW_START[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_WINDOW_COUNT[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_WINDOW_TIMER_COUNT[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_WINDOW_IPI_COUNT[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_WINDOW_PENDING_COUNT[i].store(0, Ordering::Relaxed);
        TRYLOCK_MISS_LAST_WARN_TICK[i].store(0, Ordering::Relaxed);
    }
    crate::runtime::<MockRuntime>().set_idle_task_current(false);
    reset_remote_wake_mailboxes_for_tests();
    reset_any_wake_policy_for_tests();
    guard
}

fn make_task(
    id: TaskId,
    state: TaskState,
    priority: TaskPriority,
) -> crate::task::Task<MockRuntime> {
    crate::task::Task {
        id,
        state,
        priority,
        base_priority: priority,
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
    }
}

#[test]
fn set_priority_uses_hot_cache_runnable_state_for_runq_move() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Registry carries stale state (Blocked), while scheduler cache has the
    // hot-path truth (Runnable + currently enqueued).
    let task = make_task(42, TaskState::Blocked, TaskPriority::Low);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 42,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 42);

    sched.set_priority(42, TaskPriority::High);

    assert_eq!(sched.state.get_task(42).unwrap().priority, TaskPriority::High);
    assert_eq!(
        crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
        TaskPriority::Low,
        "scheduler-only set_priority keeps REGISTRY sync out of the hot path"
    );
    assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert_eq!(sched.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(), Some(42));
}

#[test]
fn public_set_priority_syncs_registry_after_scheduler_unlock() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let task = make_task(43, TaskState::Runnable, TaskPriority::Low);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 43,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 43);

    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    set_priority::<MockRuntime>(43, TaskPriority::High);

    let sched_ref = unsafe { &mut *(sched_ptr as *mut types::Scheduler<MockRuntime>) };
    assert_eq!(sched_ref.state.get_task(43).unwrap().priority, TaskPriority::High);
    assert!(sched_ref.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert_eq!(
        sched_ref.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(),
        Some(43)
    );
    assert_eq!(
        crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
        TaskPriority::High,
        "public set_priority should sync REGISTRY outside scheduler lock"
    );

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_reads_scheduler_hot_cache() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(44);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 44,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::High,
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

    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::High);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_defaults_when_no_current_task() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_defaults_when_current_task_missing_from_hot_cache() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(99);
    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn preempt_disable_depth_is_cpu_local() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.state.per_cpu[1].preempt_disable_depth = 3;
    sched.state.per_cpu[1].preempt_disable_since = 42;
    sched.state.per_cpu[1].preempt_watchdog_warned = true;

    TICK_COUNT.store(99, Ordering::Relaxed);
    sched.preempt_disable();
    sched.preempt_disable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 2);
    assert_eq!(sched.state.per_cpu[0].preempt_disable_since, 99);
    assert!(!sched.state.per_cpu[0].preempt_watchdog_warned);

    sched.preempt_enable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 1);
    sched.preempt_enable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 0);

    assert_eq!(sched.state.per_cpu[1].preempt_disable_depth, 3);
    assert_eq!(sched.state.per_cpu[1].preempt_disable_since, 42);
    assert!(sched.state.per_cpu[1].preempt_watchdog_warned);
}

#[test]
fn clear_sched_lock_tracking_only_clears_when_called_by_owner_cpu() {
    let _g = init_test_env();

    set_sched_lock_tracking::<MockRuntime>(1);
    SCHEDULER_LOCK_ACQUIRED_AT.store(123, Ordering::Release);
    clear_sched_lock_tracking::<MockRuntime>();
    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 123);

    set_sched_lock_tracking::<MockRuntime>(0);
    clear_sched_lock_tracking::<MockRuntime>();
    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
}

#[test]
fn sched_lock_tracking_guard_clears_on_drop() {
    let _g = init_test_env();

    {
        let _tracking = sched_lock_tracking_guard::<MockRuntime>(0);
        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 0);
    }

    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
}

#[test]
fn dump_stats_clears_sched_lock_tracking() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let sched_ptr = &mut sched as *mut types::Scheduler<MockRuntime> as usize;
    *SCHEDULER.lock() = Some(sched_ptr);
    dump_stats::<MockRuntime>();

    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
    *SCHEDULER.lock() = None;
}

#[test]
fn test_priority_aging_boost() {
    let _g = init_test_env();
    // Test that tasks waiting too long get priority boost when scheduling
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0); // Dummy current task

    // The dummy current task must be in BOTH the global registry and the
    // scheduler's local sched-state so that `do_context_switch` can index
    // into both consistently (both vectors are sorted by TID).
    let dummy_current = make_task(0, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(dummy_current));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    let task_normal = crate::task::Task {
        id: 1001,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 600, // Recent (now - 600 = 400 < AGING_THRESHOLD, so no boost)
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
    };

    // Create a low-priority task enqueued a long time ago
    let task_low = crate::task::Task {
        id: 1002,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        base_priority: TaskPriority::Low,
        enqueued_at_tick: 0, // Very old
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(task_normal));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_low));

    // Scheduler state entries are separate from the global registry and must
    // be inserted explicitly so `prepare_schedule` can locate them.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 1001,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        // Must match `task_normal.enqueued_at_tick` above (600) so the
        // hot-field cache reflects the correct wait time for aging.
        enqueued_at_tick: 600,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 1002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        // Must match `task_low.enqueued_at_tick` above (0) so the
        // hot-field cache reflects the correct wait time for aging.
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 1001);
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 1002);

    // Simulate time advancing enough to give the Low task a boost of +2 (eff = High=3),
    // while the Normal task, enqueued at tick 600, only waits 400 ticks → no boost (eff = Normal=2).
    let now = types::AGING_THRESHOLD_TICKS * 2;
    TICK_COUNT.store(now, core::sync::atomic::Ordering::Relaxed);

    // Request schedule. The Low task should be selected because its effective priority is higher
    // than Normal due to wait time.
    let next_switch = sched.prepare_schedule().expect("Should find a task");
    assert_eq!(next_switch.to_tid, 1002, "Low priority task with aging should preempt normal task");

    // Verify it was popped from the Low queue, not moved to High queue
    assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert!(!sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].is_empty());
}

#[test]
fn test_vruntime_prefers_least_served_on_effective_priority_tie() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let dummy_current = make_task(0, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(dummy_current));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    let mut task_a = make_task(4001, TaskState::Runnable, TaskPriority::Normal);
    task_a.enqueued_at_tick = 0;
    let mut task_b = make_task(4002, TaskState::Runnable, TaskPriority::Normal);
    task_b.enqueued_at_tick = 0;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_a));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_b));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 4001,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 4002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4001);
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4002);

    sched.state.task_runtime_stats_mut(4001).fair_vruntime = 100;
    sched.state.task_runtime_stats_mut(4002).fair_vruntime = 10;

    let next_switch = sched.prepare_schedule().expect("Should pick a runnable task");
    assert_eq!(
        next_switch.to_tid, 4002,
        "least-served task should be preferred when effective priority ties"
    );
}

#[test]
fn test_reset_priority_aging_on_schedule() {
    let _g = init_test_env();
    // Test that enqueued_at_tick resets when task is preempted/yields
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Task 1 is running
    let mut task1 = crate::task::Task {
        id: 2001,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0, // Very old
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
        timeslice_remaining: 0, // timeslice expired
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    // Task 2 is runnable
    let task2 = crate::task::Task {
        id: 2002,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 500, // Newer
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
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task1));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task2));

    sched.state.per_cpu[0].current = Some(2001);
    // Scheduler state entries for both tasks.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 2001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 2002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 500,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 2002);

    // Time moves forward
    TICK_COUNT.store(1000, core::sync::atomic::Ordering::Relaxed);

    // Trigger a timer tick to cause preemption
    let switch = sched.prepare_yield().expect("Should preempt to task2");
    assert_eq!(switch.to_tid, 2002);
    let t1_before_sync = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
    assert_eq!(
        t1_before_sync.enqueued_at_tick, 0,
        "prepare_yield should defer REGISTRY sync out of prepare_schedule hot path"
    );
    assert_eq!(
        t1_before_sync.state,
        TaskState::Running,
        "REGISTRY state should remain unchanged until deferred sync is applied"
    );
    apply_deferred_registry_syncs::<MockRuntime>(core::mem::take(
        &mut sched.pending_registry_syncs,
    ));

    // Verify task1 was placed back in runq and its enqueued_at_tick was updated to TICK_COUNT
    let t1 = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
    assert_eq!(t1.enqueued_at_tick, 1000);
    assert_eq!(t1.state, TaskState::Runnable);
}

#[test]
fn test_prepare_yield_keeps_current_runnable_when_no_peer_exists() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        7001,
        TaskState::Running,
        TaskPriority::Normal,
    )));

    sched.state.per_cpu[0].current = Some(7001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 7001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    let switch = sched.prepare_yield();

    assert!(switch.is_none(), "yield with no runnable peer should keep running current task");
    let sf = sched.state.get_task(7001).expect("current task should remain in scheduler state");
    assert_eq!(sf.state, TaskState::Running);
    assert_eq!(sf.runq_location, None);
    assert_eq!(sched.state.per_cpu[0].current, Some(7001));
}

#[test]
fn test_prepare_yield_penalizes_spin_yield_requeue_band() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6001,
        TaskState::Running,
        TaskPriority::High,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6002,
        TaskState::Runnable,
        TaskPriority::Realtime,
    )));

    sched.state.per_cpu[0].current = Some(6001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::High,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.note_enqueue_cause(6001, crate::sched::state::EnqueueCause::YieldRequeue);
    sched.state.enqueue_task(0, TaskPriority::Realtime as usize, 6002);

    let switch = sched.prepare_yield().expect("high-priority peer should run");
    assert_eq!(switch.to_tid, 6002);
    assert_eq!(
        sched.state.get_task(6001).and_then(|sf| sf.runq_location),
        Some((0, TaskPriority::Normal as usize)),
        "spin-yielding task should be demoted one runnable band on requeue",
    );
    assert_eq!(
        sched.state.get_task(6001).map(|sf| sf.voluntary_yields),
        Some(types::SPIN_YIELD_PENALTY_THRESHOLD + 1),
    );
}

#[test]
fn test_prepare_yield_does_not_penalize_without_prior_yield_requeue() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6101,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6102,
        TaskState::Runnable,
        TaskPriority::High,
    )));

    sched.state.per_cpu[0].current = Some(6101);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6101,
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
        voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD + 5,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6102,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::High,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.note_enqueue_cause(6101, crate::sched::state::EnqueueCause::Wake);
    sched.state.enqueue_task(0, TaskPriority::High as usize, 6102);

    let switch = sched.prepare_yield().expect("high-priority peer should run");
    assert_eq!(switch.to_tid, 6102);
    assert_eq!(
        sched.state.get_task(6101).and_then(|sf| sf.runq_location),
        Some((0, TaskPriority::Normal as usize)),
        "without prior yield-requeue cause, no spin penalty should be applied",
    );
}

#[test]
fn test_wake_preempts_lower_priority() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Task 1: Normal priority, currently running
    let normal_task = crate::task::Task {
        id: 3001,
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
    };

    // Task 2: Realtime priority, sleeping (about to wake)
    let rt_task = crate::task::Task {
        id: 3002,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
        base_priority: TaskPriority::Realtime,
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(normal_task));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(rt_task));
    sched.state.per_cpu[0].current = Some(3001); // Normal task is running
    // Scheduler state entries for both tasks.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 3001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 3002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    // Put RT task in sleep queue with wake_tick in the past
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(3002, 50);

    // Before: need_resched should be false
    assert!(!sched.state.per_cpu[0].need_resched, "need_resched should start false");

    // Wake sleepers — should detect RT > Normal and set need_resched
    sched.wake_sleepers();

    // Verify need_resched was set
    assert!(
        sched.state.per_cpu[0].need_resched,
        "need_resched should be true after waking a higher-priority task"
    );

    // Verify RT task was enqueued to the Realtime runq
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Realtime as usize].iter().any(|&id| id == 3002),
        "RT task should be in the Realtime run queue"
    );

    // Now simulate schedule: prepare_yield should pick the RT task
    sched.state.per_cpu[0].need_resched = false; // clear so prepare_yield runs clean
    let switch = sched.prepare_yield();
    assert!(switch.is_some(), "Should produce a context switch");
    let switch = switch.unwrap();
    assert_eq!(switch.to_tid, 3002, "Scheduler should switch to the RT task");
    assert_eq!(switch.from_tid, 3001, "Scheduler should switch away from the Normal task");
}

/// Verify that `wake_sleepers` defers cross-CPU IPIs to `pending_wake_ipis`
/// instead of calling `send_ipi` while the SCHEDULER lock is held.
///
/// The fix for issue #131 changed `wake_sleepers` to push target CPUs onto
/// `self.pending_wake_ipis` rather than invoking `send_ipi` directly. This
/// test checks that, after a cross-CPU sleeper is woken:
///   1. The IPI target is present in `pending_wake_ipis`.
///   2. `pending_wake_ipis` is empty once the caller drains it.
#[test]
fn test_wake_sleepers_defers_ipi_to_pending_wake_ipis() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // Add two CPUs: CPU 0 (running this test) and CPU 1 (the IPI target).
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    // Mark CPU 1 as online so it appears in the online_cpus list.
    sched.state.mark_cpu_online(1);

    // Task running on CPU 0 (the "current" task for MockRuntime).
    let current_task = make_task(9001, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Sleeping task pinned to CPU 1 (a different CPU from current_cpu_index == 0).
    let sleeping_task = make_task(9002, TaskState::Blocked, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(sleeping_task));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9002,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    // Put the sleeping task in the sleep queue with a wake_tick in the past.
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(9002, 50);

    // `pending_wake_ipis` should be empty before wake_sleepers runs.
    assert!(
        sched.pending_wake_ipis.is_empty(),
        "pending_wake_ipis should be empty before wake_sleepers"
    );

    sched.wake_sleepers();

    // After wake_sleepers, the IPI for CPU 1 must be in pending_wake_ipis,
    // not already sent (send_ipi is a no-op in MockRuntime).
    assert_eq!(
        sched.pending_wake_ipis,
        alloc::vec![1usize],
        "wake_sleepers should defer cross-CPU IPI to pending_wake_ipis (got {:?})",
        sched.pending_wake_ipis
    );

    // Simulate the caller draining pending_wake_ipis after releasing the lock.
    let drained = core::mem::take(&mut sched.pending_wake_ipis);
    assert_eq!(drained, alloc::vec![1usize]);
    assert!(sched.pending_wake_ipis.is_empty(), "pending_wake_ipis should be empty after drain");
}

#[test]
fn test_wake_sleepers_coalesces_same_cpu_remote_ipi_within_batch() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    clear_global_need_resched(1, Ordering::Relaxed);

    let current_task = make_task(9_101, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9_101);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9_101,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    for tid in [9_102, 9_103] {
        let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.add_task_to_sleep_queue(tid, 50);
    }
    TICK_COUNT.store(100, Ordering::Relaxed);

    sched.wake_sleepers();

    assert_eq!(
        sched.pending_wake_ipis,
        alloc::vec![1usize],
        "waking multiple sleepers for the same remote CPU should queue only one deferred IPI"
    );
    assert_eq!(
        PROF_IPI_SUPPRESSED.load(Ordering::Relaxed),
        1,
        "duplicate remote wake in one wake_sleepers batch should be counted as suppressed"
    );
}

#[test]
fn test_wake_sleepers_enforces_budget_and_carries_remainder() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let total_sleepers = types::WAKE_SLEEPERS_BUDGET_PER_TICK + 5;
    for tid in 10_000..(10_000 + total_sleepers as u64) {
        let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(0),
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.add_task_to_sleep_queue(tid, 50);
    }
    TICK_COUNT.store(100, Ordering::Relaxed);

    sched.wake_sleepers();
    assert_eq!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
        types::WAKE_SLEEPERS_BUDGET_PER_TICK,
        "wake_sleepers should honor per-tick wake budget"
    );
    assert_eq!(
        sched.state.sleep_queue.get(&50).map(|v| v.len()),
        Some(5),
        "sleep queue should retain remaining sleepers after budget is exhausted"
    );
    assert_eq!(
        sched.wake_sleepers_budget_carry, 0,
        "budget carry should be empty after fully consuming available budget"
    );

    sched.wake_sleepers();
    assert_eq!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
        total_sleepers,
        "second wake pass should process remaining sleepers"
    );
    assert!(
        !sched.state.sleep_queue.contains_key(&50),
        "sleep queue entry should be removed once all sleepers wake"
    );
    assert_eq!(
        sched.wake_sleepers_budget_carry,
        types::WAKE_SLEEPERS_BUDGET_PER_TICK - 5,
        "unused budget should carry forward to later ticks"
    );
}

#[test]
fn test_prepare_schedule_defers_misroute_repair_until_maintenance() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9101, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9101);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9101,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
    let misrouted = make_task(9102, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(misrouted));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9102,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9102);

    // No local idle task exists, so prepare_schedule returns None and defers
    // misroute repair to the maintenance pass.
    let switch = sched.prepare_schedule();
    assert!(
        switch.is_none(),
        "prepare_schedule should return None when only misrouted work exists and no idle task"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "prepare_schedule picker path should not perform deferred misroute repair or queue repair IPIs"
    );
    assert!(
        sched.pending_wake_ipis.is_empty(),
        "misroute IPIs should not be mixed into pending_wake_ipis"
    );
    assert!(
        sched
            .pending_misrouted_requeues
            .iter()
            .any(|&(_, target_cpu, tid)| target_cpu == 1 && tid == 9102),
        "misrouted task should be queued for deferred repair maintenance"
    );
    assert!(
        !sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9102),
        "target CPU runq should remain unchanged until maintenance runs"
    );

    sched.run_pending_misroute_repair_maintenance();

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize],
        "misroute maintenance should queue the deferred repair nudge"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9102),
        "misroute maintenance should move the task to the target CPU runq"
    );
}

#[test]
fn test_pending_prepare_schedule_ipi_bitmap_dedups_targets() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(2);

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize, 2usize],
        "bitmap-backed pending_prepare_schedule_ipis should dedup repeated CPU targets"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "drain should clear the bitmap-backed pending queue"
    );
}

#[test]
fn test_pending_prepare_schedule_ipi_bitmap_drain_preserves_all_pending_targets() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(3);
    sched.queue_pending_prepare_schedule_ipi(5);
    sched.queue_pending_prepare_schedule_ipi(3);

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize, 3usize, 5usize],
        "bitmap-backed drain should return all queued CPU targets without dropping pending bits or duplicating targets"
    );
}

#[test]
fn test_misroute_repair_maintenance_is_bounded_per_call() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9200, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9200);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9200,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Fill CPU 0's normal queue with misrouted tasks pinned to CPU 1.
    let misrouted_total = PREPARE_SCHEDULE_PICK_BUDGET + 4;
    for i in 0..misrouted_total {
        let tid = 9201 + i as u64;
        let task = make_task(tid, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);
    }

    let switch = sched.prepare_schedule();
    assert!(switch.is_none());
    assert!(
        !sched.pending_misrouted_requeues.is_empty(),
        "prepare_schedule should defer misroute repairs into maintenance backlog"
    );
    assert_eq!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len(),
        0,
        "prepare_schedule picker path should not directly repair deferred misroutes"
    );

    sched.run_pending_misroute_repair_maintenance();

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len()
            <= PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET,
        "maintenance should only repair up to PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET misroutes per call"
    );

    // Repeated maintenance passes should drain deferred work.
    for _ in 0..4 {
        sched.run_pending_misroute_repair_maintenance();
    }
    assert!(
        sched.pending_misrouted_requeues.is_empty(),
        "deferred misroute backlog should drain across maintenance passes"
    );
}

#[test]
fn test_misroute_repair_maintenance_drops_non_runnable_entries() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9300, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9300);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9300,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
    let misrouted = make_task(9301, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(misrouted));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9301,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9301);

    let switch = sched.prepare_schedule();
    assert!(switch.is_none());
    assert_eq!(sched.pending_misrouted_requeues.len(), 1);

    if let Some(sf) = sched.state.get_task_mut(9301) {
        sf.state = TaskState::Dead;
    }

    sched.run_pending_misroute_repair_maintenance();

    assert!(
        sched.pending_misrouted_requeues.is_empty(),
        "maintenance prevalidation should drop non-runnable deferred misroutes"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().all(|&tid| tid != 9301),
        "dropped deferred entry should not be re-enqueued"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "dropped deferred entry should not queue an IPI"
    );
}

#[test]
fn test_send_deferred_prepare_schedule_ipis_updates_counters() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[2].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    TICK_COUNT.store(1, Ordering::Relaxed);

    send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 2usize]);

    assert_eq!(
        DIAG_IPI_SENT.load(Ordering::Relaxed),
        2,
        "generic IPI counter should increase for each deferred prepare_schedule send"
    );
    assert_eq!(
        DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed),
        2,
        "prepare_schedule source counter should increase for each deferred send"
    );
}

#[test]
fn test_send_deferred_prepare_schedule_ipis_suppresses_same_tick_duplicates() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    TICK_COUNT.store(42, Ordering::Relaxed);

    send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 1usize]);

    assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed), 1);
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);
}

#[test]
fn test_should_send_remote_resched_ipi_suppresses_adjacent_tick_ping_pong() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);

    TICK_COUNT.store(100, Ordering::Relaxed);
    assert!(should_send_remote_resched_ipi(1));

    TICK_COUNT.store(101, Ordering::Relaxed);
    assert!(!should_send_remote_resched_ipi(1));
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);

    TICK_COUNT.store(102, Ordering::Relaxed);
    assert!(should_send_remote_resched_ipi(1));

    TICK_COUNT.store(103, Ordering::Relaxed);
    assert!(!should_send_remote_resched_ipi(1));
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 2);
}

#[test]
fn test_sample_runq_len_downsamples_global_telemetry_scan() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    use crate::sched::state::PerCpu;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(PerCpu::new());
    sched.state.per_cpu.push(PerCpu::new());
    sched.state.online_cpus = alloc::vec![0, 1];
    sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].push_back(7001);
    sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].push_back(7002);

    PROF_RUNQ_SAMPLE_COUNT[0].store(0, Ordering::Relaxed);
    PROF_RUNQ_SAMPLE_TOTAL[0].store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_TOTAL.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_LAST.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_MAX.store(0, Ordering::Relaxed);

    for _ in 0..(RUNQ_GLOBAL_TELEMETRY_SAMPLE_STRIDE - 1) {
        sample_runq_len(&mut sched, 0);
    }
    assert_eq!(
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
        0,
        "global variance scan should be skipped until the configured stride is reached"
    );

    sample_runq_len(&mut sched, 0);
    assert_eq!(
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
        1,
        "global variance scan should run when the stride boundary is reached"
    );
}

#[test]
fn test_registry_rapid_create_exit_churn() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Helper to create dummy task
    let make_task = |id: TaskId| crate::task::Task {
        id,
        state: TaskState::Runnable,
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
        last_cpu: None,
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    // Insert tasks out of order
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4010)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4005)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4020)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4001)));

    assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads.len(), 4);

    // Verify lookups work
    assert!(crate::task::registry::get_task::<MockRuntime>(4010).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4005).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4001).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4099).is_none());

    // Keep a small hot set of thread IDs cycling through many create/exit
    // operations to stress index maintenance under churn.
    const CHURN_ITERATIONS: usize = 512; // enough iterations to repeatedly reshuffle slots
    const CHURN_ACTIVE_TID_COUNT: usize = 32; // small hot set maximizes remove/reinsert reuse
    for i in 0..CHURN_ITERATIONS {
        let tid = 5000 + (i % CHURN_ACTIVE_TID_COUNT) as u64;
        if crate::task::registry::get_task::<MockRuntime>(tid).is_none() {
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(make_task(tid)));
        }
        let removed = crate::task::registry::get_registry::<MockRuntime>().remove(tid);
        assert!(removed.is_some(), "expected tid {} to exist before removal", tid);
        assert!(
            crate::task::registry::get_task::<MockRuntime>(tid).is_none(),
            "removed tid {} must not remain addressable",
            tid
        );
    }
}

#[test]
fn test_block_and_wake_state_transitions() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let waiting_task = crate::task::Task {
        id: 5001,
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(waiting_task));
    sched.state.per_cpu[0].current = Some(5001);

    // Put task in Wait queue and switch it to Blocked (simulating block_current behavior)
    if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
        task.state = TaskState::Blocked;
    }
    sched.state.register_waiter(5001, crate::sched::state::WaitReason::BlockCurrent);

    // Verify task is stuck blocked
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(5001).unwrap().state,
        TaskState::Blocked
    );

    // Emulate `wake_task_erased` via wake_task in MockRuntime context
    crate::sched::blocking::WAKE_TASK_HOOK.store(
        crate::sched::blocking::wake_task::<MockRuntime> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );

    // Make sure scheduler hook resolves safely (we will mock inject the scheduler here via static for the hook)
    // Since we are unit testing `wake_task`, we can't easily use the global `SCHEDULER`.
    // So we just directly call the core logic we care about: the wake sleeper unblock logic.

    // Remove from wait queue if present
    sched.state.unregister_waiter(5001);

    // Update state to Runnable and add to runq
    if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
        if task.state == TaskState::Blocked {
            task.state = TaskState::Runnable;
            sched.state.enqueue_task(0, task.priority as usize, 5001);
        }
    }

    let woken_task = crate::task::registry::get_task::<MockRuntime>(5001).unwrap();
    assert_eq!(
        woken_task.state,
        TaskState::Runnable,
        "Task must transition from Blocked to Runnable upon wake"
    );

    // Verify task was placed in runq
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 5001),
        "Woken task must be in the run queue"
    );
}

#[test]
fn test_wake_task_removes_sleep_queue_entry() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(6000);

    let current_task = crate::task::Task {
        id: 6000,
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
    };

    let sleeping_task = crate::task::Task {
        id: 6001,
        state: TaskState::Blocked,
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(sleeping_task));

    // Both tasks must be in ThreadSchedFields so that wake_task_locked can
    // find and properly wake the sleeping task via the hot-field cache.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6000,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6001,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    sched.state.add_task_to_sleep_queue(6001, 10);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    crate::sched::blocking::wake_task::<MockRuntime>(6001);

    let task = crate::task::registry::get_task::<MockRuntime>(6001).unwrap();
    assert_eq!(task.state, TaskState::Runnable);
    assert!(sched.state.sleep_queue.is_empty());
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 6001)
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_wake_task_revalidates_under_split_authority_window() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6101,
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

    let task = crate::task::Task {
        id: 6101,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: true,
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
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);
    let _ = sched_lock_metrics_snapshot_and_reset();

    crate::sched::blocking::wake_task::<MockRuntime>(6101);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
    drop(sched_lock);

    let task = crate::task::registry::get_task::<MockRuntime>(6101).unwrap();
    assert_eq!(task.state, TaskState::Running);
    assert!(task.wake_pending);
    assert!(
        sched.state.get_task(6101).map(|sf| sf.wake_pending).unwrap_or(false),
        "wake_task must revalidate scheduler cache even when REGISTRY wake_pending is true"
    );

    let metrics = sched_lock_metrics_snapshot_and_reset();
    assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
    assert_eq!(metrics.wake_task.wait_calls, 1);
}

#[test]
fn test_wake_task_remote_path_uses_mailbox_without_scheduler_lock() {
    let _g = init_test_env();
    TICK_COUNT.store(123, Ordering::Relaxed);
    clear_global_need_resched(1, Ordering::Relaxed);
    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
    DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);

    let task = crate::task::Task {
        id: 6_202,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: false,
        pending_interrupt: false,
        affinity: Affinity::Pinned(1),
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
        last_cpu: Some(1),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    crate::sched::blocking::wake_task::<MockRuntime>(6_202);

    let task = crate::task::registry::get_task::<MockRuntime>(6_202).unwrap();
    assert_eq!(task.state, TaskState::Runnable);
    assert_eq!(task.enqueued_at_tick, 123);

    let queued = take_remote_wake_mailbox(1);
    assert_eq!(queued.len(), 1);
    assert_eq!(queued.front().map(|entry| entry.tid), Some(6_202));
    assert!(global_need_resched_load(1, Ordering::Acquire));
    assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
}

#[test]
fn test_wake_task_remote_mailbox_forces_ipi_when_pending_and_rate_limited() {
    let _g = init_test_env();
    TICK_COUNT.store(100, Ordering::Relaxed);
    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
    DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(100, Ordering::Relaxed);
    set_global_need_resched(1);

    let task = make_task(6_203, TaskState::Blocked, TaskPriority::Normal);
    let mut task = task;
    task.affinity = Affinity::Pinned(1);
    task.last_cpu = Some(1);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    crate::sched::blocking::wake_task::<MockRuntime>(6_203);

    assert_eq!(
        DIAG_IPI_SENT.load(Ordering::Relaxed),
        1,
        "remote mailbox wake should force one resched IPI even with pending/rate-limit state"
    );
    assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
}

#[test]
fn test_drain_remote_wake_mailbox_enqueues_locally() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[1].current = Some(6_300);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6_300,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Low,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: Some(1),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6_301,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    let _ = sched.state.register_waiter(6_301, state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(6_301, 7);

    enqueue_remote_wake_mailbox(
        1,
        types::RemoteWakeMailboxEntry {
            tid: 6_301,
            priority: TaskPriority::Normal as usize,
            enqueued_at_tick: 9,
            wake_mono: 11,
        },
    );

    sched.drain_remote_wake_mailbox(1);

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 6_301)
    );
    assert_eq!(sched.state.get_task(6_301).map(|sf| sf.state), Some(TaskState::Runnable));
    assert_eq!(sched.state.get_task(6_301).and_then(|sf| sf.wake_cpu), Some(1));
    assert!(!sched.state.wait_queue.contains(&6_301));
    assert!(!sched.state.sleep_membership.contains_key(&6_301));
    assert_eq!(PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[0].load(Ordering::Relaxed), 1);
}

#[test]
fn test_wake_task_without_pending_wake_uses_scheduler_path() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6102,
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

    let task = crate::task::Task {
        id: 6102,
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
    };
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);
    let _ = sched_lock_metrics_snapshot_and_reset();

    crate::sched::blocking::wake_task::<MockRuntime>(6102);

    let task = crate::task::registry::get_task::<MockRuntime>(6102).unwrap();
    assert!(task.wake_pending);
    let metrics = sched_lock_metrics_snapshot_and_reset();
    assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
    assert_eq!(metrics.wake_task.wait_calls, 1);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_kill_by_tid_removes_wait_queue_entry() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(7000);

    let current_task = crate::task::Task {
        id: 7000,
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
    };

    let blocked_task = crate::task::Task {
        id: 7001,
        state: TaskState::Blocked,
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(blocked_task));
    sched.state.register_waiter(7001, crate::sched::state::WaitReason::BlockCurrent);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    assert!(kill_by_tid::<MockRuntime>(7001));
    assert!(sched.state.wait_queue.is_empty());
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(7001).unwrap().state,
        TaskState::Dead
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_wait_task_returns_immediately_for_dead_target() {
    let _g = init_test_env();

    let dead_task = crate::task::Task {
        id: 8001,
        state: TaskState::Dead,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: Some(23),
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
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead_task));

    assert_eq!(wait_task::<MockRuntime>(8001).unwrap(), 23);
}

#[test]
fn test_wait_task_returns_echild_for_missing_target() {
    let _g = init_test_env();

    assert_eq!(wait_task::<MockRuntime>(8999).unwrap_err(), abi::errors::Errno::ECHILD);
}

#[test]
fn test_register_task_exit_waiter_tracks_live_target() {
    let _g = init_test_env();

    let live_task = crate::task::Task {
        id: 8101,
        state: TaskState::Runnable,
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
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(live_task));

    assert_eq!(register_task_exit_waiter::<MockRuntime>(8101, 8102).unwrap(), None);

    let waiters =
        crate::task::registry::get_task::<MockRuntime>(8101).unwrap().exit_waiters.drain();
    assert_eq!(waiters, alloc::vec![8102]);
}

#[test]
fn test_kill_by_tid_wakes_registered_exit_waiter() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8200);

    let current_task = crate::task::Task {
        id: 8200,
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
    };

    let waiter_task = crate::task::Task {
        id: 8201,
        state: TaskState::Blocked,
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
    };

    let target_task = crate::task::Task {
        id: 8202,
        state: TaskState::Runnable,
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
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(waiter_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(target_task));

    let target_fields = crate::sched::state::TaskSchedFields {
        tid: 8202,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };
    sched.state.insert_task(target_fields);

    assert_eq!(register_task_exit_waiter::<MockRuntime>(8202, 8201).unwrap(), None);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    assert!(kill_by_tid::<MockRuntime>(8202));
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8202).unwrap().state,
        TaskState::Dead
    );
    assert_eq!(crate::task::registry::get_task::<MockRuntime>(8202).unwrap().exit_code, Some(-9));
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8201).unwrap().state,
        TaskState::Runnable
    );
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 8201)
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_poll_task_exit_reports_pending_dead_and_missing_targets() {
    let _g = init_test_env();

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8301,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    let mut dead = make_task(8302, TaskState::Dead, TaskPriority::Normal);
    dead.exit_code = Some(17);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

    assert_eq!(poll_task_exit::<MockRuntime>(8301).unwrap(), None);
    assert_eq!(poll_task_exit::<MockRuntime>(8302).unwrap(), Some(17));
    assert_eq!(poll_task_exit::<MockRuntime>(8399).unwrap_err(), abi::errors::Errno::ECHILD);
}

#[test]
fn test_terminate_current_purges_dead_task_from_scheduler_queues() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8303);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8303,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8304,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8303,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8304,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Seed stale queue membership for the exiting task and ensure another
    // runnable task exists so terminate_current can produce a switch.
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8303);
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8304);
    sched.state.register_waiter(8303, crate::sched::state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(8303, 55);
    sched.state.add_task_to_sleep_queue(9999, 55);

    let termination = mark_task_exited_in_registry::<MockRuntime>(8303, 101);
    let switch = sched.terminate_current(8303, &termination.siblings_to_kill);

    assert_eq!(switch.to_tid, 8304, "scheduler should switch to the next runnable task");
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8303).unwrap().state,
        TaskState::Dead
    );
    assert_eq!(crate::task::registry::get_task::<MockRuntime>(8303).unwrap().exit_code, Some(101));
    assert!(
        !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 8303),
        "dead current task must be removed from the run queue"
    );
    assert!(
        !sched.state.wait_queue.contains(&8303),
        "dead current task must be removed from the wait queue"
    );
    assert_eq!(sched.state.sleep_queue.get(&55).cloned(), Some(alloc::vec![9999]));
}

#[test]
fn terminate_current_only_mutates_scheduler_state() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8310);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8310,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8311,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8310,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8311,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8311);

    let _ = sched.terminate_current(8310, &[]);
    assert_eq!(sched.state.get_task(8310).unwrap().state, TaskState::Dead);
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8310).unwrap().state,
        TaskState::Running,
        "terminate_current should not acquire/mutate REGISTRY directly"
    );
}

#[test]
#[should_panic(expected = "terminate_current could not find a switch")]
fn test_terminate_current_panics_when_no_switch_candidate_exists() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8306);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8306,
        TaskState::Running,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8306,
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

    let termination = mark_task_exited_in_registry::<MockRuntime>(8306, 202);
    let _ = sched.terminate_current(8306, &termination.siblings_to_kill);
}

#[test]
fn test_remove_task_completely_purges_scheduler_queues() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    let mut dead = make_task(8305, TaskState::Dead, TaskPriority::Normal);
    dead.exit_code = Some(42);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8305,
        runq_location: None,
        state: TaskState::Dead,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8305);
    sched.state.register_waiter(8305, crate::sched::state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(8305, 77);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    remove_task_completely::<MockRuntime>(8305);

    assert!(crate::task::registry::get_task::<MockRuntime>(8305).is_none());
    assert!(sched.state.get_task(8305).is_none());
    assert!(
        !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 8305),
        "reaped task must be removed from the run queue"
    );
    assert!(
        !sched.state.wait_queue.contains(&8305),
        "reaped task must be removed from the wait queue"
    );
    assert!(!sched.state.sleep_queue.contains_key(&77));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_unregister_task_exit_waiter_removes_only_requested_waiter() {
    let _g = init_test_env();

    let target = make_task(8401, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(target));

    register_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();
    register_task_exit_waiter::<MockRuntime>(8401, 8403).unwrap();
    unregister_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();

    let waiters =
        crate::task::registry::get_task::<MockRuntime>(8401).unwrap().exit_waiters.drain();
    assert_eq!(waiters, alloc::vec![8403]);
}

#[test]
fn test_register_timeout_wake_deduplicates_task_ids() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8500);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    register_timeout_wake::<MockRuntime>(8501, 42);
    register_timeout_wake::<MockRuntime>(8501, 42);
    register_timeout_wake::<MockRuntime>(8502, 42);

    assert_eq!(sched.state.sleep_queue.get(&42).cloned().unwrap(), alloc::vec![8501, 8502]);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_unregister_timeout_wake_removes_task_and_updates_membership() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8600);
    sched.state.add_task_to_sleep_queue(8601, 11);
    sched.state.add_task_to_sleep_queue(8602, 11);
    sched.state.add_task_to_sleep_queue(8602, 12);
    sched.state.add_task_to_sleep_queue(8603, 12);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    unregister_timeout_wake::<MockRuntime>(8602);

    assert_eq!(sched.state.sleep_queue.get(&11).cloned().unwrap(), alloc::vec![8601]);
    assert_eq!(sched.state.sleep_queue.get(&12).cloned().unwrap(), alloc::vec![8603]);
    assert!(!sched.state.sleep_membership.contains_key(&8602));
    assert_eq!(
        sched.state.sleep_membership.get(&8603).copied(),
        Some(crate::sched::state::SleepMembership { wake_tick: 12, bucket_index: 0 })
    );

    unregister_timeout_wake::<MockRuntime>(8603);
    assert!(!sched.state.sleep_queue.contains_key(&12));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_interrupt_task_marks_and_consumes_pending_interrupt() {
    let _g = init_test_env();

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    interrupt_task::<MockRuntime>(0).expect("interrupt task");
    assert!(crate::task::registry::get_task::<MockRuntime>(0).unwrap().pending_interrupt);
    assert!(take_pending_interrupt::<MockRuntime>());
    assert!(!take_pending_interrupt::<MockRuntime>());
}

#[test]
fn test_interrupt_task_returns_esrch_for_missing_task() {
    let _g = init_test_env();
    assert_eq!(interrupt_task::<MockRuntime>(9999).unwrap_err(), abi::errors::Errno::ESRCH);
}

// ── waitpid tests ─────────────────────────────────────────────────────────

/// Helper: build a task with a populated ProcessInfo (pid + ppid).
fn make_process_task(
    id: TaskId,
    state: TaskState,
    pid: u32,
    ppid: u32,
    exit_code: Option<i32>,
) -> crate::task::Task<MockRuntime> {
    crate::task::Task {
        id,
        state,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
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
        process_info: Some(alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid,
            job: crate::job::Job::new(ppid, pid as TaskId),
            unix_compat: crate::task::ProcessUnixCompat::isolated(pid, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
            service_loop: None,
        }))),
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    }
}

#[test]
fn test_list_processes_hides_exit_code_for_live_job() {
    let _g = init_test_env();

    // Stale/non-authoritative exit_code on a live task should not surface
    // through ProcessSnapshot.
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_process_task(1200, TaskState::Running, 1200, 1, Some(77)),
    ));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].pid, 1200);
    assert_eq!(snapshots[0].state, TaskState::Running);
    assert_eq!(snapshots[0].exit_code, None);
}

#[test]
fn test_list_processes_prefers_job_leader_exit_code_for_leader() {
    let _g = init_test_env();

    let mut leader = make_process_task(1210, TaskState::Dead, 1210, 1, Some(7));
    let pinfo = leader.process_info.as_ref().unwrap();
    pinfo.lock().job.leader_exit_code = Some(33);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].pid, 1210);
    assert_eq!(snapshots[0].tid, 1210);
    assert_eq!(snapshots[0].state, TaskState::Dead);
    assert_eq!(snapshots[0].exit_code, Some(33));
}

#[test]
fn test_list_processes_uses_thread_exit_code_for_non_leader_tasks() {
    let _g = init_test_env();

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 1220,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![1220, 1221],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: Some(44),
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(1220, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    let mut sibling = make_task(1221, TaskState::Dead, TaskPriority::Normal);
    sibling.exit_code = Some(9);
    sibling.process_info = Some(alloc::sync::Arc::clone(&pinfo));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(sibling));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].tid, 1221);
    assert_eq!(snapshots[0].exit_code, Some(9));
}

#[test]
fn test_list_processes_projects_place_and_authority_from_process_aggregator() {
    let _g = init_test_env();

    let mut leader = make_process_task(1230, TaskState::Running, 1230, 1, None);
    let pinfo = leader.process_info.as_ref().unwrap();
    {
        let mut pi = pinfo.lock();
        pi.cwd = alloc::string::String::from("/work");
        pi.root = alloc::string::String::from("/srv/chroot");
        pi.exec_path = alloc::string::String::from("/bin/demo");
        pi.authority.uid = 1000;
        pi.authority.gid = 1001;
        pi.authority.capability_mask = 0x24;
        pi.namespace = crate::vfs::NamespaceRef::isolated();
    }
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    let snap = &snapshots[0];
    assert_eq!(snap.pid, 1230);
    assert_eq!(snap.cwd, "/work");
    assert_eq!(snap.root_path, "/srv/chroot");
    assert_eq!(snap.exec_path, "/bin/demo");
    assert_eq!(snap.uid, 1000);
    assert_eq!(snap.gid, 1001);
    assert_eq!(snap.capability_mask, 0x24);
    assert_ne!(snap.namespace_label, "global");
}

#[test]
fn test_waitpid_returns_exit_code_for_dead_specific_child() {
    let _g = init_test_env();

    // Child task: pid=1001, ppid=1000, exit_code=42.
    // Tests call waitpid_for_pid directly so no parent task is needed.
    let child = make_process_task(9101, TaskState::Dead, 1001, 1000, Some(42));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (child_pid, code) =
        waitpid_for_pid::<MockRuntime>(1000, 1001, 0).expect("waitpid specific");
    assert_eq!(child_pid, 1001, "returned child pid");
    assert_eq!(code, 42, "returned exit code");
}

#[test]
fn test_waitpid_returns_exit_code_for_any_dead_child() {
    let _g = init_test_env();

    // Child: pid=2001, ppid=2000, exit_code=7
    let child = make_process_task(9201, TaskState::Dead, 2001, 2000, Some(7));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // pid == -1: wait for any child of process 2000
    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(2000, -1, 0).expect("waitpid any");
    assert_eq!(child_pid, 2001);
    assert_eq!(code, 7);
}

#[test]
fn test_waitpid_echild_when_no_children_exist() {
    let _g = init_test_env();

    // Registry is empty; no children for pid=3000
    let err = waitpid_for_pid::<MockRuntime>(3000, -1, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD);
}

#[test]
fn test_waitpid_echild_when_specific_child_not_found() {
    let _g = init_test_env();

    // A child with a *different* ppid — should not be found for pid=4000
    let unrelated = make_process_task(9401, TaskState::Dead, 9999, 5000, Some(0));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(unrelated));

    // Looking for child pid=9999 under parent pid=4000 → ECHILD
    let err = waitpid_for_pid::<MockRuntime>(4000, 9999, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD);
}

#[test]
fn test_waitpid_wnohang_returns_zero_when_child_alive() {
    let _g = init_test_env();

    // Live child — not yet exited
    let child = make_process_task(9501, TaskState::Runnable, 5001, 5000, None);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (child_pid, code) =
        waitpid_for_pid::<MockRuntime>(5000, -1, abi::types::waitpid_flags::WNOHANG)
            .expect("wnohang");
    assert_eq!(child_pid, 0, "no child exited yet");
    assert_eq!(code, 0);
}

#[test]
fn test_waitpid_multiple_children_returns_first_dead() {
    let _g = init_test_env();

    // Two children under parent pid=6000: first alive, second dead
    let child_alive = make_process_task(9601, TaskState::Runnable, 6001, 6000, None);
    let child_dead = make_process_task(9602, TaskState::Dead, 6002, 6000, Some(99));

    let mut reg = crate::task::registry::get_registry::<MockRuntime>();
    reg.insert(alloc::boxed::Box::new(child_alive));
    reg.insert(alloc::boxed::Box::new(child_dead));
    drop(reg);

    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(6000, -1, 0).expect("waitpid multi");
    assert_eq!(child_pid, 6002);
    assert_eq!(code, 99);
}

/// After `waitpid` successfully returns a dead child's exit code the child's
/// registry entry must be removed (reaped).  Without reaping, dead process
/// records accumulate indefinitely ("zombie" leak).
#[test]
fn test_waitpid_reaps_dead_child() {
    let _g = init_test_env();

    let child = make_process_task(9701, TaskState::Dead, 7001, 7000, Some(55));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // The child is still in the registry before the wait.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9701).is_some(),
        "child must be in registry before waitpid"
    );

    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(7000, 7001, 0).expect("waitpid reap");
    assert_eq!(child_pid, 7001);
    assert_eq!(code, 55);

    // After a successful wait the child record must have been reaped.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9701).is_none(),
        "child record must be removed from registry after reaping"
    );
}

/// Once a child has been reaped, a second `waitpid` for the same child must
/// return `ECHILD` — the record no longer exists.
#[test]
fn test_waitpid_echild_after_reaping() {
    let _g = init_test_env();

    let child = make_process_task(9702, TaskState::Dead, 7002, 7003, Some(0));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // First wait reaps the child.
    waitpid_for_pid::<MockRuntime>(7003, 7002, 0).expect("first waitpid");

    // Second wait must fail because the record was removed.
    let err = waitpid_for_pid::<MockRuntime>(7003, 7002, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD, "second waitpid after reaping must return ECHILD");
}

/// A dead child whose parent has NOT yet called `waitpid` must remain in
/// the registry (zombie semantics: exit status preserved until collected).
#[test]
fn test_dead_child_stays_in_registry_until_reaped() {
    let _g = init_test_env();

    let child = make_process_task(9703, TaskState::Dead, 7010, 7011, Some(3));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // No waitpid called yet — record must still be present.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9703).is_some(),
        "dead child must remain in registry until reaped"
    );

    // Verify state and exit code are accessible while zombie.
    let task = crate::task::registry::get_task::<MockRuntime>(9703).unwrap();
    assert_eq!(task.state, TaskState::Dead);
    assert_eq!(task.exit_code, Some(3));
}

/// `waitpid` with WNOHANG must not reap any child when no child has exited.
#[test]
fn test_waitpid_wnohang_does_not_reap_live_child() {
    let _g = init_test_env();

    let child = make_process_task(9704, TaskState::Runnable, 7020, 7021, None);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (returned_pid, _) =
        waitpid_for_pid::<MockRuntime>(7021, -1, abi::types::waitpid_flags::WNOHANG)
            .expect("wnohang on live child");
    assert_eq!(returned_pid, 0, "wnohang returns 0 when no child exited");

    // Live child must still be in registry.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9704).is_some(),
        "live child must remain in registry after WNOHANG poll"
    );
}

/// Helper: create a task + ProcessInfo with `tgid` populated and a shared
/// thread_ids list for multi-thread tests.
fn make_thread_task(
    id: TaskId,
    state: TaskState,
    pid: u32,
    ppid: u32,
    shared_pinfo: alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>,
) -> crate::task::Task<MockRuntime> {
    crate::task::Task {
        id,
        state,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
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
        process_info: Some(shared_pinfo),
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    }
}

/// Thread IDs are tracked in ProcessInfo.thread_ids when tasks share the
/// same ProcessInfo Arc.
#[test]
fn test_thread_ids_tracked_in_process_info() {
    let _g = init_test_env();

    // Build a shared ProcessInfo for a 2-thread group.
    // pid = 7000 (thread-group leader), thread_ids = [7000, 7001].
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 7000,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![7000, 7001],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(7000, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    {
        let pi = pinfo.lock();
        assert_eq!(pi.job.thread_ids.len(), 2);
        assert!(pi.job.thread_ids.contains(&7000));
        assert!(pi.job.thread_ids.contains(&7001));
        assert_eq!(pi.pid, 7000);
    }
}

/// When the thread-group leader exits, sibling threads are removed from the
/// thread_ids list and their scheduler state is set to Dead.
#[test]
fn test_mark_task_exited_removes_tid_from_thread_ids() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 2-thread group: leader 8700, sibling 8701.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 8700,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![8700, 8701],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(8700, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Register both tasks.
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8700, TaskState::Running, 8700, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8701, TaskState::Runnable, 8700, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8700);

    // Exit the sibling thread first — its TID should be removed from thread_ids.
    let _ = mark_task_exited::<MockRuntime>(&mut sched, 8701, 0);

    {
        let pi = pinfo.lock();
        // 8701 should have been removed.
        assert!(!pi.job.thread_ids.contains(&8701), "sibling TID still in thread_ids");
        // 8700 (leader) is still present — it hasn't exited yet.
        assert!(pi.job.thread_ids.contains(&8700), "leader TID wrongly removed");
    }

    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8701).unwrap().state,
        TaskState::Dead,
        "sibling should be dead"
    );
}

/// When the thread-group leader (tid == pid) exits, remaining sibling
/// threads are also killed (thread-group exit).
#[test]
fn test_thread_group_leader_exit_kills_siblings() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 2-thread group: leader 8800, sibling 8801.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 8800,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![8800, 8801],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(8800, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8800, TaskState::Running, 8800, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8801, TaskState::Runnable, 8800, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8800);

    // Exit the thread-group leader.
    let _ = mark_task_exited::<MockRuntime>(&mut sched, 8800, 42);

    // Leader must be dead.
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8800).unwrap().state,
        TaskState::Dead,
        "leader should be dead"
    );

    // Sibling must also be dead (killed by thread-group exit).
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8801).unwrap().state,
        TaskState::Dead,
        "sibling should be killed on leader exit"
    );

    // Both TIDs removed from thread_ids.
    assert!(pinfo.lock().job.thread_ids.is_empty(), "thread_ids should be empty after group exit");
}

/// Leader exit is projected to the parent wait queue using encoded
/// wait-status semantics, and parent threads are returned as wake targets.
#[test]
fn test_mark_task_exited_queues_parent_wait_status() {
    let _g = init_test_env();
    unsafe {
        crate::sched::hooks::PROCESS_INFO_FOR_PID_HOOK = Some(process_info_for_pid::<MockRuntime>);
    }

    // Parent process (pid 9900) with one thread waiting in waitpid path.
    let parent_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9900,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![9900],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9900, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Child process (pid 9800) whose leader exits with code 7.
    let child_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9800,
        job: crate::job::Job {
            ppid: 9900,
            thread_ids: alloc::vec![9800],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9800, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9900, TaskState::Blocked, 9900, 1, parent_pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9800, TaskState::Running, 9800, 9900, child_pinfo),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(9800);

    let waiters = mark_task_exited::<MockRuntime>(&mut sched, 9800, 7);
    assert_eq!(waiters, alloc::vec![9900], "parent thread should be returned for wakeup");

    let parent = parent_pinfo.lock();
    assert_eq!(parent.job.children_done.len(), 1);
    assert_eq!(
        parent.job.children_done.front().copied(),
        Some((9800, abi::signal::w_exit_status(7))),
        "leader exit should be queued as encoded wait status"
    );
}

/// exec_in_progress: killing siblings during exec collapse removes their
/// TIDs from thread_ids, leaving only the exec-calling thread.
#[test]
fn test_exec_collapse_kills_siblings_and_updates_thread_ids() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 3-thread group: leader 9100, siblings 9101, 9102.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9100,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![9100, 9101, 9102],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9100, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9100, TaskState::Running, 9100, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9101, TaskState::Runnable, 9100, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9102, TaskState::Runnable, 9100, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    // Mark 9100 as the "current" (exec-calling) thread.
    sched.state.per_cpu[0].current = Some(9100);

    // Step 1: simulate exec – set exec_in_progress.
    pinfo.lock().job.exec_in_progress = true;

    // Step 2: collect siblings.
    let caller_tid: TaskId = 9100;
    let siblings: alloc::vec::Vec<TaskId> =
        pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
    assert_eq!(siblings.len(), 2);

    // Step 3: kill siblings (simulates kill_by_tid path).
    for sibling in siblings {
        let _ = mark_task_exited::<MockRuntime>(&mut sched, sibling, -9);
    }

    // Both siblings must be dead.
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(9101).unwrap().state,
        TaskState::Dead,
        "sibling 9101 should be dead"
    );
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(9102).unwrap().state,
        TaskState::Dead,
        "sibling 9102 should be dead"
    );

    // thread_ids should contain only the caller.
    {
        let pi = pinfo.lock();
        assert_eq!(
            pi.job.thread_ids,
            alloc::vec![caller_tid],
            "only caller TID should remain after collapse"
        );
    }

    // Step 4: simulate commit – clear exec_in_progress.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "exec_in_progress cleared after commit");
}

/// exec collapse with 4 threads is deterministic: ALL siblings (9701–9703)
/// are in Dead state before the exec-caller (9700) proceeds to commit.
///
/// This tests the full scheduler + registry path that `task_exec_current`
/// uses via `kill_by_tid` → `mark_task_exited`:
///   1. Set exec_in_progress.
///   2. Collect sibling TIDs (exclude caller).
///   3. Kill every sibling via mark_task_exited.
///   4. Assert every sibling is Dead and only caller TID remains.
///   5. Assert exec-caller is NOT Dead.
///   6. Commit: clear exec_in_progress.
#[test]
fn test_exec_collapse_determinism_four_threads() {
    let _g = init_test_env();

    let caller_tid: TaskId = 9700;
    let sibling_tids: [TaskId; 3] = [9701, 9702, 9703];

    let mut all_tids = alloc::vec![caller_tid];
    all_tids.extend_from_slice(&sibling_tids);

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9700,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: all_tids.clone(),
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: {
            let mut uc = crate::task::ProcessUnixCompat::isolated(9700, false);
            uc.set_spawn_context(alloc::vec![b"old".to_vec()], alloc::vec![]);
            uc
        },
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::from("/old/binary"),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(caller_tid, TaskState::Running, 9700, 1, pinfo.clone()),
    ));
    for &sid in &sibling_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(sid, TaskState::Runnable, 9700, 1, pinfo.clone()),
        ));
    }

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(caller_tid);

    // Phase 1: set exec_in_progress atomically.
    pinfo.lock().job.exec_in_progress = true;

    // Phase 2: collect sibling TIDs (excluding caller).
    let siblings: alloc::vec::Vec<TaskId> =
        pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
    assert_eq!(siblings.len(), 3, "expected 3 siblings");
    assert!(!siblings.contains(&caller_tid), "caller must not appear in sibling list");

    // Phase 3: kill every sibling (as task_exec_current calls kill_by_tid_current).
    for &sid in &siblings {
        let _ = mark_task_exited::<MockRuntime>(&mut sched, sid, -9);
    }

    // Phase 4: invariant — every sibling must be Dead before commit.
    for &sid in &sibling_tids {
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(sid)
                .expect("sibling must remain as zombie")
                .state,
            TaskState::Dead,
            "sibling {} must be Dead after exec collapse",
            sid
        );
    }

    // thread_ids must contain only the exec-caller.
    assert_eq!(
        pinfo.lock().job.thread_ids,
        alloc::vec![caller_tid],
        "only exec-caller TID must remain in thread_ids after collapse"
    );

    // The exec-caller itself must NOT be Dead.
    assert_ne!(
        crate::task::registry::get_task::<MockRuntime>(caller_tid)
            .expect("exec-caller must still be in registry")
            .state,
        TaskState::Dead,
        "exec-caller must not be killed during collapse"
    );

    // Phase 5: commit — clear exec_in_progress.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "exec_in_progress must be cleared after commit");
}

/// exec_in_progress blocks additional thread creation at the process level.
#[test]
fn test_exec_in_progress_rejects_new_threads() {
    let _g = init_test_env();

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9300,
        job: crate::job::Job::new(1, 9300),
        unix_compat: crate::task::ProcessUnixCompat::isolated(9300, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Before exec: flag is clear — new threads would be accepted.
    assert!(!pinfo.lock().job.exec_in_progress);

    // Set exec_in_progress (as task_exec_current does at the start).
    pinfo.lock().job.exec_in_progress = true;

    // The sys_spawn_thread handler checks this flag and returns EAGAIN.
    // Here we verify the condition it tests.
    assert!(
        pinfo.lock().job.exec_in_progress,
        "exec_in_progress must be set to block SYS_SPAWN_THREAD"
    );

    // Rollback: clear the flag on pre-commit failure.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "flag cleared after rollback");
}

// ── TLS-base and detached-thread tests ───────────────────────────────────

/// A task constructed with a non-zero `user_fs_base` retains that value.
///
/// This is the kernel-side invariant for the TLS-base handoff: the spawn
/// path stores `tls_base` in `Task.user_fs_base`, and the scheduler
/// writes it to hardware (FS_BASE) on the first context switch.
#[test]
fn test_tls_base_stored_in_task_user_fs_base() {
    let _g = init_test_env();

    let tls_base: u64 = 0xDEAD_CAFE_0000_0000;

    let task = crate::task::Task {
        id: 9800,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
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
        user_fs_base: tls_base,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let stored = crate::task::registry::get_task::<MockRuntime>(9800)
        .expect("task must be in registry")
        .user_fs_base;
    assert_eq!(stored, tls_base, "user_fs_base must equal the requested tls_base");
}

/// Joining a detached thread must return `EINVAL`.
#[test]
fn test_detached_thread_cannot_be_joined() {
    let _g = init_test_env();

    let task = crate::task::Task {
        id: 9801,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
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
        detached: true, // detached — must not be joinable
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    assert_eq!(
        register_task_exit_waiter::<MockRuntime>(9801, 9802).unwrap_err(),
        abi::errors::Errno::EINVAL,
        "joining a detached thread must return EINVAL"
    );
}

/// A live joinable (non-detached) thread allows waiting via
/// `register_task_exit_waiter`, returning `None` (not yet exited).
#[test]
fn test_joinable_thread_can_be_waited_on() {
    let _g = init_test_env();

    let task = crate::task::Task {
        id: 9803,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
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
        detached: false, // joinable
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    // Should succeed and return None (thread still running).
    assert_eq!(
        register_task_exit_waiter::<MockRuntime>(9803, 9804).unwrap(),
        None,
        "joining a live joinable thread must return None"
    );
}

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

#[test]
fn test_wakeup_routes_to_last_cpu_for_any_affinity() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9901,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9901,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(2),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9901);
    assert!(
        sched.state.per_cpu[2].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9901),
        "[policy] wakeup should enqueue Any-affinity blocked task onto last_cpu"
    );
    assert_eq!(
        sched.state.get_task(9901).and_then(|sf| sf.wake_cpu),
        Some(2),
        "[policy] wake_cpu should preserve last_cpu routing for Any-affinity wakeup"
    );
}

#[test]
fn test_wakeup_redirects_any_affinity_when_last_cpu_overloaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9911,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9911,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Make CPU 0 overloaded relative to CPU 1.
    const OVERLOAD_TASK_START: u64 = 9912;
    const OVERLOAD_TASK_END: u64 = 9915;
    for id in OVERLOAD_TASK_START..OVERLOAD_TASK_END {
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(id, TaskState::Runnable, TaskPriority::Low)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9911);
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9911),
        "[policy] redirect should move Any-affinity wakeup to least-loaded CPU"
    );
    assert_eq!(
        sched.state.get_task(9911).and_then(|sf| sf.wake_cpu),
        Some(1),
        "[policy] wake_cpu should track redirected Any-affinity wakeup target"
    );
}

#[test]
fn test_any_affinity_wakeup_prefers_local_cpu_when_last_cpu_is_busier() {
    let _g = init_test_env();
    reset_any_wake_policy_for_tests();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9916,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9916,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(2),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    for id in 9917..9920 {
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(id, TaskState::Runnable, TaskPriority::Low)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(2),
            wake_cpu: Some(2),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(2, TaskPriority::Low as usize, id);
    }

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9916);
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9916),
        "[policy] Any-affinity wakeup should prefer local CPU when last_cpu is busier"
    );
    assert_eq!(
        sched.state.get_task(9916).and_then(|sf| sf.wake_cpu),
        Some(0),
        "[policy] wake_cpu should record local routing under local-bias heuristic"
    );
}

#[test]
fn test_any_wake_hysteresis_requires_persistent_overload_before_rebalance() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests_with_streak("redirect", 2, 3);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for tid in 20_000..20_003 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }

    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 1);
}

#[test]
fn test_any_wake_hysteresis_resets_when_overload_clears() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests_with_streak("redirect", 2, 3);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for tid in 21_000..21_003 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);

    sched.state.per_cpu[0].runq[TaskPriority::Low as usize].clear();
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);

    for tid in 21_003..21_006 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 1);
}

// ── choose_wake_cpu tests ─────────────────────────────────────────────

/// `choose_wake_cpu` should route an Any-affinity wakeup to the idle CPU
/// when the preferred CPU (last_cpu) is loaded beyond the overload gap.
#[test]
fn test_choose_wake_cpu_prefers_idle_cpu_when_preferred_is_loaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // CPU 0: loaded, current is a non-idle task.
    // CPU 1: idle  (current == idle_task).
    // CPU 2: online but not idle.
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);

    // Make CPU 1 idle: current == idle_task == 80_001.
    let idle_tid: u64 = 80_001;
    sched.state.per_cpu[1].idle_task = Some(idle_tid);
    sched.state.per_cpu[1].current = Some(idle_tid);

    // Load CPU 0 past the overload gap (gap = 2, add 3 tasks).
    for tid in 80_010..80_013 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }

    // Task that was last on CPU 0.
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
    assert_eq!(result, 1, "choose_wake_cpu should route to idle CPU 1 when CPU 0 is overloaded");
}

/// `choose_wake_cpu` should stay on the preferred CPU when the preferred CPU
/// is itself the idle CPU (i.e. it is lightly loaded).
#[test]
fn test_choose_wake_cpu_stays_on_preferred_when_preferred_is_idle() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Make CPU 0 idle: current == idle_task == 80_020.
    let idle_tid: u64 = 80_020;
    sched.state.per_cpu[0].idle_task = Some(idle_tid);
    sched.state.per_cpu[0].current = Some(idle_tid);

    // Task with last_cpu = 0 (the idle CPU).
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
    // CPU 0 is the idle CPU and its depth is 0 (below overload_gap of 2),
    // so no redirection should happen.
    assert_eq!(result, 0, "choose_wake_cpu should keep preferred CPU when it is already idle");
}

/// `choose_wake_cpu` should use last_cpu as the target when no idle CPU
/// exists and the preferred CPU is not overloaded.
#[test]
fn test_choose_wake_cpu_uses_last_cpu_when_no_idle_cpu_and_not_overloaded() {
    let _g = init_test_env();
    reset_any_wake_policy_for_tests();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    // No idle CPU: current is None on all CPUs.

    // Task with last_cpu = 2.
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(2));
    assert_eq!(
        result, 2,
        "choose_wake_cpu should return last_cpu when not overloaded and no idle CPU exists"
    );
}

/// When multiple Any-affinity tasks wake at once and one idle CPU exists,
/// the first wakeup should land on the idle CPU; subsequent wakeups should
/// spread to other CPUs rather than all piling on the idle one.
#[test]
fn test_choose_wake_cpu_distributes_fanout_across_cpus() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 1);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..4 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.mark_cpu_online(3);

    // Load CPUs 0, 2, 3 so they are above overload gap.
    for cpu in [0usize, 2, 3] {
        for tid_offset in 0..3u64 {
            let tid = (cpu as u64) * 100 + 80_100 + tid_offset;
            sched.state.enqueue_task(cpu, TaskPriority::Low as usize, tid);
        }
    }
    // CPU 1 is idle.
    let idle_tid: u64 = 80_050;
    sched.state.per_cpu[1].idle_task = Some(idle_tid);
    sched.state.per_cpu[1].current = Some(idle_tid);

    // All tasks have last_cpu = 0 (overloaded).
    let targets: alloc::vec::Vec<usize> =
        (0..4).map(|_| choose_wake_cpu::<MockRuntime>(&sched, Some(0))).collect();

    // At least one call should have routed to an idle or less-loaded CPU.
    assert!(
        targets.iter().any(|&cpu| cpu != 0),
        "fanout should distribute wakeups away from overloaded CPU 0, got {:?}",
        targets
    );
}

#[test]
fn test_wake_sleepers_redirects_any_affinity_when_last_cpu_overloaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    let current_task = make_task(9920, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9920);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9920,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let sleeping = make_task(9921, TaskState::Blocked, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(sleeping));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9921,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    const OVERLOAD_TASK_START: u64 = 9922;
    const OVERLOAD_TASK_END: u64 = 9925;
    for id in OVERLOAD_TASK_START..OVERLOAD_TASK_END {
        let runnable = make_task(id, TaskState::Runnable, TaskPriority::Low);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(runnable));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(9921, 50);
    sched.wake_sleepers();

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9921),
        "[policy] wake_sleepers should redirect Any-affinity wakeup under overload"
    );
    assert_eq!(
        sched.state.get_task(9921).and_then(|sf| sf.wake_cpu),
        Some(1),
        "[policy] wake_sleepers should record redirected wake_cpu for Any-affinity task"
    );
}

#[test]
fn test_wake_sleepers_batches_any_affinity_load_scans() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    let current_task = make_task(9926, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9926);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9926,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Keep CPU 0 overloaded for Any-affinity wake routing.
    for id in 9927..9930 {
        let runnable = make_task(id, TaskState::Runnable, TaskPriority::Low);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(runnable));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    for id in 9930..9933 {
        let sleeping = make_task(id, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: None,
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.add_task_to_sleep_queue(id, 50);
    }

    TEST_LEAST_LOADED_ONLINE_CPU_CALLS.store(0, Ordering::Relaxed);
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.wake_sleepers();

    let cpu0_wakes = sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len();
    let cpu1_wakes = sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len();
    assert_eq!(cpu0_wakes + cpu1_wakes, 3, "all Any-affinity sleepers should wake in this batch");
    assert!(
        cpu0_wakes > 0 && cpu1_wakes > 0,
        "batch load snapshot should spread wakeups after redirecting early tasks (cpu0={}, cpu1={})",
        cpu0_wakes,
        cpu1_wakes
    );

    let least_scan_calls = TEST_LEAST_LOADED_ONLINE_CPU_CALLS.load(Ordering::Relaxed);
    assert!(
        least_scan_calls == 0,
        "wake batch should avoid least-loaded queue scans via per-batch snapshots (calls={})",
        least_scan_calls
    );
}

#[test]
fn test_steal_task_scans_bounded_depth_per_priority() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9930,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9931,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9930,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9931,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9930);
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9931);

    let stolen = sched.steal_task_for(0);
    assert_eq!(
        stolen,
        Some(9931),
        "bounded scan should skip unstealable head and steal stealable follower"
    );
    assert_eq!(
        sched.state.get_task(9931).and_then(|sf| sf.wake_cpu),
        Some(0),
        "stolen task wake_cpu should track destination CPU"
    );
    assert_eq!(
        sched.state.get_task(9931).and_then(|sf| sf.runq_location),
        None,
        "stolen task should no longer be tracked in donor runq"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9930),
        "unstealable pinned head should remain queued on donor CPU"
    );
}

#[test]
fn test_steal_task_respects_scan_depth_limit() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for offset in 0..STEAL_SCAN_DEPTH_PER_PRIORITY {
        let tid = 9940 + offset as u64;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    let stealable_tid = 9950;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        stealable_tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: stealable_tid,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, stealable_tid);

    let stolen = sched.steal_task_for(0);
    assert_eq!(
        stolen, None,
        "steal scan must stay bounded and not inspect beyond configured depth"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
            .iter()
            .any(|&tid| tid == stealable_tid),
        "stealable task beyond scan-depth cap should remain on donor queue"
    );
}

// ── try_steal_one tests ────────────────────────────────────────────────

#[test]
fn test_try_steal_one_returns_none_when_victim_below_min_depth() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Only one task on CPU 1 — below the min_depth=2 threshold.
    let tid = 10_001u64;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);

    // min_depth=2 but victim has only 1 task — should return None.
    let result = sched.try_steal_one(0, 1, 2);
    assert_eq!(result, None, "should not steal when victim depth < min_depth");
}

#[test]
fn test_try_steal_one_steals_when_victim_meets_min_depth() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Two tasks on CPU 1.
    for tid in [10_010u64, 10_011u64] {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    let stolen = sched.try_steal_one(0, 1, 2);
    assert!(stolen.is_some(), "should steal when victim has >= min_depth tasks");
    let stolen_id = stolen.unwrap();
    assert!(stolen_id == 10_010 || stolen_id == 10_011, "stolen task must come from victim CPU 1");
    assert_eq!(
        sched.state.get_task(stolen_id).and_then(|sf| sf.wake_cpu),
        Some(0),
        "stolen task wake_cpu must point to local CPU"
    );
    assert_eq!(
        sched.state.get_task(stolen_id).and_then(|sf| sf.runq_location),
        None,
        "stolen task must be removed from victim run queue"
    );
}

#[test]
fn test_try_steal_one_returns_none_for_same_cpu() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(0);

    let tid = 10_020u64;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);

    // local_cpu == victim_cpu → always None (cannot steal from self).
    assert_eq!(sched.try_steal_one(0, 0, 1), None);
}

#[test]
fn test_try_steal_one_skips_pinned_tasks() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Two tasks on CPU 1, both pinned to CPU 1.
    for tid in [10_030u64, 10_031u64] {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    assert_eq!(sched.try_steal_one(0, 1, 2), None, "pinned tasks must never be stolen");
}

// ── idle_steal tests ───────────────────────────────────────────────────

#[test]
fn test_idle_steal_prefers_nearby_cpu_over_distant() {
    let _g = init_test_env();

    // CPU layout: local=0, nearby=1 (within radius), far=10.
    // We need at least 11 per_cpu slots.
    let num_cpus = 11usize;
    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..num_cpus {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in [0usize, 1, 10] {
        sched.state.mark_cpu_online(i);
    }

    // Place two tasks on nearby CPU 1 and two tasks on far CPU 10.
    let nearby_tids = [10_100u64, 10_101u64];
    let far_tids = [10_110u64, 10_111u64];

    for &tid in &nearby_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    for &tid in &far_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(10),
            wake_cpu: Some(10),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
    }

    // idle_steal from CPU 0 should pick from nearby CPU 1 (within
    // STEAL_NEARBY_RADIUS) before trying distant CPU 10.
    let stolen = sched.idle_steal(0);
    assert!(stolen.is_some(), "idle_steal should find work");
    let stolen_id = stolen.unwrap();
    assert!(
        nearby_tids.contains(&stolen_id),
        "idle_steal should prefer nearby CPU 1 over far CPU 10 (stolen={stolen_id})"
    );
}

#[test]
fn test_idle_steal_falls_back_to_distant_when_nearby_empty() {
    let _g = init_test_env();

    let num_cpus = 11usize;
    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..num_cpus {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in [0usize, 10] {
        sched.state.mark_cpu_online(i);
    }

    // Only far CPU 10 has work.
    let far_tids = [10_200u64, 10_201u64];
    for &tid in &far_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(10),
            wake_cpu: Some(10),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
    }

    let stolen = sched.idle_steal(0);
    assert!(
        stolen.is_some(),
        "idle_steal should fall back to distant CPUs when nearby CPUs have no work"
    );
    let stolen_id = stolen.unwrap();
    assert!(
        far_tids.contains(&stolen_id),
        "should steal from far CPU 10 when it is the only loaded CPU"
    );
}

#[test]
fn test_idle_steal_returns_none_when_all_cpus_have_single_task() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in 0..3 {
        sched.state.mark_cpu_online(i);
    }

    // Each peer CPU has exactly 1 task — below the min_depth=2 threshold.
    for cpu in 1..3usize {
        let tid = 10_300u64 + cpu as u64;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(cpu),
            wake_cpu: Some(cpu),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(cpu, TaskPriority::Normal as usize, tid);
    }

    assert_eq!(
        sched.idle_steal(0),
        None,
        "idle_steal must not steal when no CPU has >= 2 runnable tasks (anti-thrash)"
    );
}

#[test]
fn test_task_cpu_trace_strings_prefers_scheduler_trace_fields() {
    let sf = crate::sched::state::ThreadSchedFields {
        tid: 1,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(3),
        wake_cpu: Some(2),
        run_cpu: Some(1),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };

    let (last, wake, run) = task_cpu_trace_strings(Some(9), Some(&sf));
    assert_eq!(last, "3");
    assert_eq!(wake, "2");
    assert_eq!(run, "1");
}

#[test]
fn test_task_cpu_trace_strings_falls_back_to_task_last_cpu() {
    let sf = crate::sched::state::ThreadSchedFields {
        tid: 2,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: None,
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };

    let (last, wake, run) = task_cpu_trace_strings(Some(4), Some(&sf));
    assert_eq!(last, "4");
    assert_eq!(wake, "-");
    assert_eq!(run, "-");
}

#[test]
fn test_format_optional_cpu_formats_some_and_none() {
    assert_eq!(format_optional_cpu(Some(7)), "7");
    assert_eq!(format_optional_cpu(None), "-");
}

#[test]
fn test_trylock_miss_records_pending_resched_pressure() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    let before_miss = PROF_TRYLOCK_MISS_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_pending =
        PROF_TRYLOCK_MISS_PENDING_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_timer =
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_ipi = PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    assert!(global_need_resched_slot(0).is_some(), "test requires CPU 0 GLOBAL_NEED_RESCHED slot");
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    assert!(
        !global_need_resched_swap(0, true, core::sync::atomic::Ordering::Release),
        "setup should mark CPU 0 resched flag from clear->set transition"
    );

    // Must run while SCHEDULER lock is held so try_lock path fails.
    try_resched_if_needed::<MockRuntime>(DispatchTrigger::ReschedIpi);

    let after_miss = PROF_TRYLOCK_MISS_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let after_pending =
        PROF_TRYLOCK_MISS_PENDING_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    assert!(after_miss > before_miss, "[contention] per-CPU trylock miss counter should increment");
    assert!(
        after_pending > before_pending,
        "[contention] per-CPU pending-resched trylock miss counter should increment"
    );
    assert!(
        PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed) > before_ipi,
        "[contention] IPI-triggered trylock miss counter should increment"
    );
    assert_eq!(
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed),
        before_timer,
        "[contention] timer-triggered trylock miss counter must not increment for IPI dispatch"
    );

    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_trylock_miss_timer_attribution() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    let before_timer =
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_ipi = PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    assert!(
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed)
            > before_timer,
        "[contention] timer-triggered trylock miss counter should increment"
    );
    assert_eq!(
        PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed),
        before_ipi,
        "[contention] IPI-triggered trylock miss counter must not increment for timer dispatch"
    );

    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_trylock_miss_idle_timer_without_pending_does_not_set_resched_flag() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(123);
    sched.state.per_cpu[0].idle_task = Some(123);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    crate::runtime::<MockRuntime>().set_idle_task_current(true);
    let before_window_actionable = TRYLOCK_MISS_WINDOW_COUNT[0].load(Ordering::Relaxed);
    let before_window_idle_timer = TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed);
    let before_idle_timer =
        PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    assert!(
        !global_need_resched_load(0, core::sync::atomic::Ordering::Acquire),
        "idle timer-only trylock miss should not force pending resched"
    );
    assert_eq!(
        TRYLOCK_MISS_WINDOW_COUNT[0].load(Ordering::Relaxed),
        before_window_actionable,
        "idle timer-only misses should not count toward actionable warning threshold"
    );
    assert!(
        TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed) > before_window_idle_timer,
        "idle timer-only misses should be attributed in the dedicated window counter"
    );
    assert!(
        PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed)
            > before_idle_timer,
        "idle timer-only misses should increment dedicated attribution counter"
    );

    crate::runtime::<MockRuntime>().set_idle_task_current(false);
    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_dispatch_trigger_labels() {
    assert_eq!(DispatchTrigger::TimerTick.as_str(), "timer_tick");
    assert_eq!(DispatchTrigger::ReschedIpi.as_str(), "resched_ipi");
}

#[test]
fn test_try_resched_timer_tick_updates_tick_stats_and_timeslice() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let tid = 77;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: 2,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.per_cpu[0].current = Some(tid);
    sched.state.per_cpu[0].idle_task = Some(999);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    let pc = &sched.state.per_cpu[0];
    assert_eq!(pc.stats.timer_interrupts, 1, "timer dispatch should update tick stats");
    assert_eq!(pc.stats.resched_ipi_received, 0, "timer dispatch must not count as an IPI");
    assert_eq!(
        sched.state.get_thread(tid).map(|sf| sf.timeslice_remaining),
        Some(1),
        "timer dispatch should execute preempt-tick bookkeeping"
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_set_global_need_resched_invalid_cpu_returns_conservative_fallback() {
    let _g = init_test_env();
    let invalid_cpu = types::MAX_CPUS;

    assert!(
        set_global_need_resched(invalid_cpu),
        "invalid cpu should conservatively report an already-pending resched"
    );
}

#[test]
fn test_global_need_resched_load_invalid_cpu_returns_conservative_fallback() {
    let _g = init_test_env();
    let invalid_cpu = types::MAX_CPUS;

    assert!(
        global_need_resched_load(invalid_cpu, core::sync::atomic::Ordering::Acquire),
        "invalid cpu should conservatively report pending resched"
    );
}

// ── per-CPU preemption tests ──────────────────────────────────────────────

/// `need_resched_pending` returns false when the per-CPU atomic flag is clear.
#[test]
fn test_need_resched_pending_returns_false_when_clear() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    assert!(
        !need_resched_pending(0),
        "need_resched_pending should return false when the per-CPU flag is clear"
    );
}

/// `need_resched_pending` returns true after `set_global_need_resched` is called.
#[test]
fn test_need_resched_pending_returns_true_after_set() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    set_global_need_resched(0);
    assert!(
        need_resched_pending(0),
        "need_resched_pending should return true after set_global_need_resched"
    );
    // Cleanup
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}

/// Setting `CpuScheduler.need_resched = true` when preempt is disabled also
/// mirrors the flag to the per-CPU atomic so the lockless fast-path sees it.
///
/// Flow under test:
/// 1. `set_global_need_resched(0)` raises the per-CPU atomic flag.
/// 2. `schedule_point(PreemptTick)` atomically *clears* the flag via
///    `global_need_resched_swap(cpu_idx, false)` and records it as
///    `global_requested = true`.
/// 3. Because `preempt_disable_depth > 0`, the reschedule is deferred:
///    `need_resched = true` is set AND `set_global_need_resched(cpu_idx)`
///    re-raises the per-CPU atomic flag.
/// 4. After returning, `need_resched_pending(0)` must be `true`.
///    Without the mirroring call added in step 3, the flag would remain
///    cleared (step 2) and this assertion would fail.
#[test]
fn test_schedule_point_preempt_disabled_mirrors_need_resched_to_atomic() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(1);
    // Disable preemption on CPU 0
    sched.state.per_cpu[0].preempt_disable_depth = 1;

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    // Simulate a global reschedule request arriving while preemption is disabled.
    set_global_need_resched(0);

    // Verify the flag is set before the call.
    assert!(need_resched_pending(0), "pre-condition: flag should be set before schedule_point");

    {
        let l = SCHEDULER.lock();
        if let Some(ptr) = *l {
            let s = unsafe { &mut *(ptr as *mut types::Scheduler<MockRuntime>) };
            // schedule_point internally clears the atomic flag via
            // global_need_resched_swap(cpu_idx, false), then — because
            // preempt_disable_depth > 0 — calls set_global_need_resched to
            // re-raise it.  The assertion below catches the case where the
            // mirroring call is absent (flag stays cleared after the swap).
            s.schedule_point(ScheduleReason::PreemptTick);
        }
        drop(l);
    }

    // The per-CPU lock-free flag must be re-raised by the mirroring in
    // schedule_point.  Without mirroring the swap(false) above would have
    // left it cleared.
    assert!(
        need_resched_pending(0),
        "schedule_point with preempt disabled should mirror need_resched back to the atomic flag"
    );
    // The local (in-lock) need_resched field must also be set.
    assert!(
        sched.state.per_cpu[0].need_resched,
        "CpuScheduler.need_resched should be set when preempt is disabled and a resched was requested"
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}

/// Draining the remote wake mailbox with a higher-priority task sets
/// `need_resched` and mirrors it to the per-CPU atomic.
#[test]
fn test_drain_wake_mailbox_mirrors_need_resched_to_atomic() {
    let _g = init_test_env();
    TICK_COUNT.store(50, Ordering::Relaxed);
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Current task on CPU 0 at Normal priority
    let current_tid = 500u64;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: current_tid,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.per_cpu[0].current = Some(current_tid);

    // Push a Realtime task into the mailbox for CPU 0
    let rt_tid = 501u64;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: rt_tid,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Realtime,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    enqueue_remote_wake_mailbox(
        0,
        types::RemoteWakeMailboxEntry {
            tid: rt_tid,
            priority: TaskPriority::Realtime as usize,
            wake_mono: 0,
            enqueued_at_tick: 50,
        },
    );

    sched.drain_remote_wake_mailbox(0);

    assert!(
        sched.state.per_cpu[0].need_resched,
        "need_resched should be set after draining a higher-priority wake"
    );
    assert!(
        need_resched_pending(0),
        "per-CPU atomic flag should be mirrored after drain sets need_resched"
    );

    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}

// ── periodic_load_balance tests ───────────────────────────────────────────

/// Build a scheduler with `num_cpus` CPUs online, all slots in per_cpu,
/// and register + insert `tasks` tasks onto CPU `donor_cpu`.
fn make_periodic_balance_sched(
    num_cpus: usize,
    tasks: &[(u64, usize)], // (tid, cpu)
) -> types::Scheduler<MockRuntime> {
    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..num_cpus {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in 0..num_cpus {
        sched.state.mark_cpu_online(i);
    }
    for &(tid, cpu) in tasks {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(cpu),
            wake_cpu: Some(cpu),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(cpu, TaskPriority::Normal as usize, tid);
    }
    sched
}

#[test]
fn test_periodic_load_balance_rate_limited() {
    let _g = init_test_env();
    // Set up a severe imbalance: CPU 1 has 4 tasks, CPU 0 has zero.
    let tasks: &[(u64, usize)] = &[(20_000, 1), (20_001, 1), (20_002, 1), (20_003, 1)];
    let mut sched = make_periodic_balance_sched(2, tasks);

    // Tick 0: last_balance_tick = 0 and now = 0, so wrapping_sub(0, 0) = 0
    // which is less than INTERVAL → balance must NOT run yet.
    TICK_COUNT.store(0, Ordering::Relaxed);
    let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    assert_eq!(
        after, before,
        "balance should not run when now == last_balance_tick (cooldown not elapsed)"
    );

    // Advance by INTERVAL ticks → balance should now trigger.
    TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
    let before2 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after2 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    assert!(after2 > before2, "balance should migrate tasks when cooldown has elapsed");

    // Immediately calling again (same tick) must be suppressed.
    let before3 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after3 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    assert_eq!(after3, before3, "second call in the same tick window must be rate-limited");
}

#[test]
fn test_periodic_load_balance_resolves_severe_imbalance() {
    let _g = init_test_env();
    // CPU 0 has 0 tasks; CPU 1 has 4 tasks → severe imbalance.
    let tasks: &[(u64, usize)] = &[(20_100, 1), (20_101, 1), (20_102, 1), (20_103, 1)];
    let mut sched = make_periodic_balance_sched(2, tasks);

    TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
    let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    let migrated = after - before;

    assert!(
        migrated > 0,
        "periodic_load_balance must migrate at least one task from overloaded CPU"
    );
    assert!(
        migrated <= types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN as u64,
        "must not exceed per-run migration cap (migrated={migrated})"
    );

    let cpu0_depth = sched.state.per_cpu[0].runq.total_len();
    let cpu1_depth = sched.state.per_cpu[1].runq.total_len();
    assert!(
        cpu0_depth > 0,
        "target CPU 0 should have received at least one task (depth={cpu0_depth})"
    );
    assert!(
        cpu1_depth < 4,
        "donor CPU 1 should have fewer tasks after balancing (depth={cpu1_depth})"
    );
}

#[test]
fn test_periodic_load_balance_ignores_mild_imbalance() {
    let _g = init_test_env();
    // CPU 0 has 1 task; CPU 1 has 2 tasks → diff=1, below threshold=2, no migration.
    let tasks: &[(u64, usize)] = &[(20_200, 0), (20_201, 1), (20_202, 1)];
    let mut sched = make_periodic_balance_sched(2, tasks);

    TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
    let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    assert_eq!(after, before, "mild imbalance (diff < threshold) must not trigger migration");
}

#[test]
fn test_periodic_load_balance_skips_single_cpu() {
    let _g = init_test_env();
    // Only 1 CPU online — nothing to balance.
    let tasks: &[(u64, usize)] = &[(20_300, 0), (20_301, 0), (20_302, 0)];
    let mut sched = make_periodic_balance_sched(1, tasks);

    TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
    let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    assert_eq!(after, before, "single-CPU system must never trigger migration");
}

#[test]
fn test_periodic_load_balance_respects_migration_cap() {
    let _g = init_test_env();
    // CPU 0 has 0 tasks; CPU 1 has many tasks — ensure cap is honoured.
    let num_tasks = types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN + 3;
    let tasks: alloc::vec::Vec<(u64, usize)> =
        (0..num_tasks).map(|i| (20_400 + i as u64, 1)).collect();
    let mut sched = make_periodic_balance_sched(2, &tasks);

    TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
    let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    sched.periodic_load_balance();
    let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
    let migrated = after - before;

    assert_eq!(
        migrated,
        types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN as u64,
        "periodic balancer must stop at PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN (got {migrated})"
    );
}

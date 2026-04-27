use super::super::*;
use crate::sched::balancing::reset_any_wake_policy_for_tests;
use crate::sched::mailbox::reset_remote_wake_mailboxes_for_tests;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
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

pub(crate) fn make_task(
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

// ── waitpid tests ─────────────────────────────────────────────────────────

/// Helper: build a task with a populated ProcessInfo (pid + ppid).
pub(crate) fn make_process_task(
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

/// Helper: create a task + ProcessInfo with `tgid` populated and a shared
/// thread_ids list for multi-thread tests.
pub(crate) fn make_thread_task(
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

// ── periodic_load_balance tests ───────────────────────────────────────────

/// Build a scheduler with `num_cpus` CPUs online, all slots in per_cpu,
/// and register + insert `tasks` tasks onto CPU `donor_cpu`.
pub(crate) fn make_periodic_balance_sched(
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

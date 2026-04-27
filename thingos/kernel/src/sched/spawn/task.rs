use core::sync::atomic::{AtomicUsize, Ordering};

use super::super::SCHEDULER;
use super::super::types::{DEFAULT_TIMESLICE, Scheduler};
use super::process_info::{current_parent_pid, default_process_info};
use crate::task::{Affinity, StartupArg, Task, TaskId, TaskState};
use crate::{BootRuntime, BootTasking, UserEntry};

const KERNEL_STACK_SIZE: usize = 65536;

// Global round-robin index for CPU selection
pub(crate) static RR_IDX: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn with_spawn_scheduler_lock<R: BootRuntime, T>(
    current_cpu: usize,
    f: impl FnOnce(&mut Scheduler<R>) -> T,
) -> T {
    let lock = SCHEDULER.lock();
    let _tracking = crate::sched::sched_lock_tracking_guard::<R>(current_cpu);
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    f(sched)
}

/// Register `tid` in the owning process's thread-group list.
///
/// This is the **single, mandatory** registration point for every thread that
/// belongs to a user process.  All spawn paths (`spawn_user_thread`,
/// `spawn_user_task`, `boot_spawn_process`, `boot_spawn_process_ex`,
/// `spawn_process_from_path`) call this function so that exec-collapse and
/// thread-group exit can reliably enumerate every live thread via
/// `ProcessInfo::thread_ids`.
///
/// Duplicate-safe: the TID is only appended if not already present, so calling
/// this more than once for the same TID is harmless.
///
/// # Why this matters
/// Previously the registration in `spawn_user_thread` was inadvertently gated
/// on `tls_base != 0`.  Threads created without a TLS base (the common case for
/// many POSIX-style threads) were invisible to exec-collapse and thread-group
/// teardown.  Extracting the logic into this function ensures:
///   1. The invariant is expressed once, not scattered across callers.
///   2. Future spawn paths cannot forget to register by accident — they just
///      call `register_thread_in_process` after building the task.
pub(crate) fn register_thread_in_process(
    pinfo: &Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>>,
    tid: crate::task::TaskId,
) {
    if let Some(pinfo) = pinfo {
        let mut pi = pinfo.lock();
        if !pi.job.thread_ids.contains(&tid) {
            pi.job.thread_ids.push(tid);
        }
    }
}

impl<R: BootRuntime> Scheduler<R> {
    fn pick_cpu_and_bringup(&mut self, affinity: Affinity, _trigger_smp: bool) -> usize {
        let rt = crate::runtime::<R>();
        let count = self.state.online_cpu_count;
        let idx = RR_IDX.fetch_add(1, Ordering::Relaxed);

        match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => {
                // During early boot keep all tasks on the local (boot) CPU to
                // avoid cross-CPU placement overhead and unnecessary IPI
                // traffic before steady-state scheduling begins.
                if self.bringup_in_progress {
                    return crate::sched::current_cpu_index::<R>();
                }
                // will be brought up manually when needed.
                //     if let Some(next_cpu_id) = rt.next_offline_cpu() {
                //         let target_cpu = next_cpu_id.0 as usize;
                //         unsafe {
                //             let _ = rt.start_cpu(next_cpu_id, crate::kernel_secondary_entry::<R>, target_cpu);
                //         }
                //         return target_cpu;
                //     }
                // }
                let _ = (rt, count); // Suppress unused variable warning
                self.state.pick_online_cpu_excluding_bsp(idx)
            }
            Affinity::Restricted(ref aff) => {
                // During early boot keep tasks local to avoid cross-CPU
                // placement overhead, same as Any.
                if self.bringup_in_progress {
                    let boot_cpu = crate::sched::current_cpu_index::<R>();
                    // The task may not be in the allowed set; the normal
                    // scheduler picker will detect the misroute via
                    // `prepare_schedule` and call `defer_or_repair_misroute`
                    // to move it to an allowed CPU once steady-state scheduling
                    // begins.
                    return boot_cpu;
                }
                let cpu_count = self.state.per_cpu.len().max(1);
                let target = aff.pick_cpu(cpu_count).unwrap_or_else(|| {
                    // Allowed set is empty or all CPUs are offline: fall back to
                    // round-robin among online CPUs so the task is not lost.
                    crate::kwarn!(
                        "SCHED: Affinity::Restricted has no valid CPU in {} online CPUs; \
                         falling back to round-robin",
                        cpu_count
                    );
                    self.state.pick_online_cpu_excluding_bsp(idx)
                });
                crate::kdebug!(
                    "SCHED[affinity]: Restricted(mask={:#x}) → cpu{}",
                    aff.allowed.0,
                    target
                );
                target
            }
        }
    }
    pub fn spawn(
        &mut self,
        entry: extern "C" fn(usize) -> !,
        arg: StartupArg,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack for task {}", id);
        }
        let stack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let ctx = rt.tasking().init_kernel_context(entry, stack_top, arg.to_raw());

        // Determine target CPU: Balanced among online CPUs.
        let target_cpu = self.pick_cpu_and_bringup(affinity, false);
        crate::kdebug!("SCHED: Task {} assigned to CPU {}", id, target_cpu);

        // Push to target CPU's run queue
        let cpu_count = self.state.per_cpu.len(); // Should match rt.cpu_count()
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace: rt.tasking().active_address_space(),
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            enqueued_at_tick: crate::sched::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let sched_fields = crate::sched::bridge::TaskSchedCache::from_thread(&task)
            .with_wake_cpu(Some(safe_cpu))
            .into_sched_fields(task.id);
        self.state.insert_task(sched_fields);
        let boxed_task = alloc::boxed::Box::new(task);
        if crate::sched::scheduler_lock_held_by_this_cpu::<R>() {
            self.pending_registry_inserts.push(boxed_task);
        } else {
            crate::task::registry::get_registry::<R>().insert(boxed_task);
        }

        let current_cpu = crate::sched::current_cpu_index::<R>();
        let now_tick = crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
        let wake_mono = crate::runtime::<R>().mono_ticks();
        if safe_cpu == current_cpu {
            // Local CPU: enqueue directly into the local run queue.
            self.state.enqueue_task(safe_cpu, priority as usize, id);
        } else {
            // Remote CPU: route through the wake mailbox so the owning CPU
            // enqueues the task itself at its next scheduling point.
            crate::sched::enqueue_remote_wake_mailbox(
                safe_cpu,
                crate::sched::types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: priority as usize,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }

        // Under the scheduler lock we only mark the target CPU dirty.
        // The caller sends the IPI after unlocking so the target CPU's
        // resched handler can take the lock immediately.
        if safe_cpu == current_cpu {
            self.state.per_cpu[safe_cpu].need_resched = true;
        }
        // GLOBAL_NEED_RESCHED and the actual IPI send for remote CPUs are
        // handled post-lock by nudge_spawned_task to avoid sending IPIs while
        // holding the scheduler lock (which delays the target CPU's handler).

        let _parent_tid = self.state.per_cpu[current_cpu].current;

        id
    }

    pub fn spawn_user_thread(
        &mut self,
        entry: usize,
        stack: usize,
        arg: StartupArg,
        stack_info: abi::types::StackInfo,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
        tls_base: u64,
        detached: bool,
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate kernel stack for user thread {}", id);
        }
        let kstack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        // Inherit address space, mappings, and process_info from the current process (not the
        // current task) so that the canonical VM state is always sourced from
        // Process rather than from an arbitrary thread's cached copy.
        let parent_task = if let Some(current_id) =
            self.state.per_cpu[crate::sched::current_cpu_index::<R>()].current
        {
            crate::task::registry::get_task::<R>(current_id)
        } else {
            None
        };

        let aspace = parent_task
            .as_ref()
            .map(|t| t.aspace)
            .unwrap_or_else(|| rt.tasking().active_address_space());

        let parent_pinfo = parent_task.as_ref().and_then(|parent| parent.process_info.clone());

        // Clone the mappings Arc from the parent process (same underlying
        // MappingList object).  Fall back to an empty list only when there is
        // no parent process (should not happen for user threads).
        let mappings =
            parent_pinfo.as_ref().map(|pi| pi.lock().space.mappings_arc()).unwrap_or_else(|| {
                alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
            });

        let spec = crate::UserTaskSpec {
            entry: entry as u64,
            stack_top: stack as u64,
            aspace,
            arg: arg.to_raw(),
        };

        let ctx = rt.tasking().init_user_context(spec, kstack_top);

        let target_cpu = self.pick_cpu_and_bringup(affinity, false);
        crate::kdebug!("SCHED: Task {} (user thread) assigned to CPU {}", id, target_cpu);

        // Push to target CPU's run queue
        let cpu_count = self.state.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: Some(stack_info),
            mappings,
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: parent_pinfo,
            enqueued_at_tick: crate::sched::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: tls_base,
            detached,
            signals: crate::signal::ThreadSignals::new(),
        };

        // Register this thread's TID in the owning process so exec and exit
        // can enumerate all threads.  Delegated to the central helper so that
        // every spawn path enforces the invariant identically and future callers
        // cannot accidentally skip registration.
        register_thread_in_process(&task.process_info, id);

        let sched_fields = crate::sched::bridge::TaskSchedCache::from_thread(&task)
            .with_wake_cpu(Some(safe_cpu))
            .into_sched_fields(task.id);
        self.state.insert_task(sched_fields);
        let boxed_task = alloc::boxed::Box::new(task);
        if crate::sched::scheduler_lock_held_by_this_cpu::<R>() {
            self.pending_registry_inserts.push(boxed_task);
        } else {
            crate::task::registry::get_registry::<R>().insert(boxed_task);
        }

        let current_cpu = crate::sched::current_cpu_index::<R>();
        let now_tick = crate::sched::TICK_COUNT.load(Ordering::Relaxed);
        let wake_mono = crate::runtime::<R>().mono_ticks();
        if safe_cpu == current_cpu {
            self.state.enqueue_task(safe_cpu, priority as usize, id);
        } else {
            crate::sched::enqueue_remote_wake_mailbox(
                safe_cpu,
                crate::sched::types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: priority as usize,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }

        // Under the scheduler lock we only mark the target CPU dirty.
        if safe_cpu == current_cpu {
            self.state.per_cpu[safe_cpu].need_resched = true;
        }
        // GLOBAL_NEED_RESCHED and the actual IPI send for remote CPUs are
        // handled post-lock by nudge_spawned_task.

        let _parent_tid = self.state.per_cpu[current_cpu].current;

        id
    }

    pub fn spawn_user_task(
        &mut self,
        entry: crate::UserEntry,
        aspace: <R::Tasking as BootTasking>::AddressSpace,
        stack_info: abi::types::StackInfo,
        regions: alloc::vec::Vec<abi::vm::VmRegionInfo>,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
    ) -> Option<TaskId> {
        let rt = crate::runtime::<R>();
        let id = self.next_id;

        self.next_id += 1;
        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            return None;
        }
        let stack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking().init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let mapping_list = crate::memory::mappings::MappingList { regions };
        let ppid = current_parent_pid::<R>(self);
        let mappings_arc = alloc::sync::Arc::new(spin::Mutex::new(mapping_list));
        let aspace_raw = rt.tasking().aspace_to_raw(aspace);
        let pinfo = default_process_info(
            id as u32,
            ppid,
            crate::task::ProcessAddressSpace::from_parts(mappings_arc.clone(), aspace_raw),
        );

        let target_cpu = self.pick_cpu_and_bringup(affinity, true);
        let cpu_count = self.state.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: Some(stack_info),
            mappings: mappings_arc,
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: Some(pinfo),
            enqueued_at_tick: crate::sched::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let sched_fields = crate::sched::bridge::TaskSchedCache::from_thread(&task)
            .with_wake_cpu(Some(safe_cpu))
            .into_sched_fields(task.id);
        self.state.insert_task(sched_fields);
        let boxed_task = alloc::boxed::Box::new(task);
        if crate::sched::scheduler_lock_held_by_this_cpu::<R>() {
            self.pending_registry_inserts.push(boxed_task);
        } else {
            crate::task::registry::get_registry::<R>().insert(boxed_task);
        }

        let current_cpu = crate::sched::current_cpu_index::<R>();
        let now_tick = crate::sched::TICK_COUNT.load(Ordering::Relaxed);
        let wake_mono = crate::runtime::<R>().mono_ticks();
        if safe_cpu == current_cpu {
            self.state.enqueue_task(safe_cpu, priority as usize, id);
        } else {
            crate::sched::enqueue_remote_wake_mailbox(
                safe_cpu,
                crate::sched::types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: priority as usize,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }

        if safe_cpu == current_cpu {
            self.state.per_cpu[safe_cpu].need_resched = true;
        }
        // GLOBAL_NEED_RESCHED and the actual IPI send for remote CPUs are
        // handled post-lock by nudge_spawned_task.

        let _parent_tid = self.state.per_cpu[current_cpu].current;
        // Link affinity and initial location
        let _ = affinity; // affinity is captured in the task; no additional bookkeeping needed
        // Initial location matches target runq

        Some(id)
    }

    /// Create a new user task in the `Blocked` state without enqueuing it.
    ///
    /// Used by spawn functions that need to complete post-spawn setup
    /// (process_info, name, inherited FDs) outside the SCHEDULER lock before
    /// making the task runnable.  The caller MUST call `wake_task(id)` after
    /// setup is complete.
    ///
    /// Unlike [`spawn_user_task`], this variant:
    /// - Does not call `current_parent_pid` (avoids nested REGISTRY lock)
    /// - Creates the task with `process_info: None` (caller sets it outside lock)
    /// - Does not enqueue the task or set `need_resched`
    pub fn spawn_user_task_deferred(
        &mut self,
        entry: crate::UserEntry,
        aspace: <R::Tasking as BootTasking>::AddressSpace,
        stack_info: abi::types::StackInfo,
        regions: alloc::vec::Vec<abi::vm::VmRegionInfo>,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
    ) -> Option<TaskId> {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            return None;
        }
        let stack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking().init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let mapping_list = crate::memory::mappings::MappingList { regions };
        let mappings_arc = alloc::sync::Arc::new(spin::Mutex::new(mapping_list));

        let target_cpu = self.pick_cpu_and_bringup(affinity, true);
        let cpu_count = self.state.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };

        let task: Task<R> = Task {
            id,
            // Start blocked so the caller can finish setup before the task runs.
            state: TaskState::Blocked,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: Some(stack_info),
            mappings: mappings_arc,
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            // process_info is None; the caller sets it outside the SCHEDULER lock.
            process_info: None,
            enqueued_at_tick: crate::sched::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let sched_fields = crate::sched::bridge::TaskSchedCache::from_thread(&task)
            .with_wake_cpu(Some(safe_cpu))
            .into_sched_fields(task.id);
        self.state.insert_task(sched_fields);
        let boxed_task = alloc::boxed::Box::new(task);
        if crate::sched::scheduler_lock_held_by_this_cpu::<R>() {
            self.pending_registry_inserts.push(boxed_task);
        } else {
            // Insert into the registry so the task can be looked up by TID.
            // The task is Blocked and not in any runqueue; it cannot be scheduled
            // until the caller calls wake_task(id).
            crate::task::registry::get_registry::<R>().insert(boxed_task);
        }

        Some(id)
    }
}

pub fn spawn<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
    affinity: crate::task::Affinity,
) -> TaskId {
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let _irq = rt.irq_disable();
    let (id, in_bringup, deferred_registry_inserts) =
        with_spawn_scheduler_lock::<R, _>(current_cpu, |sched| {
            let id = sched.spawn(entry, arg, priority, affinity);
            // Capture before releasing the lock so nudge is coherent with placement.
            let in_bringup = sched.bringup_in_progress;
            let deferred_registry_inserts = sched.drain_pending_registry_inserts();
            (id, in_bringup, deferred_registry_inserts)
        });
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    // Skip remote wakeup IPIs during early-boot bringup.  Tasks placed on the
    // local CPU will be picked up naturally by the scheduler loop; deferred
    // tasks on remote CPUs will be woken when end_bringup() is called.
    if !in_bringup {
        nudge_spawned_task::<R>(current_cpu, id);
    }
    rt.irq_restore(_irq);
    id
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
) -> TaskId {
    spawn::<R>(entry, arg, priority, crate::task::Affinity::Any)
}

pub unsafe fn spawn_user_thread<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: StartupArg,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
) -> TaskId {
    unsafe { spawn_user_thread_ex::<R>(entry, stack, arg, stack_info, priority, 0, false) }
}

/// Extended version of `spawn_user_thread` with explicit TLS base and detached flag.
pub unsafe fn spawn_user_thread_ex<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: StartupArg,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
    tls_base: u64,
    detached: bool,
) -> TaskId {
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let _irq = rt.irq_disable();
    let (id, in_bringup, deferred_registry_inserts) =
        with_spawn_scheduler_lock::<R, _>(current_cpu, |sched| {
            let id = sched.spawn_user_thread(
                entry,
                stack,
                arg,
                stack_info,
                priority,
                crate::task::Affinity::Any,
                tls_base,
                detached,
            );
            let in_bringup = sched.bringup_in_progress;
            let deferred_registry_inserts = sched.drain_pending_registry_inserts();
            (id, in_bringup, deferred_registry_inserts)
        });
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    if !in_bringup {
        nudge_spawned_task::<R>(current_cpu, id);
    }
    rt.irq_restore(_irq);
    id
}

pub unsafe fn spawn_user_task_full<R: BootRuntime>(
    entry: UserEntry,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    stack_info: abi::types::StackInfo,
    regions: alloc::vec::Vec<abi::vm::VmRegionInfo>,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let _irq = rt.irq_disable();
    let (id, in_bringup, deferred_registry_inserts) =
        with_spawn_scheduler_lock::<R, _>(current_cpu, |sched| {
            let id = sched.spawn_user_task(
                entry,
                aspace,
                stack_info,
                regions,
                priority,
                crate::task::Affinity::Any,
            );
            let in_bringup = sched.bringup_in_progress;
            let deferred_registry_inserts = sched.drain_pending_registry_inserts();
            (id, in_bringup, deferred_registry_inserts)
        });
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    if let Some(id) = id {
        if !in_bringup {
            nudge_spawned_task::<R>(current_cpu, id);
        }
        rt.irq_restore(_irq);
        Some(id)
    } else {
        rt.irq_restore(_irq);
        None
    }
}

fn nudge_spawned_task<R: BootRuntime>(current_cpu: usize, id: TaskId) {
    let Some(target_cpu) = crate::task::registry::get_task::<R>(id)
        .map(|task| crate::sched::bridge::SchedulableRuntime::from_thread(&*task).last_cpu)
        .flatten()
    else {
        return;
    };

    if target_cpu != current_cpu {
        // Set the per-CPU pending flag.  If it was already set a previous IPI
        // is in flight (or another spawn just set it for the same CPU); in that
        // case the target will process this task when it handles that IPI, so
        // we suppress the duplicate send.
        let already_pending = crate::sched::set_global_need_resched(target_cpu);
        if !already_pending {
            if !crate::sched::should_send_remote_resched_ipi(target_cpu) {
                return;
            }
            crate::kdebug!(
                "SCHED: Sending post-unlock Resched IPI to CPU {} for task {}",
                target_cpu,
                id
            );
            crate::sched::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::sched::DIAG_IPI_SENT_SPAWN.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::runtime::<R>().send_ipi(target_cpu, 0x30);
        } else {
            crate::sched::PROF_IPI_SUPPRESSED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
    }
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    let rt = crate::runtime::<R>();
    crate::sched::set_cpu_current_task(rt.current_cpu_index(), rt.current_tid());
    let entry_ptr = arg as *mut UserEntry;
    let entry = unsafe { *alloc::boxed::Box::from_raw(entry_ptr) };

    crate::kdebug!(
        "USER_TRAMPOLINE: PC=0x{:x} SP=0x{:x} ARG0=0x{:x}",
        entry.entry_pc,
        entry.user_sp,
        entry.arg0
    );

    unsafe { rt.tasking().enter_user(entry) }
}

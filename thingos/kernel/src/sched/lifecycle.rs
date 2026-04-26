//! Task lifecycle management: exit, wait, kill, signal, CPU management.
use super::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};
use core::sync::atomic::Ordering;

/// Transition the scheduler out of early-boot mode.
///
/// During early boot (`bringup_in_progress == true`) the scheduler places all
/// `Affinity::Any` tasks on the local (boot) CPU and suppresses remote-wakeup
/// IPIs so that service bring-up incurs minimal cross-CPU coordination.
///
/// Call this function once all initial services have been spawned and the
/// system is ready to enter steady-state scheduling.  After this point the
/// normal round-robin CPU selection and IPI delivery resume.
pub fn end_bringup<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        if sched.bringup_in_progress {
            crate::kdebug!(
                "SCHED: early-boot bringup complete; resuming steady-state SMP scheduling"
            );
            sched.bringup_in_progress = false;
        }
    }
    rt.irq_restore(_irq);
}

pub fn set_priority<R: BootRuntime>(id: TaskId, priority: TaskPriority) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let updated = {
        let lock = SCHEDULER.lock();
        let updated = if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            sched.set_priority_hot_cache(id, priority)
        } else {
            false
        };
        updated
    };
    if updated {
        debug_assert_scheduler_not_held_by_this_cpu::<R>("set_priority registry sync");
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
            task.priority = priority;
            task.base_priority = priority;
        }
    }
    rt.irq_restore(_irq);
}

pub fn task_status<R: BootRuntime>(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    // Task state and exit code live in the registry, not the scheduler.
    // Holding SCHEDULER here was unnecessary and caused timer-ISR try_lock
    // misses on all other CPUs (the supervisor polls every task every cycle).
    PROF_TASK_STATUS_POLLS.fetch_add(1, Ordering::Relaxed);
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let res = crate::task::registry::get_task::<R>(id).map(|t| (t.state, t.exit_code));
    rt.irq_restore(_irq);
    res
}

pub fn current_priority<R: BootRuntime>() -> TaskPriority {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let res = if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let cpu = current_cpu_index::<R>();
            sched
                .state
                .per_cpu
                .get(cpu)
                .and_then(|pc| pc.current)
                .and_then(|tid| sched.state.get_thread(tid))
                .map(|sf| sf.priority)
                .unwrap_or(TaskPriority::Normal)
        } else {
            TaskPriority::Normal
        }
    } else {
        TaskPriority::Normal
    };
    rt.irq_restore(_irq);
    res
}

pub fn current_tid<R: BootRuntime>() -> u64 {
    crate::runtime::<R>().current_tid()
}

fn effective_parallelism_from_state(online_cpu_count: usize, affinity: Affinity) -> usize {
    let online = online_cpu_count.max(1);
    match affinity {
        Affinity::Pinned(_) => 1,
        Affinity::Any => online,
        Affinity::Restricted(ref aff) => aff.effective_parallelism(online),
    }
}

pub fn available_parallelism<R: BootRuntime>() -> usize {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let result = if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let online = sched.state.online_cpu_count;
            let affinity = crate::task::registry::get_task::<R>(rt.current_tid())
                .map(|task| bridge::SchedulableRuntime::from_thread(&*task).affinity)
                .unwrap_or(Affinity::Any);
            effective_parallelism_from_state(online, affinity)
        } else {
            1
        }
    } else {
        1
    };

    rt.irq_restore(_irq);
    result.max(1)
}

pub(super) fn current_task_name_impl<R: BootRuntime>() -> [u8; 32] {
    let tid = current_tid::<R>();
    if let Some(task) = crate::task::registry::get_task::<R>(tid) {
        task.name
    } else {
        let mut n = [0u8; 32];
        n[0..7].copy_from_slice(b"unknown");
        n
    }
}

/// Update the current task's stored `user_fs_base` field.
///
/// Called by the TLS-set syscall handler after writing the hardware register,
/// so that the value is saved correctly on the next context switch without
/// needing an extra MSR read.
pub(super) fn set_current_user_fs_base<R: BootRuntime>(base: u64) {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.user_fs_base = base;
    }
}

/// Return the current task's stored `user_fs_base` field.
pub(super) fn current_user_fs_base<R: BootRuntime>() -> u64 {
    let tid = crate::runtime::<R>().current_tid();
    crate::task::registry::get_task::<R>(tid).map(|task| task.user_fs_base).unwrap_or(0)
}

/// Update the calling thread's human-readable name.
///
/// Called by `SYS_TASK_SET_NAME`.  Defensively clamps to 31 bytes even
/// though the syscall handler already enforces this limit, so that the
/// function stays safe if called from other internal paths in the future.
pub(super) fn set_current_task_name<R: BootRuntime>(ptr: *const u8, len: usize) {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        let len = len.min(31);
        // SAFETY: `ptr` points to a kernel buffer that was copied from user
        // space by the syscall handler before this hook is called.
        let src = unsafe { core::slice::from_raw_parts(ptr, len) };
        task.name[..len].copy_from_slice(src);
        task.name_len = len as u8;
    }
}

pub(super) fn interrupt_task<R: BootRuntime>(tid: TaskId) -> Result<(), abi::errors::Errno> {
    let should_wake = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        if task.state == TaskState::Dead {
            return Err(abi::errors::Errno::ESRCH);
        }
        task.pending_interrupt = true;
        task.state == TaskState::Blocked
    } else {
        return Err(abi::errors::Errno::ESRCH);
    };

    if should_wake {
        wake_task::<R>(tid);
    }

    Ok(())
}

pub(super) fn take_pending_interrupt<R: BootRuntime>() -> bool {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        let was_pending = task.pending_interrupt;
        task.pending_interrupt = false;
        was_pending
    } else {
        false
    }
}

/// Get the current task's ProcessInfo Arc, if any.
pub fn process_info<R: BootRuntime>()
-> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Prefer the runtime's current TID. During syscall/trap handling this stays
    // authoritative even if the scheduler's per-CPU `current` view is transiently stale.
    let runtime_tid = rt.current_tid();
    let mut maybe_process_info =
        crate::task::registry::get_task::<R>(runtime_tid).and_then(|t| t.process_info.clone());
    if maybe_process_info.is_none() {
        let scheduler_current_tid = {
            let lock = SCHEDULER.lock();
            if let Some(ptr) = *lock {
                let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
                let cpu_idx = current_cpu_index::<R>();
                sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current)
            } else {
                None
            }
        };
        maybe_process_info = scheduler_current_tid
            .and_then(|tid| crate::task::registry::get_task::<R>(tid))
            .and_then(|t| t.process_info.clone());
    }

    rt.irq_restore(_irq);
    maybe_process_info
}

pub fn process_info_for_tid<R: BootRuntime>(
    tid: u64,
) -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let result = crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());
    rt.irq_restore(_irq);
    result
}

pub fn process_info_for_pid<R: BootRuntime>(
    pid: u32,
) -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let reg = crate::task::registry::get_registry::<R>();

    let mut candidate: Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> = None;
    for task in reg.threads.iter() {
        let Some(pi_arc) = &task.process_info else {
            continue;
        };
        if pi_arc.lock().pid != pid {
            continue;
        }

        if task.id == pid as u64 {
            candidate = Some(pi_arc.clone());
            break;
        }

        if candidate.is_none() {
            candidate = Some(pi_arc.clone());
        }
    }

    drop(reg);
    rt.irq_restore(_irq);
    candidate
}

/// Return a snapshot of all live processes (those with a ProcessInfo).
///
/// Called from the `LIST_PROCESSES_HOOK` slot so that procfs can render
/// `/proc/<pid>/…` files without knowing the concrete `R` type.
pub fn list_processes<R: BootRuntime>() -> alloc::vec::Vec<hooks::ProcessSnapshot> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let mut out = alloc::vec::Vec::new();
    let foreground_pgid = crate::vfs::devfs::console_foreground_pgid();
    {
        let reg = crate::task::registry::get_registry::<R>();

        // Phase 9: build a TID→state map so each process snapshot can carry the
        // full set of thread states from `ProcessLifecycle.thread_ids`.  This
        // allows `kernel::job::bridge::job_state_from_snapshot` to give accurate
        // `JobState` for multi-threaded processes rather than only seeing the
        // thread-group leader's state.
        let mut tid_state: alloc::collections::BTreeMap<TaskId, TaskState> =
            alloc::collections::BTreeMap::new();
        for task in reg.threads.iter() {
            tid_state.insert(task.id, task.state);
        }

        for task in reg.threads.iter() {
            if let Some(pi_arc) = &task.process_info {
                let pi = pi_arc.lock();
                let name_bytes = &task.name[..task.name_len as usize];
                let name = alloc::string::String::from_utf8_lossy(name_bytes).into_owned();
                out.push(pi.compatibility_snapshot_for_task(
                    task.id,
                    name,
                    task.state,
                    task.exit_code,
                    foreground_pgid,
                    &tid_state,
                ));
            }
        }
    }
    rt.irq_restore(_irq);
    out
}

/// Return the current PID membership snapshot for one Unix-compat process group.
///
/// Unlike [`list_processes`], this helper never waits on a `ProcessInfo` mutex
/// while the task registry is held. It first snapshots unique `ProcessInfo`
/// Arcs from the registry, then drops the registry lock before inspecting
/// `unix_compat.pgid`. This avoids registry -> process lock inversion on
/// latency-sensitive paths such as TTY-generated `SIGINT` fanout.
pub fn list_process_ids_by_pgid<R: BootRuntime>(pgid: u32) -> alloc::vec::Vec<u32> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let process_infos = {
        let reg = crate::task::registry::get_registry::<R>();
        let mut seen = alloc::collections::BTreeSet::new();
        let mut infos = alloc::vec::Vec::new();

        for task in reg.threads.iter() {
            let Some(pi_arc) = &task.process_info else {
                continue;
            };
            let key = alloc::sync::Arc::as_ptr(pi_arc) as usize;
            if seen.insert(key) {
                infos.push(pi_arc.clone());
            }
        }

        infos
    };

    rt.irq_restore(_irq);

    let mut members = alloc::vec::Vec::new();
    for pinfo in process_infos {
        let process = pinfo.lock();
        if process.unix_compat.pgid == pgid {
            members.push(process.pid);
        }
    }

    members
}

fn register_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    // Joining a detached thread is not permitted.
    if target.detached {
        return Err(abi::errors::Errno::EINVAL);
    }

    if let Some(pinfo) = target.process_info.as_ref() {
        let pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            if let Some(code) = pi.job.leader_exit_code {
                return Ok(Some(code));
            }
            pi.job.leader_exit_waiters.push_back(waiter_tid);
            return Ok(None);
        }
    }

    if target.state == TaskState::Dead {
        return Ok(Some(target.exit_code.unwrap_or(0)));
    }

    target.exit_waiters.push_back(waiter_tid);
    Ok(None)
}

pub fn poll_task_exit<R: BootRuntime>(
    target_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    if let Some(pinfo) = target.process_info.as_ref() {
        let pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            if let Some(code) = pi.job.leader_exit_code {
                return Ok(Some(code));
            }
            return Ok(None);
        }
    }

    if target.state == TaskState::Dead { Ok(Some(target.exit_code.unwrap_or(0))) } else { Ok(None) }
}

pub fn register_task_exit_waiter_public<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    register_task_exit_waiter::<R>(target_tid, waiter_tid)
}

pub fn unregister_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<(), abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;
    if let Some(pinfo) = target.process_info.as_ref() {
        let pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            pi.job.leader_exit_waiters.remove(waiter_tid);
            return Ok(());
        }
    }
    target.exit_waiters.remove(waiter_tid);
    Ok(())
}

struct TerminationRegistryOutcome {
    waiters: alloc::vec::Vec<u64>,
    siblings_to_kill: alloc::vec::Vec<TaskId>,
    auto_reap: bool,
}

fn mark_task_exited_in_registry<R: BootRuntime>(
    tid: TaskId,
    code: i32,
) -> TerminationRegistryOutcome {
    let mut auto_reap = false;
    debug_assert_scheduler_not_held_by_this_cpu::<R>("mark_task_exited_in_registry");
    if tid == 6 {
        crate::kdebug!("SCHED[TID6]: exited (code={})", code);
    }
    // Collect exit waiters and mark the task dead.
    let mut waiters = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.state = TaskState::Dead;
        task.exit_code = Some(code);
        auto_reap = task.detached;
        task.exit_waiters.drain()
    } else {
        alloc::vec::Vec::new()
    };

    // Remove this TID from the process's thread group list.
    // If this is the thread-group leader, drain the remaining siblings in one
    // step to avoid a separate clone + clear pass.
    // Also capture ppid/pid for lifecycle status queueing.
    let mut notify_ppid: u32 = 0;
    let mut notify_pid: u32 = 0;
    // Capture the exit observer inbox ID (if set) for canonical JobExit delivery.
    let mut exit_observer_inbox: Option<crate::inbox::InboxId> = None;

    let mut siblings_to_kill: alloc::vec::Vec<TaskId> = {
        let pinfo_opt =
            crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());
        if let Some(pinfo) = pinfo_opt {
            let mut pi = pinfo.lock();
            pi.remove_thread_from_job(tid);

            // If the exiting thread is the thread-group leader (its TID == pid),
            // drain all remaining siblings and schedule them for termination.
            if pi.is_job_leader_tid(tid) {
                notify_ppid = pi.runtime_parent_pid();
                notify_pid = pi.runtime_pid();
                exit_observer_inbox = pi.job_exit_observer_inbox();
                if pi.job.leader_exit_code.is_none() {
                    waiters.extend(pi.job.complete_leader_exit(code));
                }
                pi.take_job_thread_ids()
            } else {
                alloc::vec::Vec::new()
            }
        } else {
            alloc::vec::Vec::new()
        }
    };

    // Project leader-exit lifecycle semantics through the canonical Job bridge
    // (Unix wait status queue + optional JobExit observer notification).
    let parent_waiters =
        crate::job::bridge::publish_leader_exit(notify_ppid, notify_pid, code, exit_observer_inbox);
    waiters.extend(parent_waiters);

    // If this process was a thread-group leader, its exit orphans its children.
    // Reparent all children whose ppid matches this dying process to init (PID 1).
    if notify_pid != 0 {
        let mut registry = crate::task::registry::get_registry::<R>();
        for task in registry.threads.iter_mut() {
            if let Some(pinfo) = &task.process_info {
                let mut pi = pinfo.lock();
                if pi.job.ppid == notify_pid {
                    pi.job.ppid = 1;
                }
            }
        }
    }

    // Kill sibling threads (thread-group exit).
    for &sibling in &siblings_to_kill {
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(sibling) {
            if task.state != TaskState::Dead {
                task.state = TaskState::Dead;
                task.exit_code = Some(code);
                let sibling_waiters = task.exit_waiters.drain();
                waiters.extend(sibling_waiters);
                crate::kdebug!("SCHED: Killed sibling thread {} (thread-group exit)", sibling);
            }
        }
    }

    siblings_to_kill.retain(|&sibling| sibling != tid);

    TerminationRegistryOutcome { waiters, siblings_to_kill, auto_reap }
}

fn mark_task_exited<R: BootRuntime>(
    sched: &mut types::Scheduler<R>,
    tid: TaskId,
    code: i32,
) -> alloc::vec::Vec<u64> {
    let termination = mark_task_exited_in_registry::<R>(tid, code);

    if let Some(task) = sched.state.get_task_mut(tid) {
        task.runq_location = None;
        task.state = TaskState::Dead;
    }
    purge_task_from_scheduler_queues::<R>(sched, tid);

    for &sibling in &termination.siblings_to_kill {
        if sibling == tid {
            continue;
        }
        if let Some(task) = sched.state.get_task_mut(sibling) {
            task.runq_location = None;
            task.state = TaskState::Dead;
        }
        purge_task_from_scheduler_queues::<R>(sched, sibling);
    }

    termination.waiters
}

pub(super) fn purge_task_from_scheduler_queues<R: BootRuntime>(sched: &mut types::Scheduler<R>, tid: TaskId) {
    sched.state.remove_task_from_runq(tid);
    sched.state.unregister_waiter(tid);

    let _ = sched.state.remove_task_from_sleep_queue(tid);
}

fn wake_waiters(waiters: &[u64]) {
    for &tid in waiters {
        unsafe {
            crate::sched::wake_task_erased(tid);
        }
    }
}

fn release_task_devices<R: BootRuntime>(tid: TaskId) {
    debug_assert_scheduler_not_held_by_this_cpu::<R>("release_task_devices");
    let released = crate::device_registry::REGISTRY.lock().release_all_for_task(tid);
    if released > 0 {
        crate::kdebug!("DEVICE: released {} claims for task {}", released, tid);
    }
}

pub(super) fn current_task_resource_id_impl<R: BootRuntime>() -> Option<u64> {
    None
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let current_tid = rt.current_tid();
    // Lock-order policy: perform REGISTRY / DEVICE_REGISTRY exit cleanup before
    // taking SCHEDULER, then run scheduler-only termination under SCHEDULER.
    let termination = mark_task_exited_in_registry::<R>(current_tid, code);
    release_task_devices::<R>(current_tid);

    let (
        switch_decision,
        deferred_prepare_ipis,
        deferred_registry_syncs,
        deferred_registry_inserts,
    ) = {
        let lock = SCHEDULER.lock();
        set_sched_lock_tracking::<R>(rt.current_cpu_index());
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let switch_decision = sched.terminate_current(current_tid, &termination.siblings_to_kill);
        if termination.auto_reap {
            sched.state.remove_task(current_tid);
        }
        let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
        let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
        let deferred_registry_inserts = sched.drain_pending_registry_inserts();
        clear_sched_lock_tracking::<R>();
        (switch_decision, deferred_prepare_ipis, deferred_registry_syncs, deferred_registry_inserts)
    };

    if termination.auto_reap {
        // Same drop-order fix as in `remove_task_completely`: bind to a `let`
        // so the RegistryGuard (REGISTRY lock) is released at the semicolon
        // before the Box<Thread> is dropped at end of block.
        let _removed = crate::task::registry::get_registry::<R>().remove(current_tid);
        // REGISTRY lock released here.  _removed dropped at end of block.
    }

    send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
    apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    wake_waiters(&termination.waiters);
    let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
    let mut ghost_fs_base = 0;
    let switch = resolve_switch_params::<R>(
        switch_decision,
        &mut ghost_ctx,
        &mut ghost_fs_base,
    ).unwrap_or_else(|| {
        panic!(
            "scheduler invariant violated: terminate_current produced switch decision (from={}, to={}) but registry lookup failed",
            switch_decision.from_tid, switch_decision.to_tid
        )
    });

    // Pre-switch: update CPU_CURRENT_TASK to prevent cross-CPU deadlock.
    let exit_cpu = current_cpu_index::<R>();
    crate::sched::set_cpu_current_task(exit_cpu, switch.to_tid);

    let mut _spins = 0u32;
    while crate::sched::is_task_on_any_cpu(switch.to_tid) {
        _spins += 1;
        if _spins > 10_000 {
            break;
        }
        core::hint::spin_loop();
    }

    if switch.to_aspace != switch.from_aspace {
        rt.tasking().activate_address_space(switch.to_aspace);
    }

    unsafe {
        rt.tasking().switch_with_tls(
            &mut *(switch.from_ctx as *mut _),
            &*switch.to_ctx,
            switch.to_tid,
            switch.from_user_fs_base,
            switch.to_user_fs_base,
        );
    }

    unreachable!("Thread continued after terminating!");
}

/// Kill an arbitrary task by TID. Returns true if the task was found and killed.
/// The task is marked Dead with exit code -9 and removed from all run queues.
pub fn kill_by_tid<R: BootRuntime>(tid: u64) -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let (killed, waiters) = {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };

            // Don't allow killing the current task via this path
            let cpu_idx = current_cpu_index::<R>();
            if let Some(current_id) = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
                if current_id == tid {
                    (false, alloc::vec::Vec::new())
                } else {
                    let task_is_dead = sched
                        .state
                        .get_thread(tid)
                        .map(|task| task.state == TaskState::Dead)
                        .unwrap_or(true);

                    if task_is_dead {
                        (false, alloc::vec::Vec::new())
                    } else {
                        let waiters = mark_task_exited::<R>(sched, tid, -9);

                        // Release any claimed devices
                        let released =
                            crate::device_registry::REGISTRY.lock().release_all_for_task(tid);
                        if released > 0 {
                            crate::kdebug!("DEVICE: released {} claims for task {}", released, tid);
                        }

                        (true, waiters)
                    }
                }
            } else {
                (false, alloc::vec::Vec::new())
            }
        } else {
            (false, alloc::vec::Vec::new())
        }
    };

    wake_waiters(&waiters);
    rt.irq_restore(_irq);
    killed
}

/// Wait semantics are non-consuming today: any task that can name a TID may
/// observe its terminal status, and dead task records stay resident for later polls/waits.
pub fn wait_task<R: BootRuntime>(tid: TaskId) -> Result<i32, abi::errors::Errno> {
    let current_tid = current_tid::<R>();

    if tid == current_tid {
        return Err(abi::errors::Errno::EINVAL);
    }

    loop {
        if let Some(code) = register_task_exit_waiter::<R>(tid, current_tid)? {
            return Ok(code);
        }

        unsafe {
            block_current_erased();
        }

        if take_pending_interrupt::<R>() {
            let _ = unregister_task_exit_waiter::<R>(tid, current_tid);
            return Err(abi::errors::Errno::EINTR);
        }
    }
}

/// Collect TIDs of all tasks that are children of `our_pid`.
///
/// If `target_pid > 0`, only the specific child with that PID is returned.
/// Otherwise, all direct children are returned.
fn collect_child_tids<R: BootRuntime>(our_pid: u32, target_pid: i64) -> alloc::vec::Vec<TaskId> {
    let reg = crate::task::registry::get_registry::<R>();
    reg.threads
        .iter()
        .filter_map(|task| {
            task.process_info.as_ref().and_then(|pi| {
                let pi = pi.lock();
                if pi.runtime_parent_pid() != our_pid {
                    return None;
                }
                if target_pid > 0 && pi.runtime_pid() != target_pid as u32 {
                    return None;
                }
                Some(task.id)
            })
        })
        .collect()
}

fn queued_status_matches(flags: u32, status: i32) -> bool {
    use abi::signal::{wifcontinued, wifstopped};
    use abi::types::waitpid_flags;

    if wifstopped(status) {
        return (flags & waitpid_flags::WUNTRACED) != 0;
    }
    if wifcontinued(status) {
        return (flags & waitpid_flags::WCONTINUED) != 0;
    }
    true
}

fn reap_child_pid_if_dead<R: BootRuntime>(child_pid: u32, status: i32) {
    if abi::signal::wifstopped(status) || abi::signal::wifcontinued(status) {
        return;
    }

    let reg = crate::task::registry::get_registry::<R>();
    let child_tid = reg.threads.iter().find_map(|task| {
        task.process_info
            .as_ref()
            .filter(|pi| pi.lock().runtime_pid() == child_pid)
            .map(|_| task.id)
    });
    drop(reg);

    if let Some(child_tid) = child_tid {
        remove_task_completely::<R>(child_tid);
    }
}

fn take_queued_child_status<R: BootRuntime>(
    _our_pid: u32,
    target_pid: i64,
    flags: u32,
) -> Option<(u64, i32)> {
    let our_process = crate::sched::process_info_current()?;
    let mut process = our_process.lock();
    let mut match_index: Option<usize> = None;

    for (idx, (child_pid, status)) in process.job.children_done.iter().enumerate() {
        if target_pid > 0 && *child_pid != target_pid as u32 {
            continue;
        }
        if queued_status_matches(flags, *status) {
            match_index = Some(idx);
            break;
        }
    }

    let (child_pid, status) = process.job.children_done.remove(match_index?)?;
    drop(process);
    reap_child_pid_if_dead::<R>(child_pid, status);
    Some((child_pid as u64, status))
}

/// Internal implementation: performs the wait with an explicit `our_pid`.
///
/// Factored out so tests can exercise the core logic without registering a
/// task at the current-TID slot (which is always 0 in the `MockRuntime`).
fn waitpid_for_pid<R: BootRuntime>(
    our_pid: u32,
    pid: i64,
    flags: u32,
) -> Result<(u64, i32), abi::errors::Errno> {
    use abi::types::waitpid_flags;
    let wnohang = (flags & waitpid_flags::WNOHANG) != 0;

    let our_tid = current_tid::<R>();

    loop {
        if let Some(result) = take_queued_child_status::<R>(our_pid, pid, flags) {
            crate::ktrace!(
                "waitpid: queued status delivered parent_pid={} parent_tid={} target_pid={} child_pid={} status=0x{:x} flags=0x{:x}",
                our_pid,
                our_tid,
                pid,
                result.0,
                result.1 as u32,
                flags
            );
            return Ok(result);
        }

        // Collect matching children (releases registry guard before returning).
        let children = collect_child_tids::<R>(our_pid, pid);

        if children.is_empty() {
            return Err(abi::errors::Errno::ECHILD);
        }

        // Fast path: look for a dead child without registering.
        for &child_tid in &children {
            // Collect exit info without holding the registry guard across the reap call.
            let dead_info = crate::task::registry::get_task::<R>(child_tid).and_then(|task| {
                // `task` (ThreadRef / registry guard) is dropped when this closure returns.
                if task.state == TaskState::Dead {
                    let (child_pid, code) = task
                        .process_info
                        .as_ref()
                        .map(|pinfo| {
                            let pi_guard = pinfo.lock();
                            let code =
                                pi_guard.effective_exit_code_for_tid(child_tid, task.exit_code);
                            (pi_guard.runtime_pid() as u64, code.unwrap_or(0))
                        })
                        .unwrap_or((child_tid, task.exit_code.unwrap_or(0)));
                    Some((child_pid, code))
                } else {
                    None
                }
            });

            if let Some((child_pid, code)) = dead_info {
                // Reap: remove the dead child's record from both the registry and
                // the scheduler state so they stay in sync.
                remove_task_completely::<R>(child_tid);
                return Ok((child_pid, code));
            }
        }

        if wnohang {
            // POSIX: return 0 as the child PID to indicate "no child exited yet".
            return Ok((0, 0));
        }

        // Register as an exit waiter for every live child.  If any child has
        // already died between the check above and the register call,
        // `register_task_exit_waiter` returns `Some(code)` immediately.
        let mut registered: alloc::vec::Vec<TaskId> = alloc::vec::Vec::new();
        let mut early_result: Option<(u64, i32)> = None;
        let mut early_reap_tid: Option<TaskId> = None;

        for &child_tid in &children {
            match register_task_exit_waiter::<R>(child_tid, our_tid) {
                Ok(Some(code)) => {
                    // Child died between our fast-path check and now.
                    let child_pid = crate::task::registry::get_task::<R>(child_tid)
                        .and_then(|t| {
                            t.process_info.as_ref().map(|pi| pi.lock().runtime_pid() as u64)
                        })
                        .unwrap_or(child_tid);
                    early_result = Some((child_pid, code));
                    early_reap_tid = Some(child_tid);
                    break;
                }
                Ok(None) => registered.push(child_tid),
                Err(_) => {} // Child vanished — skip it.
            }
        }

        if let Some(result) = early_result {
            // Clean up any waiters we already registered before finding the dead child.
            for &child_tid in &registered {
                let _ = unregister_task_exit_waiter::<R>(child_tid, our_tid);
            }
            // Reap the dead child.
            if let Some(reap_tid) = early_reap_tid {
                remove_task_completely::<R>(reap_tid);
            }
            return Ok(result);
        }

        if registered.is_empty() {
            // All children died in the window between collection and registration.
            crate::ktrace!(
                "waitpid: no registrations parent_pid={} parent_tid={} target_pid={} flags=0x{:x}; retrying",
                our_pid,
                our_tid,
                pid,
                flags
            );
            continue;
        }

        crate::ktrace!(
            "waitpid: blocking parent_pid={} parent_tid={} target_pid={} flags=0x{:x} registered_children={}",
            our_pid,
            our_tid,
            pid,
            flags,
            registered.len()
        );

        // Block until any registered child exits.
        unsafe {
            block_current_erased();
        }

        let interrupted = take_pending_interrupt::<R>();

        crate::ktrace!(
            "waitpid: woke parent_pid={} parent_tid={} target_pid={} flags=0x{:x} interrupted={}",
            our_pid,
            our_tid,
            pid,
            flags,
            interrupted
        );

        // After waking, unregister from children that haven't yet exited.
        for &child_tid in &registered {
            let _ = unregister_task_exit_waiter::<R>(child_tid, our_tid);
        }

        if interrupted {
            return Err(abi::errors::Errno::EINTR);
        }

        // Loop back to find the next queued child state transition.
    }
}

/// Wait for a child process to exit, returning `(child_pid, exit_code)`.
///
/// - `pid > 0`: wait for the specific child with that PID.
/// - `pid == -1` or `pid == 0`: wait for any child of the calling process.
/// - `flags & WNOHANG`: return `Ok((0, 0))` immediately if no child has exited.
///
/// Returns `Err(ECHILD)` when no matching children exist at all.
pub fn waitpid<R: BootRuntime>(pid: i64, flags: u32) -> Result<(u64, i32), abi::errors::Errno> {
    let our_tid = current_tid::<R>();

    // Retrieve the calling process's PID from its ProcessInfo.
    let our_pid = {
        let task =
            crate::task::registry::get_task::<R>(our_tid).ok_or(abi::errors::Errno::EINVAL)?;
        task.process_info
            .as_ref()
            .map(|pi| pi.lock().runtime_pid())
            .ok_or(abi::errors::Errno::EINVAL)?
    };

    waitpid_for_pid::<R>(our_pid, pid, flags)
}

// ── Signal mask hooks ─────────────────────────────────────────────────────────

pub(super) fn get_signal_mask<R: BootRuntime>() -> abi::signal::SigSet {
    let tid = current_tid::<R>();
    crate::task::registry::get_task::<R>(tid)
        .map(|t| t.signals.effective_mask())
        .unwrap_or(abi::signal::SigSet::EMPTY)
}

pub(super) fn set_signal_mask<R: BootRuntime>(mask: abi::signal::SigSet) {
    let tid = current_tid::<R>();
    if let Some(mut t) = crate::task::registry::get_task_mut::<R>(tid) {
        t.signals.mask = abi::signal::SigSet(mask.0 & !crate::signal::UNCATCHABLE.0);
    }
}

pub(super) fn get_thread_pending<R: BootRuntime>() -> abi::signal::SigSet {
    let tid = current_tid::<R>();
    crate::task::registry::get_task::<R>(tid)
        .map(|t| t.signals.pending)
        .unwrap_or(abi::signal::SigSet::EMPTY)
}

pub(super) fn set_thread_pending<R: BootRuntime>(pending: abi::signal::SigSet) {
    let tid = current_tid::<R>();
    if let Some(mut t) = crate::task::registry::get_task_mut::<R>(tid) {
        t.signals.pending = pending;
    }
}

pub fn register_timeout_wake<R: BootRuntime>(tid: TaskId, wake_tick: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.state.add_task_to_sleep_queue(tid, wake_tick);
    }
    rt.irq_restore(_irq);
}

pub fn unregister_timeout_wake<R: BootRuntime>(tid: TaskId) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let _ = sched.state.remove_task_from_sleep_queue(tid);
    }
    rt.irq_restore(_irq);
}

pub fn cpu_online<R: BootRuntime>(cpu_index: usize) {
    let rt = crate::runtime::<R>();
    let current_cpu = rt.current_cpu_index();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let mut deferred_registry_inserts = alloc::vec::Vec::new();
    set_sched_lock_tracking::<R>(current_cpu);
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.cpu_online(cpu_index);
        deferred_registry_inserts = sched.drain_pending_registry_inserts();
    }
    clear_sched_lock_tracking::<R>();
    drop(lock);
    apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    rt.irq_restore(_irq);
}

/// Remove a task completely from both the global registry and the scheduler state.
///
/// This is the canonical way to "reap" a task.  It ensures that the thread
/// list in the registry stays in sync with the scheduler's sorted `threads`
/// vector, preventing index drift that would otherwise lead to panics in the
/// context switcher.
pub fn remove_task_completely<R: BootRuntime>(tid: TaskId) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // 1. Remove from scheduler state (requires SCHEDULER lock)
    {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            purge_task_from_scheduler_queues::<R>(sched, tid);
            sched.state.remove_task(tid);
        }
    }

    // 2. Remove from global registry (requires REGISTRY lock).
    //
    // IMPORTANT: Bind the returned `Box<Thread>` to a local variable so that
    // the `RegistryGuard` temporary (which holds the REGISTRY lock) is dropped
    // at the end of this statement — before `_removed_task` is dropped at end
    // of scope.  If we used the expression-statement form
    // `get_registry().remove(tid);` the unnamed return value would be dropped
    // *before* the guard (reverse creation order), meaning `close_all()` →
    // `wake_task_erased()` → `wake_task()` → `get_task_mut()` would try to
    // re-acquire the REGISTRY spin-lock from the same CPU → spin-deadlock.
    let _removed_task = crate::task::registry::get_registry::<R>().remove(tid);
    // REGISTRY lock released here (RegistryGuard dropped at semicolon).
    // `_removed_task` (Option<Box<Thread>>) is dropped at end of scope,
    // after the lock has been released.

    rt.irq_restore(_irq);
}

fn format_optional_cpu(cpu: Option<usize>) -> alloc::string::String {
    cpu.map_or_else(|| alloc::string::String::from("-"), |c| alloc::format!("{}", c))
}

/// Build `(last_cpu, wake_cpu, run_cpu)` diagnostic strings for dump output.
///
/// `last_cpu` prefers scheduler hot-trace state when present and falls back to
/// the task-local `last_cpu` mirror. `wake_cpu` and `run_cpu` come from
/// scheduler trace fields.
fn task_cpu_trace_strings(
    task_last_cpu: Option<usize>,
    sched_fields: Option<&crate::sched::state::ThreadSchedFields>,
) -> (alloc::string::String, alloc::string::String, alloc::string::String) {
    let last_cpu = sched_fields.and_then(|sf| sf.last_cpu).or(task_last_cpu);
    let wake_cpu = sched_fields.and_then(|sf| sf.wake_cpu);
    let run_cpu = sched_fields.and_then(|sf| sf.run_cpu);
    (format_optional_cpu(last_cpu), format_optional_cpu(wake_cpu), format_optional_cpu(run_cpu))
}

pub fn dump_stats<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    set_sched_lock_tracking::<R>(rt.current_cpu_index());
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };

    crate::kprint!("\n====== TASK DUMP ======\n");
    crate::kprint!(
        "CPUs: {} online / {} total\n",
        sched.state.online_cpu_count,
        sched.total_cpu_count
    );
    // Columns: TID, STATE, PRI, LAST, WAKE, RUN, MIGS, RUNS, USER, SLICE, KSTK, AFFIN, NAME.
    crate::kprint!(
        " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>5}  {:>6}  {:>4}  {:>7}  {:>6}  {:>6}  {}\n",
        "TID",
        "STATE",
        "PRI",
        "LAST",
        "WAKE",
        "RUN",
        "MIGS",
        "RUNS",
        "USER",
        "SLICE",
        "KSTK",
        "AFFIN",
        "NAME"
    );

    let mut runnable_count = 0u32;
    let mut total_runs = 0u64;
    let mut total_migrations = 0u64;
    for task in crate::task::registry::get_registry::<R>().threads.iter() {
        let state_str = match task.state {
            TaskState::Runnable => {
                runnable_count += 1;
                "Runnable"
            }
            TaskState::Running => {
                runnable_count += 1;
                "Running"
            }
            TaskState::Blocked => "Blocked",
            TaskState::Dead => "Dead",
        };
        let pri_str = match task.priority {
            crate::task::TaskPriority::Idle => "Idle",
            crate::task::TaskPriority::Low => "Low",
            crate::task::TaskPriority::Normal => "Norm",
            crate::task::TaskPriority::High => "High",
            crate::task::TaskPriority::Realtime => "RT",
        };
        let (cpu_str, wake_cpu_str, run_cpu_str) =
            task_cpu_trace_strings(task.last_cpu, sched.state.get_task(task.id));
        let runtime_stats = sched.state.task_runtime_stats(task.id);
        let migrations = runtime_stats.migration_count;
        let run_count = runtime_stats.run_count;
        total_runs = total_runs.saturating_add(run_count);
        total_migrations = total_migrations.saturating_add(migrations);
        let user_str = if task.is_user { "Y" } else { "N" };
        let aff_str: alloc::string::String = match task.affinity {
            crate::task::Affinity::Any => alloc::string::String::from("Any"),
            crate::task::Affinity::Pinned(c) => alloc::format!("Pin({})", c),
            crate::task::Affinity::Restricted(ref aff) => {
                alloc::format!("Rst({:#x})", aff.allowed.0)
            }
        };
        let name_str = if task.name_len > 0 {
            core::str::from_utf8(&task.name[..task.name_len as usize]).unwrap_or("?")
        } else {
            "-"
        };
        crate::kprint!(
            " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>5}  {:>6}  {:>4}  {:>3}/{:<3}  {:>5}K  {:>6}  {}\n",
            task.id,
            state_str,
            pri_str,
            cpu_str,
            wake_cpu_str,
            run_cpu_str,
            migrations,
            run_count,
            user_str,
            task.timeslice_remaining,
            types::DEFAULT_TIMESLICE,
            task.kstack_size / 1024,
            aff_str,
            name_str
        );
    }
    let migrations_per_1k_runs =
        if total_runs == 0 { 0 } else { total_migrations.saturating_mul(1000) / total_runs };
    crate::kprint!(
        "  Locality damage: migrations={} runs={} migrations_per_1k_runs={}\n",
        total_migrations,
        total_runs,
        migrations_per_1k_runs
    );

    // Per-CPU run-queue summary
    for &i in &sched.state.online_cpus {
        let pc = &sched.state.per_cpu[i];
        let total: usize = pc.runq.iter().map(|q| q.len()).sum();
        let sample_count = pc.stats.runq_sample_count;
        let sample_total = pc.stats.runq_sample_total;
        let avg_runq = if sample_count == 0 { 0 } else { sample_total / sample_count };
        let idle_hist = pc.stats.idle_episode_hist;
        crate::kprint!(
            "  CPU {}: current={:?} runq={} avg_runq={} idle={:?} idle_ticks={} idle_total_us={} idle_eps={} idle_longest_us={} idle_hist=[{},{},{},{}] dispatch={} steals_in={} steals_out={} ctxsw={} idle->busy={} tick={} ipi_rx={} enq={} deq={} rqchg={} wake={} lock_miss={} lock_miss_pending={} lock_miss_timer={} lock_miss_ipi={} lock_blocked={}\n",
            i,
            pc.current,
            total,
            avg_runq,
            pc.idle_task,
            PROF_IDLE_TICKS_PER_CPU[i].load(Ordering::Relaxed),
            pc.stats.idle_total_us,
            pc.stats.idle_episodes,
            pc.stats.idle_longest_us,
            idle_hist[0],
            idle_hist[1],
            idle_hist[2],
            idle_hist[3],
            pc.stats.dispatch_count,
            pc.stats.steals_in,
            pc.stats.steals_out,
            pc.stats.context_switches,
            pc.stats.idle_to_nonidle,
            pc.stats.timer_interrupts,
            pc.stats.resched_ipi_received,
            pc.stats.runnable_enqueues,
            pc.stats.runnable_dequeues,
            pc.stats.runq_depth_change_events,
            pc.stats.wakeups,
            PROF_TRYLOCK_MISS_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_TIMER_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_IPI_PER_CPU[i].load(Ordering::Relaxed),
            pc.stats.lock_blocked_dispatch
        );
    }
    let wake_to_run_count = PROF_WAKE_TO_RUN_COUNT.load(Ordering::Relaxed);
    let wake_to_run_total = PROF_WAKE_TO_RUN_TICKS_TOTAL.load(Ordering::Relaxed);
    let wake_to_run_avg =
        if wake_to_run_count == 0 { 0 } else { wake_to_run_total / wake_to_run_count };
    crate::kprint!(
        "  Wake→run latency(µs): count={} avg={} max={} hist=[0-5:{},5-20:{},20-100:{},100-500:{},>500:{}]\n",
        wake_to_run_count,
        wake_to_run_avg,
        PROF_WAKE_TO_RUN_TICKS_MAX.load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[0].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[1].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[2].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[3].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[4].load(Ordering::Relaxed)
    );
    crate::kprint!(
        "  Remote wake mailbox age(µs): hist=[0-4:{},5-19:{},20-99:{},100-499:{},>=500:{}] no_ipi={}\n",
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[0].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[1].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[2].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[3].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[4].load(Ordering::Relaxed),
        DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed),
    );
    crate::kprint!(
        "  Runq depth variance: last={} max={} imbalance(total_us={}, episodes={}, longest_us={})\n",
        PROF_RUNQ_DEPTH_VARIANCE_LAST.load(Ordering::Relaxed),
        PROF_RUNQ_DEPTH_VARIANCE_MAX.load(Ordering::Relaxed),
        PROF_IMBALANCE_TOTAL_US.load(Ordering::Relaxed),
        PROF_IMBALANCE_EPISODES.load(Ordering::Relaxed),
        PROF_IMBALANCE_LONGEST_US.load(Ordering::Relaxed)
    );

    crate::kprint!("Sleep queue: {} tasks\n", sched.state.sleep_queue.len());
    crate::kprint!(
        "=== {} tasks, {} runnable ===\n\n",
        crate::task::registry::get_registry::<R>().threads.len(),
        runnable_count
    );

    clear_sched_lock_tracking::<R>();
    drop(lock);
    rt.irq_restore(_irq);
}

/// Collect a [`hooks::SchedDiag`] snapshot without any debug output.
///
/// This is the back-end for the type-erased [`hooks::COLLECT_SCHED_DIAG_HOOK`]
/// and is called by procfs to render `/proc/sched/stat` and
/// `/proc/sched/cpu<N>`.
pub fn collect_sched_diag<R: BootRuntime>() -> hooks::SchedDiag {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = match lock.as_ref() {
        Some(&p) => p,
        None => {
            rt.irq_restore(_irq);
            return hooks::SchedDiag::default();
        }
    };
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };

    let mut per_cpu = alloc::vec::Vec::with_capacity(
        sched.state.online_cpus.iter().filter(|&&i| i < sched.state.per_cpu.len()).count(),
    );
    for &i in &sched.state.online_cpus {
        if i >= sched.state.per_cpu.len() {
            continue;
        }
        let pc = &sched.state.per_cpu[i];
        per_cpu.push(hooks::CpuSchedDiag {
            cpu_id: i,
            runnable_count: pc.runq.runnable_count(),
            context_switches: pc.stats.context_switches,
            wakeups: pc.stats.wakeups,
            steals_in: pc.stats.steals_in,
            steals_out: pc.stats.steals_out,
            timer_interrupts: pc.stats.timer_interrupts,
            idle_total_us: pc.stats.idle_total_us,
            idle_episodes: pc.stats.idle_episodes,
            dispatch_count: pc.stats.dispatch_count,
            resched_ipi_received: pc.stats.resched_ipi_received,
            mailbox_pushes: pc.stats.mailbox_pushes,
            mailbox_tasks_drained: pc.stats.mailbox_tasks_drained,
        });
    }

    let diag = hooks::SchedDiag { online_cpu_count: sched.state.online_cpu_count, per_cpu };

    drop(lock);
    rt.irq_restore(_irq);
    diag
}

pub(super) extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    let cpu_idx = rt.current_cpu_index();
    crate::sched::set_cpu_current_task(cpu_idx, rt.current_tid());
    loop {
        if global_need_resched_load(cpu_idx, Ordering::Acquire) {
            // Use yield_now to trigger a blocking lock acquisition for the
            // scheduler if a reschedule is pending.
            yield_now::<R>();
        }
        rt.wait_for_interrupt();
    }
}

pub static CPU_ONLINE: AtomicUsize = AtomicUsize::new(0);

/// Entry point for secondary CPUs.
///
/// # Safety
/// Must only be called from `kernel_secondary_entry`.
pub unsafe fn enter_secondary(cpu_index: usize) -> ! {
    // Mark as online
    CPU_ONLINE.fetch_add(1, Ordering::Relaxed);
    crate::kdebug!("SMP: Secondary CPU {} online!", cpu_index);

    // Enter scheduler loop via the hook which bootstraps this CPU.
    // The run_scheduler hook will call bootstrap_cpu to set up this CPU's
    if let Some(hook) = unsafe { hooks::RUN_SCHEDULER_HOOK } {
        hook();
    } else {
        panic!("Scheduler hook not initialized!");
    }

    // Fallback if run_scheduler returns (it shouldn't)
    loop {
        crate::runtime_base().wait_for_interrupt();
    }
}


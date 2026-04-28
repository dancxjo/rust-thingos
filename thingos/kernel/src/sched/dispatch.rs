//! Main entry points for the scheduler dispatch and context-switching.
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use super::mailbox::{claim_remote_wake_mailbox_ipi_epoch, enqueue_remote_wake_mailbox};
use super::metrics::*;
use super::policy::SchedPolicy;
use super::profiling::*;
use super::registry_sync::*;
use super::{lifecycle, state, types};
use crate::task::{StartupArg, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchTrigger {
    Timer,
    Ipi,
}

pub fn on_tick<R: BootRuntime>() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    try_resched_if_needed::<R>(DispatchTrigger::Timer);
}

pub fn on_resched_ipi<R: BootRuntime>() {
    let cpu = super::current_cpu_index::<R>();
    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);
    // Use the epoch-based check to avoid redundant schedule_point calls when
    // multiple IPIs were coalesced by the hardware.
    if claim_remote_wake_mailbox_ipi_epoch(cpu) {
        try_resched_if_needed::<R>(DispatchTrigger::Ipi);
    }
}

fn try_resched_if_needed<R: BootRuntime>(trigger: DispatchTrigger) {
    let cpu_idx = super::current_cpu_index::<R>();
    let rt = crate::runtime::<R>();

    let wait_start = rt.mono_ticks();
    let sched_opt = super::SCHEDULER.try_lock();

    if sched_opt.is_none() {
        PROF_RESCHED_TRYLOCK_MISS.fetch_add(1, Ordering::Relaxed);
        PROF_TRYLOCK_MISS_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        match trigger {
            DispatchTrigger::Timer => {
                PROF_TRYLOCK_MISS_TIMER_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
                if rt.is_idle_task_current() {
                    PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
                }
            }
            DispatchTrigger::Ipi => {
                PROF_TRYLOCK_MISS_IPI_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
            }
        }

        if super::GLOBAL_NEED_RESCHED[cpu_idx].load(Ordering::Acquire) {
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        }

        let now = TICK_COUNT.load(Ordering::Relaxed);
        let last_warn = super::TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].load(Ordering::Relaxed);
        if now.saturating_sub(last_warn) >= 100 {
            super::TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].store(now, Ordering::Relaxed);
            let window_start = super::TRYLOCK_MISS_WINDOW_START[cpu_idx].load(Ordering::Relaxed);
            if now.saturating_sub(window_start) >= 100 {
                let count = super::TRYLOCK_MISS_WINDOW_COUNT[cpu_idx].swap(0, Ordering::Relaxed);
                let timer_count =
                    super::TRYLOCK_MISS_WINDOW_TIMER_COUNT[cpu_idx].swap(0, Ordering::Relaxed);
                let ipi_count =
                    super::TRYLOCK_MISS_WINDOW_IPI_COUNT[cpu_idx].swap(0, Ordering::Relaxed);
                let idle_timer_count =
                    super::TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[cpu_idx].swap(0, Ordering::Relaxed);
                let pending_count =
                    super::TRYLOCK_MISS_WINDOW_PENDING_COUNT[cpu_idx].swap(0, Ordering::Relaxed);

                if count > 50 {
                    crate::kwarn!(
                        "SCHED[cpu{}]: High trylock miss rate: {} misses in 1s (timer={}, ipi={}, idle_timer={}, pending={})",
                        cpu_idx,
                        count,
                        timer_count,
                        ipi_count,
                        idle_timer_count,
                        pending_count
                    );
                }
                super::TRYLOCK_MISS_WINDOW_START[cpu_idx].store(now, Ordering::Relaxed);
            }
            super::TRYLOCK_MISS_WINDOW_COUNT[cpu_idx].fetch_add(1, Ordering::Relaxed);
            match trigger {
                DispatchTrigger::Timer => {
                    super::TRYLOCK_MISS_WINDOW_TIMER_COUNT[cpu_idx].fetch_add(1, Ordering::Relaxed);
                    if rt.is_idle_task_current() {
                        super::TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[cpu_idx]
                            .fetch_add(1, Ordering::Relaxed);
                    }
                }
                DispatchTrigger::Ipi => {
                    super::TRYLOCK_MISS_WINDOW_IPI_COUNT[cpu_idx].fetch_add(1, Ordering::Relaxed);
                }
            }
            if super::GLOBAL_NEED_RESCHED[cpu_idx].load(Ordering::Acquire) {
                super::TRYLOCK_MISS_WINDOW_PENDING_COUNT[cpu_idx].fetch_add(1, Ordering::Relaxed);
            }
        }
        return;
    }

    let sched_ptr = *sched_opt.as_ref().unwrap().as_ref().expect("Scheduler not initialized");
    let sched = unsafe { &mut *(sched_ptr as *mut types::Scheduler<R>) };

    record_sched_lock_wait::<R>(
        &PROF_SCHED_WAIT_BLOCK_CURRENT_CALLS,
        &PROF_SCHED_WAIT_BLOCK_CURRENT_US_TOTAL,
        &PROF_SCHED_WAIT_BLOCK_CURRENT_US_MAX,
        &PROF_SCHED_WAIT_BLOCK_CURRENT_HIST,
        wait_start,
    );

    let lock_start = rt.mono_ticks();
    let lock_tracking = super::sched_lock_tracking_guard::<R>(cpu_idx);
    let decision = match trigger {
        DispatchTrigger::Timer => {
            sched.check_preempt_watchdog();
            sched.on_tick()
        }
        DispatchTrigger::Ipi => sched.on_resched_ipi(),
    };

    if let Some(switch) = decision {
        let deferred_syncs = sched.take_deferred_registry_syncs();
        let deferred_ipis = sched.take_pending_wake_ipis();
        drop(lock_tracking);
        drop(sched_opt);

        record_sched_lock_hold::<R>(
            &PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
            lock_start,
        );

        apply_deferred_registry_syncs::<R>(deferred_syncs);
        send_deferred_prepare_schedule_ipis::<R>(deferred_ipis);
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        if let Some(params) = resolve_switch_params::<R>(switch, &mut ghost_ctx, &mut ghost_fs_base)
        {
            execute_context_switch::<R>(params);
        }
    } else {
        let deferred_syncs = sched.take_deferred_registry_syncs();
        let deferred_ipis = sched.take_pending_wake_ipis();
        drop(lock_tracking);
        drop(sched_opt);

        record_sched_lock_hold::<R>(
            &PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
            lock_start,
        );

        apply_deferred_registry_syncs::<R>(deferred_syncs);
        send_deferred_prepare_schedule_ipis::<R>(deferred_ipis);
    }
}

pub(crate) fn resolve_switch_params<R: BootRuntime>(
    decision: types::SwitchDecision,
    ghost_ctx: &mut <R::Tasking as BootTasking>::Context,
    ghost_fs_base: &mut u64,
) -> Option<
    types::SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    >,
> {
    super::debug_assert_scheduler_not_held_by_this_cpu::<R>("resolve_switch_params");
    let mut registry = crate::task::registry::get_registry::<R>();
    let from_idx = registry.get_index(decision.from_tid);
    let to_idx = registry.get_index(decision.to_tid)?;

    if let Some(from_idx) = from_idx {
        let (from_task, to_task) = if from_idx < to_idx {
            let (left, right) = registry.threads.split_at_mut(to_idx);
            (&mut left[from_idx], &mut right[0])
        } else if from_idx > to_idx {
            let (left, right) = registry.threads.split_at_mut(from_idx);
            (&mut right[0], &mut left[to_idx])
        } else {
            return None;
        };

        crate::sched::vm::CURRENT_MAPPINGS[decision.cpu_idx]
            .store(alloc::sync::Arc::as_ptr(&to_task.mappings) as *mut _, Ordering::Release);

        from_task.simd.save(crate::runtime::<R>());
        to_task.simd.restore(crate::runtime::<R>());

        crate::trace::irq_ring::push(abi::trace::TraceEvent::ContextSwitch {
            from: from_task.id,
            to: to_task.id,
            timestamp: crate::trace::now(),
        });

        Some(types::SwitchParams {
            from_ctx: &mut from_task.ctx,
            to_ctx: &to_task.ctx,
            to_aspace: to_task.aspace,
            from_aspace: from_task.aspace,
            from_tid: from_task.id,
            to_tid: to_task.id,
            from_user: from_task.is_user,
            to_user: to_task.is_user,
            from_user_fs_base: &mut from_task.user_fs_base,
            to_user_fs_base: to_task.user_fs_base,
        })
    } else {
        let to_task = &mut registry.threads[to_idx];

        crate::sched::vm::CURRENT_MAPPINGS[decision.cpu_idx]
            .store(alloc::sync::Arc::as_ptr(&to_task.mappings) as *mut _, Ordering::Release);
        to_task.simd.restore(crate::runtime::<R>());

        Some(types::SwitchParams {
            from_ctx: ghost_ctx as *mut _,
            to_ctx: &to_task.ctx,
            to_aspace: to_task.aspace,
            from_aspace: to_task.aspace,
            from_tid: decision.from_tid,
            to_tid: decision.to_tid,
            from_user: false,
            to_user: to_task.is_user,
            from_user_fs_base: ghost_fs_base as *mut _,
            to_user_fs_base: to_task.user_fs_base,
        })
    }
}

pub(crate) fn execute_context_switch<R: BootRuntime>(
    switch: types::SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    >,
) {
    let rt = crate::runtime::<R>();
    let cpu_idx = rt.current_cpu_index();
    crate::sched::set_cpu_current_task(cpu_idx, switch.to_tid);

    let mut spins = 0u32;
    while crate::sched::is_task_on_other_cpu(switch.to_tid, cpu_idx) {
        spins += 1;
        if spins > 10_000 {
            break;
        }
        core::hint::spin_loop();
    }

    if switch.to_aspace != switch.from_aspace {
        rt.tasking().activate_address_space(switch.to_aspace);
    }

    unsafe {
        rt.tasking().switch_with_tls(
            &mut *switch.from_ctx,
            &*switch.to_ctx,
            switch.to_tid,
            switch.from_user_fs_base,
            switch.to_user_fs_base,
        );
    }

    crate::sched::set_cpu_current_task(
        crate::runtime::<R>().current_cpu_index(),
        crate::runtime::<R>().current_tid(),
    );
}

pub(crate) fn send_deferred_prepare_schedule_ipis<R: BootRuntime>(cpus: Vec<usize>) {
    for cpu in cpus {
        DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.fetch_add(1, Ordering::Relaxed);
        crate::runtime::<R>().send_ipi(cpu, 0x30);
    }
}

pub fn emit_debug_summary<R: BootRuntime>() {
    let now = crate::runtime::<R>().mono_ticks();
    let last = LAST_DEBUG_SUMMARY_MONO.load(Ordering::Relaxed);
    if now.wrapping_sub(last) < crate::runtime::<R>().mono_freq_hz() * 5 {
        return;
    }
    LAST_DEBUG_SUMMARY_MONO.store(now, Ordering::Relaxed);

    let mut resched_pending_count = 0;
    for cpu in 0..types::MAX_CPUS {
        if super::GLOBAL_NEED_RESCHED[cpu].load(Ordering::Acquire) {
            resched_pending_count += 1;
        }
    }

    crate::kdebug!(
        "SCHED: tick={} ipi_sent={} ipi_handler={} resched_pending={} coalesced={} hlt_wake={}",
        TICK_COUNT.load(Ordering::Relaxed),
        DIAG_IPI_SENT.load(Ordering::Relaxed),
        DIAG_IPI_HANDLER.load(Ordering::Relaxed),
        resched_pending_count,
        PROF_RESCHED_COALESCED.load(Ordering::Relaxed),
        DIAG_HLT_WAKE.load(Ordering::Relaxed)
    );
}

const PREPARE_SCHEDULE_PICK_BUDGET: usize = 16;
const PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET: usize = 8;
const PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP: usize = 128;
const TERMINATE_CURRENT_SWITCH_RETRY_BUDGET: usize = 32;

impl<R: BootRuntime> types::Scheduler<R> {
    pub fn on_tick(&mut self) -> Option<types::SwitchDecision> {
        let cpu_idx = super::current_cpu_index::<R>();
        self.drain_remote_wake_mailbox(cpu_idx);
        self.wake_sleepers();

        if let Some(current_id) = self.state.per_cpu[cpu_idx].current {
            if let Some(task) = self.state.get_thread_mut(current_id) {
                if task.timeslice_remaining > 0 {
                    task.timeslice_remaining -= 1;
                }
                if task.timeslice_remaining == 0 {
                    self.state.per_cpu[cpu_idx].need_resched = true;
                    super::set_global_need_resched(cpu_idx);
                }
            }
        }

        if self.state.per_cpu[cpu_idx].need_resched {
            self.state.per_cpu[cpu_idx].need_resched = false;
            return self.schedule_point(types::ScheduleReason::SafePoint);
        }
        None
    }

    pub fn on_resched_ipi(&mut self) -> Option<types::SwitchDecision> {
        let cpu_idx = super::current_cpu_index::<R>();
        self.drain_remote_wake_mailbox(cpu_idx);
        self.schedule_point(types::ScheduleReason::SafePoint)
    }

    pub fn schedule_point(
        &mut self,
        reason: types::ScheduleReason,
    ) -> Option<types::SwitchDecision> {
        let cpu_idx = super::current_cpu_index::<R>();
        let _ = super::global_need_resched_swap(cpu_idx, false, Ordering::Acquire);
        self.drain_remote_wake_mailbox(cpu_idx);

        if self.state.per_cpu[cpu_idx].preempt_disable_depth > 0 {
            self.state.per_cpu[cpu_idx].need_resched = true;
            super::set_global_need_resched(cpu_idx);
            return None;
        }

        let switch = self.prepare_schedule();
        self.run_pending_misroute_repair_maintenance();

        if let Some(ref s) = switch {
            if s.from_tid == s.to_tid {
                return None;
            }
        }
        switch
    }

    pub fn prepare_schedule(&mut self) -> Option<types::SwitchDecision> {
        self.flush_metrics_if_needed();

        let rt = crate::runtime::<R>();
        let cpu_idx = super::current_cpu_index::<R>();
        if cpu_idx >= self.state.per_cpu.len() || cpu_idx >= types::MAX_CPUS {
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();

        super::clear_global_need_resched(cpu_idx, Ordering::Release);
        self.state.per_cpu[cpu_idx].need_resched = false;

        super::sample_runq_len(self, cpu_idx);

        let now = TICK_COUNT.load(Ordering::Relaxed);
        let current_id = self.state.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

        let mut next_id = None;
        let mut pick_attempts = 0usize;
        let mut dequeue_failures = 0usize;
        while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET
            && dequeue_failures < PREPARE_SCHEDULE_PICK_BUDGET
        {
            let Some((p, best_idx)) = self.policy.pick_next_task(&self.state, cpu_idx, now) else {
                break;
            };

            let Some(&best_tid) = self.state.per_cpu[cpu_idx].runq[p].get(best_idx) else {
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };

            let still_same = self.state.per_cpu[cpu_idx].runq[p]
                .get(best_idx)
                .copied()
                .is_some_and(|tid| tid == best_tid);
            if !still_same {
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            }
            let Some(id) = self.state.dequeue_task_at(cpu_idx, p, best_idx) else {
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };
            if id != best_tid {
                self.state.enqueue_task(cpu_idx, p, id);
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };
            pick_attempts += 1;
            self.metrics.pops += 1;

            match self.state.get_thread(id) {
                None => continue,
                Some(sf)
                    if sf.state == TaskState::Dead
                        || sf.state == TaskState::Blocked
                        || (sf.state == TaskState::Running && id != current_id) =>
                {
                    continue;
                }
                Some(sf) => {
                    if let crate::task::Affinity::Pinned(target) = sf.affinity {
                        if target != cpu_idx && target < per_cpu_len {
                            self.defer_or_repair_misroute(sf.priority as usize, target, id);
                            continue;
                        }
                    }
                    if let crate::task::Affinity::Restricted(ref aff) = sf.affinity {
                        if !aff.allows(cpu_idx, per_cpu_len) {
                            let target = aff.pick_cpu(per_cpu_len).unwrap_or_else(|| {
                                sf.last_cpu
                                    .filter(|&c| c < per_cpu_len)
                                    .unwrap_or_else(|| self.state.pick_online_cpu_excluding_bsp(0))
                            });
                            self.defer_or_repair_misroute(sf.priority as usize, target, id);
                            continue;
                        }
                    }
                    next_id = Some(id);
                    break;
                }
            }
        }

        let next_id = match next_id {
            Some(id) => Some(id),
            None => {
                let mut found_idle_q = None;
                while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET {
                    let Some(id) = self.state.dequeue_task_front(cpu_idx, 0) else {
                        break;
                    };
                    pick_attempts += 1;
                    self.metrics.pops += 1;
                    match self.state.get_thread(id) {
                        None => continue,
                        Some(sf)
                            if sf.state == TaskState::Dead
                                || sf.state == TaskState::Blocked
                                || (sf.state == TaskState::Running && id != current_id) =>
                        {
                            continue;
                        }
                        Some(sf) => {
                            if let crate::task::Affinity::Pinned(target) = sf.affinity {
                                if target != cpu_idx && target < per_cpu_len {
                                    self.defer_or_repair_misroute(sf.priority as usize, target, id);
                                    continue;
                                }
                            }
                            if let crate::task::Affinity::Restricted(ref aff) = sf.affinity {
                                if !aff.allows(cpu_idx, per_cpu_len) {
                                    let target = aff.pick_cpu(per_cpu_len).unwrap_or_else(|| {
                                        sf.last_cpu.filter(|&c| c < per_cpu_len).unwrap_or_else(
                                            || self.state.pick_online_cpu_excluding_bsp(0),
                                        )
                                    });
                                    self.defer_or_repair_misroute(sf.priority as usize, target, id);
                                    continue;
                                }
                            }
                        }
                    }
                    found_idle_q = Some(id);
                    break;
                }
                if let Some(id) = found_idle_q {
                    Some(id)
                } else {
                    let stolen = self.idle_steal(cpu_idx);
                    if stolen.is_some() {
                        stolen
                    } else if let Some(idle) = self.state.per_cpu[cpu_idx].idle_task {
                        self.metrics.idle_picks += 1;
                        Some(idle)
                    } else {
                        None
                    }
                }
            }
        };

        let next_id = next_id?;

        if next_id == current_id {
            if let Some(current_sched) = self.state.get_task_mut(current_id) {
                current_sched.state = TaskState::Running;
                current_sched.run_cpu = Some(cpu_idx);

                let next_is_idle = Some(next_id) == self.state.per_cpu[cpu_idx].idle_task;
                rt.set_idle_task_current(next_is_idle);
            }
            return None;
        }

        let current_was_idle = Some(current_id) == self.state.per_cpu[cpu_idx].idle_task;
        let next_is_nonidle = Some(next_id) != self.state.per_cpu[cpu_idx].idle_task;
        let now_mono = rt.mono_ticks();
        if !current_was_idle && !next_is_nonidle {
            if self.state.per_cpu[cpu_idx].idle_enter_mono_ticks.is_none() {
                self.state.per_cpu[cpu_idx].idle_enter_mono_ticks = Some(now_mono);
                self.state.per_cpu[cpu_idx].stats.idle_episodes =
                    self.state.per_cpu[cpu_idx].stats.idle_episodes.saturating_add(1);
            }
        }
        if current_was_idle && next_is_nonidle {
            self.state.per_cpu[cpu_idx].stats.idle_to_nonidle =
                self.state.per_cpu[cpu_idx].stats.idle_to_nonidle.saturating_add(1);
            if let Some(idle_enter) = self.state.per_cpu[cpu_idx].idle_enter_mono_ticks.take() {
                let idle_us = ticks_to_us::<R>(now_mono.wrapping_sub(idle_enter));
                let stats = &mut self.state.per_cpu[cpu_idx].stats;
                stats.idle_total_us = stats.idle_total_us.saturating_add(idle_us);
                stats.idle_longest_us = stats.idle_longest_us.max(idle_us);
                let bucket = idle_episode_hist_bucket(idle_us);
                stats.idle_episode_hist[bucket] = stats.idle_episode_hist[bucket].saturating_add(1);
            }
        }
        self.state.per_cpu[cpu_idx].stats.context_switches =
            self.state.per_cpu[cpu_idx].stats.context_switches.saturating_add(1);
        self.state.per_cpu[cpu_idx].stats.dispatch_count =
            self.state.per_cpu[cpu_idx].stats.dispatch_count.saturating_add(1);

        self.state.per_cpu[cpu_idx].current = Some(next_id);

        let old_was_running = self
            .state
            .get_task(current_id)
            .map(|task| task.state == TaskState::Running)
            .unwrap_or(false);
        let mut old_registry_sync = types::DeferredRegistrySync {
            tid: current_id,
            new_state: if old_was_running { Some(TaskState::Runnable) } else { None },
            new_enqueued_at_tick: if old_was_running { Some(now) } else { None },
            new_last_cpu: Some(cpu_idx),
        };

        if let Some(task) = self.state.get_task_mut(current_id) {
            if task.state == TaskState::Running {
                task.state = TaskState::Runnable;
                task.enqueued_at_tick = now;
            }
            task.last_cpu = Some(cpu_idx);
            task.run_cpu = None;
        }

        if let Some(task) = self.state.get_task_mut(next_id) {
            task.state = TaskState::Running;
            task.run_cpu = Some(cpu_idx);
            task.timeslice_remaining = types::DEFAULT_TIMESLICE;
            task.runq_location = None;

            self.pending_registry_syncs.push(types::DeferredRegistrySync {
                tid: next_id,
                new_state: Some(TaskState::Running),
                new_enqueued_at_tick: None,
                new_last_cpu: Some(cpu_idx),
            });
        }

        self.pending_registry_syncs.push(old_registry_sync);

        Some(types::SwitchDecision { cpu_idx, from_tid: current_id, to_tid: next_id })
    }

    pub fn preempt_disable(&mut self) {
        let cpu_idx = super::current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth == 0 {
            per_cpu.preempt_disable_since = TICK_COUNT.load(Ordering::Relaxed);
            per_cpu.preempt_watchdog_warned = false;
        }
        per_cpu.preempt_disable_depth += 1;
        if per_cpu.preempt_disable_depth == 1 {
            crate::trace::irq_ring::push(abi::trace::TraceEvent::PreemptDisable {
                depth: per_cpu.preempt_disable_depth as u32,
                timestamp: crate::trace::now(),
            });
        }
    }

    fn flush_metrics_if_needed(&mut self) {
        let rt = crate::runtime::<R>();
        let now = rt.mono_ticks();
        let limit = rt.mono_freq_hz() * 2;

        if self.metrics.last_flush == 0 {
            self.metrics.last_flush = now;
            return;
        }

        if now - self.metrics.last_flush > limit {
            self.metrics.yields = 0;
            self.metrics.pops = 0;
            self.metrics.pushes = 0;
            self.metrics.idle_picks = 0;
            self.metrics.last_flush = now;
        }
    }

    pub fn preempt_enable(&mut self) -> Option<types::SwitchDecision> {
        let cpu_idx = super::current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth > 0 {
            per_cpu.preempt_disable_depth -= 1;
        }

        if per_cpu.preempt_disable_depth == 0 && per_cpu.need_resched {
            per_cpu.need_resched = false;
            return self.schedule_point(types::ScheduleReason::SafePoint);
        }
        None
    }

    pub fn prepare_yield(&mut self) -> Option<types::SwitchDecision> {
        let cpu_idx = super::current_cpu_index::<R>();
        let current_id = self.state.per_cpu.get(cpu_idx)?.current?;

        self.metrics.yields += 1;

        if Some(current_id) != self.state.per_cpu[cpu_idx].idle_task {
            let previous_enqueue_was_yield = matches!(
                self.state.last_enqueue_cause(current_id),
                state::EnqueueCause::YieldRequeue
            );
            let mut requeue_prio = None;
            if let Some(sf_mut) = self.state.get_thread_mut(current_id) {
                if sf_mut.state != TaskState::Dead && sf_mut.state != TaskState::Blocked {
                    let spin_penalty_eligible =
                        sf_mut.voluntary_yields >= types::SPIN_YIELD_PENALTY_THRESHOLD;
                    let mut prio = sf_mut.priority as usize;
                    sf_mut.voluntary_yields = sf_mut.voluntary_yields.saturating_add(1);
                    if previous_enqueue_was_yield && spin_penalty_eligible {
                        prio = prio
                            .saturating_sub(types::SPIN_YIELD_PENALTY_BANDS)
                            .max(TaskPriority::Low as usize);
                    }
                    requeue_prio = Some(prio);
                }
            }
            if let Some(requeue_prio) = requeue_prio {
                self.state.enqueue_task(cpu_idx, requeue_prio, current_id);
                self.state.note_enqueue_cause(current_id, state::EnqueueCause::YieldRequeue);
                self.metrics.pushes += 1;
            }
        }

        let switch = self.prepare_schedule();
        self.run_pending_misroute_repair_maintenance();
        switch
    }

    pub fn check_preempt_watchdog(&mut self) {
        let cpu_idx = super::current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth > 0 && !per_cpu.preempt_watchdog_warned {
            let now = TICK_COUNT.load(Ordering::Relaxed);
            if now.saturating_sub(per_cpu.preempt_disable_since) > 500 {
                per_cpu.preempt_watchdog_warned = true;
            }
        }
    }

    #[inline]
    fn flush_pending_misrouted_requeues_bounded(&mut self, max_to_flush: usize) {
        let current_cpu = super::current_cpu_index::<R>();
        let now_tick = TICK_COUNT.load(Ordering::Relaxed);
        for _ in 0..max_to_flush {
            let Some((prio, target_cpu, id)) = self.pending_misrouted_requeues.pop() else {
                break;
            };
            if target_cpu == current_cpu {
                self.state.enqueue_task(target_cpu, prio, id);
                self.state.note_enqueue_cause(id, state::EnqueueCause::AffinityRepair);
            } else {
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    target_cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: id,
                        priority: prio,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
            }
            self.queue_prepare_schedule_ipi_dedup(target_cpu);
        }
    }

    #[inline]
    fn resolve_pending_misroute_requeue(
        &self,
        queued_prio: usize,
        queued_target_cpu: usize,
        id: TaskId,
    ) -> Option<(usize, usize)> {
        let sf = self.state.get_thread(id)?;
        if sf.state != TaskState::Runnable || sf.runq_location.is_some() {
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();
        if per_cpu_len == 0 {
            return None;
        }
        let prio = sf.priority as usize;
        match sf.affinity {
            crate::task::Affinity::Pinned(cpu) if cpu < per_cpu_len => Some((prio, cpu)),
            crate::task::Affinity::Pinned(_) => {
                Some((queued_prio, queued_target_cpu.min(per_cpu_len - 1)))
            }
            crate::task::Affinity::Any => {
                let fallback = queued_target_cpu.min(per_cpu_len - 1);
                Some((prio, sf.last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(fallback)))
            }
            crate::task::Affinity::Restricted(ref aff) => {
                let target = aff
                    .pick_cpu(per_cpu_len)
                    .unwrap_or_else(|| queued_target_cpu.min(per_cpu_len - 1));
                Some((prio, target))
            }
        }
    }

    #[inline]
    fn prevalidate_pending_misrouted_requeues(&mut self) {
        let mut i = 0usize;
        while i < self.pending_misrouted_requeues.len() {
            let (queued_prio, queued_target_cpu, id) = self.pending_misrouted_requeues[i];
            if let Some((prio, target_cpu)) =
                self.resolve_pending_misroute_requeue(queued_prio, queued_target_cpu, id)
            {
                if (prio, target_cpu) != (queued_prio, queued_target_cpu) {
                    self.pending_misrouted_requeues[i] = (prio, target_cpu, id);
                }
                i += 1;
            } else {
                self.pending_misrouted_requeues.swap_remove(i);
            }
        }
    }

    #[inline]
    pub fn run_pending_misroute_repair_maintenance(&mut self) {
        self.prevalidate_pending_misrouted_requeues();
        self.flush_pending_misrouted_requeues_bounded(PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET);
    }

    #[inline]
    fn defer_or_repair_misroute(&mut self, prio: usize, target_cpu: usize, id: TaskId) {
        let Some((prio, target_cpu)) = self.resolve_pending_misroute_requeue(prio, target_cpu, id)
        else {
            return;
        };
        if self.pending_misrouted_requeues.len() < PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP {
            self.pending_misrouted_requeues.push((prio, target_cpu, id));
            return;
        }
        let current_cpu = super::current_cpu_index::<R>();
        if target_cpu == current_cpu {
            self.state.enqueue_task(target_cpu, prio, id);
            self.state.note_enqueue_cause(id, state::EnqueueCause::AffinityRepair);
        } else {
            let now_tick = TICK_COUNT.load(Ordering::Relaxed);
            let wake_mono = crate::runtime::<R>().mono_ticks();
            enqueue_remote_wake_mailbox(
                target_cpu,
                types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: prio,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }
        self.queue_prepare_schedule_ipi_dedup(target_cpu);
    }

    #[inline]
    pub fn queue_prepare_schedule_ipi_dedup(&mut self, target_cpu: usize) {
        let already_pending = super::set_global_need_resched(target_cpu);
        if !already_pending {
            self.queue_pending_prepare_schedule_ipi(target_cpu);
        } else {
            PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn terminate_current(
        &mut self,
        terminating_tid: TaskId,
        siblings_to_kill: &[TaskId],
    ) -> types::SwitchDecision {
        let cpu_idx = super::current_cpu_index::<R>();
        let current_id = self
            .state
            .per_cpu
            .get(cpu_idx)
            .and_then(|pc| pc.current)
            .expect("terminate_current called with no current task");
        if terminating_tid != current_id {
            panic!(
                "scheduler invariant violated: terminate_current tid mismatch (cpu={}, scheduler_current={}, terminating_tid={})",
                cpu_idx, current_id, terminating_tid
            );
        }

        if let Some(task) = self.state.get_task_mut(current_id) {
            task.runq_location = None;
            task.state = TaskState::Dead;
        }
        lifecycle::purge_task_from_scheduler_queues::<R>(self, current_id);

        for &sibling in siblings_to_kill {
            if sibling == current_id {
                continue;
            }
            if let Some(sf) = self.state.get_task_mut(sibling) {
                sf.runq_location = None;
                sf.state = TaskState::Dead;
            }
            lifecycle::purge_task_from_scheduler_queues::<R>(self, sibling);
        }

        for _ in 0..TERMINATE_CURRENT_SWITCH_RETRY_BUDGET {
            if let Some(switch) = self.prepare_schedule() {
                self.run_pending_misroute_repair_maintenance();
                return switch;
            }
            self.run_pending_misroute_repair_maintenance();
            core::hint::spin_loop();
        }

        panic!(
            "scheduler invariant violated: terminate_current could not find a switch after {} attempts (cpu={}, current_tid={})",
            TERMINATE_CURRENT_SWITCH_RETRY_BUDGET, cpu_idx, current_id
        );
    }

    pub fn set_priority_hot_cache(&mut self, id: TaskId, priority: TaskPriority) -> bool {
        let mut requeue_cpu = None;
        let Some(task) = self.state.get_task_mut(id) else {
            return false;
        };
        task.priority = priority;
        if task.state == TaskState::Runnable {
            requeue_cpu = task.runq_location.map(|(cpu, _)| cpu);
        }

        if let Some(cpu) = requeue_cpu {
            self.state.remove_task_from_runq(id);
            let current_cpu = super::current_cpu_index::<R>();
            if cpu == current_cpu {
                self.state.enqueue_task(cpu, priority as usize, id);
                self.state.per_cpu[cpu].need_resched = true;
            } else {
                let now_tick = TICK_COUNT.load(Ordering::Relaxed);
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: id,
                        priority: priority as usize,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
                self.queue_prepare_schedule_ipi_dedup(cpu);
            }
        }
        true
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        let _ = self.set_priority_hot_cache(id, priority);
    }

    pub fn cpu_online(&mut self, cpu_index: usize) {
        while self.state.per_cpu.len() <= cpu_index {
            self.state.per_cpu.push(state::PerCpu::new());
        }
        self.state.mark_cpu_online(cpu_index);
        self.total_cpu_count = self.total_cpu_count.max(cpu_index.saturating_add(1));

        let i = cpu_index;
        let idle_id = self.spawn(
            lifecycle::idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        self.state.remove_task_from_runq(idle_id);
        self.state.per_cpu[i].idle_task = Some(idle_id);

        if let Some(sf) = self.state.get_task_mut(idle_id) {
            sf.affinity = crate::task::Affinity::Pinned(i);
        }
    }
}

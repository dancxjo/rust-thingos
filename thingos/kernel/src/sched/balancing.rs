//! Scheduler load balancing and CPU selection logic.
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

use super::mailbox::enqueue_remote_wake_mailbox;
use super::profiling::*;
use super::{state, types};
use crate::task::{TaskId, TaskPriority};

// Keep steal scans bounded to limit idle-path latency while still peeking past
// a small pinned/unstealable head segment.
pub(super) const STEAL_SCAN_DEPTH_PER_PRIORITY: usize = 8;
/// Minimum run-queue depth on a victim CPU before it becomes a steal target.
///
/// A threshold of 2 means we only steal when there is genuine imbalance: the
/// victim already has one task running plus at least one waiting.
pub(super) const STEAL_MIN_VICTIM_DEPTH: usize = 2;
/// CPU-index radius that defines "nearby" CPUs for steal ordering.
///
/// CPUs whose index falls within `[local - STEAL_NEARBY_RADIUS, local +
/// STEAL_NEARBY_RADIUS]` (inclusive, clamped to valid range) are tried first
/// during `idle_steal`.  This is a topology heuristic: nearby indices are
/// often on the same package or share last-level cache, so stealing from them
/// tends to have lower cache-miss overhead than stealing from distant CPUs.
pub(super) const STEAL_NEARBY_RADIUS: usize = 4;

#[inline]
pub(crate) fn cross_cpu_runq_migration_enabled() -> bool {
    false
}

// Allow local wake routing for Any-affinity tasks when the previous CPU is
// meaningfully busier, while still preserving cache locality under similar load.
pub(super) const ANY_WAKE_LOCAL_DEPTH_BIAS: usize = 1;

pub(super) static ANY_WAKE_POLICY_INIT_DONE: AtomicBool = AtomicBool::new(false);
pub(super) static ANY_WAKE_OVERLOAD_POLICY: AtomicU8 =
    AtomicU8::new(AnyWakeOverloadPolicy::Steal as u8);
pub(super) static ANY_WAKE_OVERLOAD_GAP: AtomicUsize = AtomicUsize::new(2);
// The streak counter storage is AtomicU8, but clamp APIs operate on usize.
pub(super) const ANY_WAKE_OVERLOAD_STREAK_MAX: usize = u8::MAX as usize;
pub(super) static ANY_WAKE_OVERLOAD_STREAK_REQUIRED: AtomicUsize = AtomicUsize::new(1);
pub(super) static ANY_WAKE_OVERLOAD_STREAK: [AtomicU8; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(super) enum AnyWakeOverloadPolicy {
    Off = 0,
    Redirect = 1,
    /// Preserve locality preference only when not overloaded; otherwise treat
    /// the wakeup as stealable by the least-loaded online CPU.
    ///
    /// Current implementation routes to the same target as `Redirect`; the
    /// distinct variant keeps a policy surface for follow-up steal mechanics.
    Steal = 2,
}

pub(super) fn any_wake_overload_policy_from_u8(v: u8) -> AnyWakeOverloadPolicy {
    match v {
        0 => AnyWakeOverloadPolicy::Off,
        1 => AnyWakeOverloadPolicy::Redirect,
        2 => AnyWakeOverloadPolicy::Steal,
        _ => AnyWakeOverloadPolicy::Off,
    }
}

pub(super) fn parse_any_wake_overload_policy(value: &str) -> AnyWakeOverloadPolicy {
    match value {
        "redirect" => AnyWakeOverloadPolicy::Redirect,
        "steal" => AnyWakeOverloadPolicy::Steal,
        _ => AnyWakeOverloadPolicy::Off,
    }
}

pub(super) fn init_any_wake_policy_from_env_once() {
    if ANY_WAKE_POLICY_INIT_DONE
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        // Build-time tunables via `option_env!` (captured at compile time).
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_POLICY") {
            ANY_WAKE_OVERLOAD_POLICY
                .store(parse_any_wake_overload_policy(v) as u8, Ordering::Release);
        }
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_OVERLOAD_GAP") {
            if let Ok(gap) = v.parse::<usize>() {
                ANY_WAKE_OVERLOAD_GAP.store(gap.max(1), Ordering::Release);
            }
        }
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_OVERLOAD_STREAK") {
            if let Ok(streak) = v.parse::<usize>() {
                ANY_WAKE_OVERLOAD_STREAK_REQUIRED
                    .store(streak.clamp(1, ANY_WAKE_OVERLOAD_STREAK_MAX), Ordering::Release);
            }
        }
    }
}

pub(super) fn runq_depth_for_cpu(state: &state::SchedState, cpu: usize) -> usize {
    state.per_cpu.get(cpu).map(|pc| pc.runq.total_len()).unwrap_or(0)
}

pub(super) fn least_loaded_online_cpu(state: &state::SchedState) -> Option<(usize, usize)> {
    #[cfg(test)]
    super::TEST_LEAST_LOADED_ONLINE_CPU_CALLS.fetch_add(1, Ordering::Relaxed);
    state
        .online_cpus
        .iter()
        .copied()
        .filter(|&cpu| cpu < state.per_cpu.len())
        .map(|cpu| (cpu, runq_depth_for_cpu(state, cpu)))
        .min_by_key(|&(_, depth)| depth)
}

pub(super) struct WakeBatchLoadSnapshot {
    pub(super) per_cpu_depths: alloc::vec::Vec<usize>,
}

impl WakeBatchLoadSnapshot {
    pub(super) fn new(state: &state::SchedState) -> Self {
        let mut per_cpu_depths = alloc::vec::Vec::with_capacity(state.per_cpu.len());
        for cpu in 0..state.per_cpu.len() {
            per_cpu_depths.push(runq_depth_for_cpu(state, cpu));
        }
        Self { per_cpu_depths }
    }

    #[inline]
    pub(super) fn depth_for_cpu(&self, cpu: usize) -> usize {
        self.per_cpu_depths.get(cpu).copied().unwrap_or(0)
    }

    pub(super) fn least_loaded_online_cpu(
        &self,
        state: &state::SchedState,
    ) -> Option<(usize, usize)> {
        state
            .online_cpus
            .iter()
            .copied()
            .filter(|&cpu| cpu < self.per_cpu_depths.len())
            .map(|cpu| (cpu, self.depth_for_cpu(cpu)))
            .min_by_key(|&(_, depth)| depth)
    }

    #[inline]
    pub(super) fn note_enqueue(&mut self, cpu: usize) {
        if let Some(depth) = self.per_cpu_depths.get_mut(cpu) {
            *depth = depth.saturating_add(1);
        }
    }
}

pub(super) fn is_cpu_idle(state: &state::SchedState, cpu: usize) -> bool {
    state
        .per_cpu
        .get(cpu)
        .map(|pc| matches!((pc.current, pc.idle_task), (Some(cur), Some(idle)) if cur == idle))
        .unwrap_or(false)
}

pub(super) fn find_idle_online_cpu(state: &state::SchedState) -> Option<usize> {
    state
        .online_cpus
        .iter()
        .copied()
        .find(|&cpu| cpu < state.per_cpu.len() && is_cpu_idle(state, cpu))
}

pub(crate) fn choose_wake_cpu<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
) -> usize {
    if !cross_cpu_runq_migration_enabled() {
        let per_cpu_len = sched.state.per_cpu.len();
        if per_cpu_len == 0 {
            return 0;
        }
        let current_cpu = super::current_cpu_index::<R>().min(per_cpu_len - 1);
        return last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(current_cpu);
    }

    let preferred = select_preferred_any_affinity_wake_cpu::<R>(sched, last_cpu);

    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let preferred_depth = runq_depth_for_cpu(&sched.state, preferred);
    if preferred_depth >= overload_gap {
        if let Some(idle_cpu) = find_idle_online_cpu(&sched.state) {
            if idle_cpu != preferred {
                return idle_cpu;
            }
        }
    }

    select_any_affinity_wake_cpu::<R>(sched, preferred)
}

pub(super) fn choose_wake_cpu_from_snapshot<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
    load_snapshot: &WakeBatchLoadSnapshot,
) -> usize {
    if !cross_cpu_runq_migration_enabled() {
        let per_cpu_len = sched.state.per_cpu.len();
        if per_cpu_len == 0 {
            return 0;
        }
        let current_cpu = super::current_cpu_index::<R>().min(per_cpu_len - 1);
        return last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(current_cpu);
    }

    let preferred =
        select_preferred_any_affinity_wake_cpu_from_snapshot::<R>(sched, last_cpu, load_snapshot);

    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let preferred_depth = load_snapshot.depth_for_cpu(preferred);
    if preferred_depth >= overload_gap {
        if let Some(idle_cpu) = find_idle_online_cpu(&sched.state) {
            if idle_cpu != preferred {
                return idle_cpu;
            }
        }
    }

    select_any_affinity_wake_cpu_from_snapshot::<R>(sched, preferred, load_snapshot)
}

pub(crate) fn select_any_affinity_wake_cpu<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    preferred_cpu: usize,
) -> usize {
    init_any_wake_policy_from_env_once();

    let local_cpu = super::current_cpu_index::<R>();
    let preferred = if preferred_cpu < sched.state.per_cpu.len()
        && sched.state.online_cpus.contains(&preferred_cpu)
    {
        preferred_cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        local_cpu
    } else {
        0
    };

    let policy = any_wake_overload_policy_from_u8(ANY_WAKE_OVERLOAD_POLICY.load(Ordering::Acquire));
    if policy == AnyWakeOverloadPolicy::Off {
        return preferred;
    }

    let preferred_depth = runq_depth_for_cpu(&sched.state, preferred);
    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let overloaded = preferred_depth >= overload_gap;
    let Some(streak_cell) = ANY_WAKE_OVERLOAD_STREAK.get(preferred) else {
        return preferred;
    };
    if !overloaded {
        streak_cell.store(0, Ordering::Release);
        return preferred;
    }

    let streak_required = ANY_WAKE_OVERLOAD_STREAK_REQUIRED.load(Ordering::Acquire) as u8;
    let prior_streak = streak_cell.load(Ordering::Acquire);
    let next_streak = prior_streak.saturating_add(1);
    streak_cell.store(next_streak, Ordering::Release);
    if next_streak < streak_required {
        return preferred;
    }

    let Some((least_cpu, least_depth)) = least_loaded_online_cpu(&sched.state) else {
        return preferred;
    };
    // Run-queue depth is a bounded queue-length sum; use saturating subtraction
    // so "depth delta >= gap" cannot wrap.
    let overloaded_vs_least = preferred_depth.saturating_sub(least_depth) >= overload_gap;
    if overloaded_vs_least && least_cpu != preferred {
        streak_cell.store(0, Ordering::Release);
        least_cpu
    } else {
        preferred
    }
}

pub(super) fn select_any_affinity_wake_cpu_from_snapshot<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    preferred_cpu: usize,
    load_snapshot: &WakeBatchLoadSnapshot,
) -> usize {
    init_any_wake_policy_from_env_once();

    let local_cpu = super::current_cpu_index::<R>();
    let preferred = if preferred_cpu < sched.state.per_cpu.len()
        && sched.state.online_cpus.contains(&preferred_cpu)
    {
        preferred_cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        local_cpu
    } else {
        0
    };

    let policy = any_wake_overload_policy_from_u8(ANY_WAKE_OVERLOAD_POLICY.load(Ordering::Acquire));
    if policy == AnyWakeOverloadPolicy::Off {
        return preferred;
    }

    let preferred_depth = load_snapshot.depth_for_cpu(preferred);
    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let overloaded = preferred_depth >= overload_gap;
    let Some(streak_cell) = ANY_WAKE_OVERLOAD_STREAK.get(preferred) else {
        return preferred;
    };
    if !overloaded {
        streak_cell.store(0, Ordering::Release);
        return preferred;
    }

    let streak_required = ANY_WAKE_OVERLOAD_STREAK_REQUIRED.load(Ordering::Acquire) as u8;
    let prior_streak = streak_cell.load(Ordering::Acquire);
    let next_streak = prior_streak.saturating_add(1);
    streak_cell.store(next_streak, Ordering::Release);
    if next_streak < streak_required {
        return preferred;
    }

    let Some((least_cpu, least_depth)) = load_snapshot.least_loaded_online_cpu(&sched.state) else {
        return preferred;
    };
    let overloaded_vs_least = preferred_depth.saturating_sub(least_depth) >= overload_gap;
    if overloaded_vs_least && least_cpu != preferred {
        streak_cell.store(0, Ordering::Release);
        least_cpu
    } else {
        preferred
    }
}

pub(crate) fn select_preferred_any_affinity_wake_cpu<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
) -> usize {
    let local_cpu = super::current_cpu_index::<R>();
    let local_online =
        local_cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&local_cpu);
    let fallback = if local_online {
        local_cpu
    } else if let Some(cpu) =
        sched.state.online_cpus.iter().copied().find(|&cpu| cpu < sched.state.per_cpu.len())
    {
        cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        // Defensive fallback for transient test/bootstrap states where online
        // bookkeeping lags but per-CPU storage is already initialized.
        local_cpu
    } else {
        0
    };

    let Some(last_cpu) = last_cpu
        .filter(|&cpu| cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&cpu))
    else {
        return fallback;
    };

    if !local_online || last_cpu == local_cpu {
        return last_cpu;
    }

    // Compare total runnable depth across all priority queues on each CPU.
    let local_depth = runq_depth_for_cpu(&sched.state, local_cpu);
    let last_depth = runq_depth_for_cpu(&sched.state, last_cpu);
    // A bias of 1 preserves locality by keeping `last_cpu` unless local CPU has
    // at least 2 fewer queued tasks.
    // saturating_add is defensive for pathological queue lengths.
    if local_depth.saturating_add(ANY_WAKE_LOCAL_DEPTH_BIAS) < last_depth {
        local_cpu
    } else {
        last_cpu
    }
}

pub(super) fn select_preferred_any_affinity_wake_cpu_from_snapshot<R: crate::BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
    load_snapshot: &WakeBatchLoadSnapshot,
) -> usize {
    let local_cpu = super::current_cpu_index::<R>();
    let local_online =
        local_cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&local_cpu);
    let fallback = if local_online {
        local_cpu
    } else if let Some(cpu) =
        sched.state.online_cpus.iter().copied().find(|&cpu| cpu < sched.state.per_cpu.len())
    {
        cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        local_cpu
    } else {
        0
    };

    let Some(last_cpu) = last_cpu
        .filter(|&cpu| cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&cpu))
    else {
        return fallback;
    };

    if !local_online || last_cpu == local_cpu {
        return last_cpu;
    }

    let local_depth = load_snapshot.depth_for_cpu(local_cpu);
    let last_depth = load_snapshot.depth_for_cpu(last_cpu);
    if local_depth.saturating_add(ANY_WAKE_LOCAL_DEPTH_BIAS) < last_depth {
        local_cpu
    } else {
        last_cpu
    }
}

#[cfg(test)]
pub(super) fn reset_any_wake_policy_for_tests() {
    ANY_WAKE_POLICY_INIT_DONE.store(true, Ordering::Release);
    ANY_WAKE_OVERLOAD_POLICY.store(AnyWakeOverloadPolicy::Off as u8, Ordering::Release);
    ANY_WAKE_OVERLOAD_GAP.store(4, Ordering::Release);
    ANY_WAKE_OVERLOAD_STREAK_REQUIRED.store(3, Ordering::Release);
    for streak in &ANY_WAKE_OVERLOAD_STREAK {
        streak.store(0, Ordering::Release);
    }
}

#[cfg(test)]
pub(super) fn set_any_wake_policy_for_tests(policy: &str, overload_gap: usize) {
    set_any_wake_policy_for_tests_with_streak(policy, overload_gap, 1);
}

#[cfg(test)]
pub(super) fn set_any_wake_policy_for_tests_with_streak(
    policy: &str,
    overload_gap: usize,
    overload_streak: usize,
) {
    ANY_WAKE_POLICY_INIT_DONE.store(true, Ordering::Release);
    ANY_WAKE_OVERLOAD_POLICY.store(parse_any_wake_overload_policy(policy) as u8, Ordering::Release);
    ANY_WAKE_OVERLOAD_GAP.store(overload_gap.max(1), Ordering::Release);
    ANY_WAKE_OVERLOAD_STREAK_REQUIRED
        .store(overload_streak.clamp(1, ANY_WAKE_OVERLOAD_STREAK_MAX), Ordering::Release);
    for streak in &ANY_WAKE_OVERLOAD_STREAK {
        streak.store(0, Ordering::Release);
    }
}

impl<R: crate::BootRuntime> types::Scheduler<R> {
    /// Slow-path periodic load balancer: proactively migrate tasks from overloaded
    /// imbalances that do not trigger idle-steal.
    pub(super) fn periodic_load_balance(&mut self) {
        if !cross_cpu_runq_migration_enabled() {
            return;
        }

        let now = TICK_COUNT.load(Ordering::Relaxed);

        // Rate limit: skip if we balanced recently.
        if now.wrapping_sub(self.last_balance_tick) < types::PERIODIC_BALANCE_INTERVAL_TICKS {
            return;
        }
        self.last_balance_tick = now;

        // Require at least two online CPUs for inter-CPU migration.
        if self.state.online_cpus.len() < 2 {
            return;
        }

        // Find the busiest and least-loaded online CPUs in a single pass.
        let mut busiest_cpu = 0usize;
        let mut busiest_depth = 0usize;
        let mut least_cpu = 0usize;
        let mut least_depth = usize::MAX;
        let mut found_any = false;

        for &cpu in &self.state.online_cpus {
            let depth = runq_depth_for_cpu(&self.state, cpu);
            if !found_any || depth > busiest_depth {
                busiest_depth = depth;
                busiest_cpu = cpu;
            }
            if !found_any || depth < least_depth {
                least_depth = depth;
                least_cpu = cpu;
            }
            found_any = true;
        }

        if !found_any || busiest_cpu == least_cpu {
            return;
        }

        // Only rebalance when the imbalance is severe enough to warrant
        // migration and the victim has enough tasks to donate.
        let depth_diff = busiest_depth.saturating_sub(least_depth);
        if depth_diff < types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF
            || busiest_depth < STEAL_MIN_VICTIM_DEPTH
        {
            return;
        }

        // Migrate up to PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN tasks.
        let mut migrated = 0usize;
        while migrated < types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN {
            // Re-check imbalance before each individual migration to avoid
            // over-migrating when the situation improves mid-pass.
            let cur_busiest = runq_depth_for_cpu(&self.state, busiest_cpu);
            let cur_least = runq_depth_for_cpu(&self.state, least_cpu);
            if cur_busiest.saturating_sub(cur_least)
                < types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF
                || cur_busiest < STEAL_MIN_VICTIM_DEPTH
            {
                break;
            }

            // Steal one task from the busiest CPU.  try_steal_one removes it
            // from the victim's run queue, sets wake_cpu = least_cpu, and
            // records the enqueue cause as Steal.
            let Some(stolen_id) =
                self.try_steal_one(least_cpu, busiest_cpu, STEAL_MIN_VICTIM_DEPTH)
            else {
                break;
            };

            // Determine the priority to use for enqueueing.
            let priority = self
                .state
                .get_task(stolen_id)
                .map(|sf| sf.priority as usize)
                .unwrap_or(TaskPriority::Normal as usize);

            // Place the task into the target CPU's run queue.  Route through
            // the wake mailbox when least_cpu is a remote CPU so that only the
            // owning CPU directly mutates its own run queue.
            let current_cpu = super::current_cpu_index::<R>();
            if least_cpu == current_cpu {
                self.state.enqueue_task(least_cpu, priority, stolen_id);
            } else {
                let now_tick = TICK_COUNT.load(Ordering::Relaxed);
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    least_cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: stolen_id,
                        priority,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
            }

            migrated += 1;
            PROF_PERIODIC_BALANCE_MIGRATIONS.fetch_add(1, Ordering::Relaxed);
        }

        if migrated > 0 {
            // Nudge the target CPU so it picks up the newly enqueued work
            // promptly.  The IPI is sent after releasing the SCHEDULER lock
            // (via the deferred bitmap mechanism) to avoid lock-order issues.
            self.queue_pending_prepare_schedule_ipi(least_cpu);
        }
    }

    pub(super) fn idle_steal(&mut self, local_cpu: usize) -> Option<TaskId> {
        if !cross_cpu_runq_migration_enabled() {
            return None;
        }

        // Snapshot online CPUs to break the immutable borrow on self.state
        // so we can call try_steal_one (&mut self) in the loop.
        let victims: alloc::vec::Vec<usize> = self.state.online_cpus.iter().copied().collect();

        // Try nearby CPUs first (locality heuristic).
        let start = local_cpu.saturating_sub(STEAL_NEARBY_RADIUS);
        let end = local_cpu.saturating_add(STEAL_NEARBY_RADIUS);
        for &victim in &victims {
            if victim == local_cpu || victim < start || victim > end {
                continue;
            }
            if let Some(id) = self.try_steal_one(local_cpu, victim, STEAL_MIN_VICTIM_DEPTH) {
                return Some(id);
            }
        }
        // Fall back to scanning all online CPUs.
        for &victim in &victims {
            if victim == local_cpu || (victim >= start && victim <= end) {
                continue;
            }
            if let Some(id) = self.try_steal_one(local_cpu, victim, STEAL_MIN_VICTIM_DEPTH) {
                return Some(id);
            }
        }
        None
    }

    pub(super) fn try_steal_one(
        &mut self,
        local_cpu: usize,
        victim_cpu: usize,
        min_victim_depth: usize,
    ) -> Option<TaskId> {
        if runq_depth_for_cpu(&self.state, victim_cpu) < min_victim_depth {
            return None;
        }

        // Victim scan: find a stealable task in priority order.
        for p in (0..TaskPriority::COUNT).rev() {
            let mut scan_depth = 0;
            let mut i = 0;
            let cpu_count = self.state.per_cpu.len();
            while i < self.state.per_cpu[victim_cpu].runq[p].len()
                && scan_depth < STEAL_SCAN_DEPTH_PER_PRIORITY
            {
                let id = self.state.per_cpu[victim_cpu].runq[p][i];
                let allowed = self
                    .state
                    .get_thread(id)
                    .map(|sf| match sf.affinity {
                        crate::task::Affinity::Any => true,
                        crate::task::Affinity::Pinned(_) => false,
                        crate::task::Affinity::Restricted(ref aff) => {
                            aff.allows(local_cpu, cpu_count)
                        }
                    })
                    .unwrap_or(false);
                if allowed {
                    let removed = self.state.per_cpu[victim_cpu].runq.remove_at(p, i);
                    if removed != Some(id) {
                        i += 1;
                        scan_depth += 1;
                        continue;
                    }
                    if let Some(sf) = self.state.get_thread_mut(id) {
                        // Check affinity. Restricted affinity must allow the local CPU.
                        // Success: remove from victim runq.
                        sf.runq_location = None;
                        sf.wake_cpu = Some(local_cpu);
                        self.state.note_enqueue_cause(id, state::EnqueueCause::Steal);
                        return Some(id);
                    }
                }
                i += 1;
                scan_depth += 1;
            }
        }
        None
    }
}

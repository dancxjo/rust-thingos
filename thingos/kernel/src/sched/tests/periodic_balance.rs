use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

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

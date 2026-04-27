use super::WAKE_LATENCY_HIST_BUCKETS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueCause {
    Unknown,
    Spawn,
    Wake,
    Steal,
    AffinityRepair,
    YieldRequeue,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskRuntimeStats {
    pub run_count: u64,
    /// CFS-style fairness debt approximation used by scheduler pick heuristics.
    ///
    /// Lower values indicate less accumulated CPU service; picker logic can
    /// prefer tasks with lower debt when effective priority ties.
    pub fair_vruntime: u64,
    pub migration_count: u64,
    pub migration_wake: u64,
    pub migration_steal: u64,
    pub migration_affinity: u64,
    pub migration_yield_requeue: u64,
    pub migration_other: u64,
    pub runs_since_last_migration: u64,
    pub runs_between_migrations_total: u64,
    pub min_runs_between_migrations: u64,
    pub max_runs_between_migrations: u64,
    pub wake_to_run_count: u64,
    pub wake_to_run_ticks_total: u64,
    pub wake_to_run_ticks_max: u64,
    pub wake_to_run_hist: [u64; WAKE_LATENCY_HIST_BUCKETS],
}

impl Default for TaskRuntimeStats {
    fn default() -> Self {
        TaskRuntimeStats {
            run_count: 0,
            fair_vruntime: 0,
            migration_count: 0,
            migration_wake: 0,
            migration_steal: 0,
            migration_affinity: 0,
            migration_yield_requeue: 0,
            migration_other: 0,
            runs_since_last_migration: 0,
            runs_between_migrations_total: 0,
            min_runs_between_migrations: u64::MAX,
            max_runs_between_migrations: 0,
            wake_to_run_count: 0,
            wake_to_run_ticks_total: 0,
            wake_to_run_ticks_max: 0,
            wake_to_run_hist: [0; WAKE_LATENCY_HIST_BUCKETS],
        }
    }
}

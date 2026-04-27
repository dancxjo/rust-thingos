use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

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

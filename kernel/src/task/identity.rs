//! Canonical/runtime/compatibility identity mapping helpers for schedulable entities.
//!
//! This module makes identity conversions explicit at call sites so code does not
//! rely on ad hoc `as` casts between runtime TIDs, canonical IDs, and
//! Unix-compatibility IDs.

/// Runtime scheduler identity (kernel thread/task ID).
pub type RuntimeTaskId = crate::sched::state::ThreadId;
/// Canonical stable task identity (`thingos.task.id`).
pub type CanonicalTaskId = thingos::kinds::TaskId;
/// Canonical stable job identity (`thingos.job.id`).
pub type CanonicalJobId = thingos::kinds::JobId;
/// Canonical stable group identity (`thingos.group.id`).
pub type CanonicalGroupId = thingos::kinds::GroupId;
/// Canonical stable space identity (`thingos.space.id`).
pub type CanonicalSpaceId = thingos::kinds::SpaceId;

/// Unix-compatibility IDs carried by the transitional `Process` projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatibilityIds {
    pub pid: u32,
    pub pgid: u32,
    pub sid: u32,
}

/// Canonical relation IDs associated with a task.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalTaskRelations {
    pub job: CanonicalJobId,
    pub group: CanonicalGroupId,
    pub session: CanonicalGroupId,
    pub space: CanonicalSpaceId,
}

/// Explicit identity layering for one runtime schedulable entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskIdentityLayers {
    pub runtime_task_id: RuntimeTaskId,
    pub canonical_task_id: CanonicalTaskId,
    pub compatibility: Option<CompatibilityIds>,
    pub canonical_relations: Option<CanonicalTaskRelations>,
}

/// Runtime task ID ⇄ canonical task ID is a stable 1:1 mapping.
#[inline]
pub const fn canonical_task_id_from_runtime_task_id(runtime_task_id: RuntimeTaskId) -> CanonicalTaskId {
    runtime_task_id
}

/// Runtime task ID ⇄ canonical task ID is a stable 1:1 mapping.
#[inline]
pub const fn runtime_task_id_from_canonical_task_id(canonical_task_id: CanonicalTaskId) -> RuntimeTaskId {
    canonical_task_id
}

/// Transitional mapping: compatibility PID currently backs canonical Job ID.
#[inline]
pub const fn canonical_job_id_from_compat_pid(pid: u32) -> CanonicalJobId {
    pid as CanonicalJobId
}

/// Transitional mapping: compatibility PGID currently backs canonical Group ID.
#[inline]
pub const fn canonical_group_id_from_compat_pgid(pgid: u32) -> CanonicalGroupId {
    pgid as CanonicalGroupId
}

/// Transitional mapping: compatibility SID currently backs canonical Group ID
/// for session identity.
#[inline]
pub const fn canonical_group_id_from_compat_sid(sid: u32) -> CanonicalGroupId {
    sid as CanonicalGroupId
}

/// Mapping from runtime `thingos::space::SpaceId` wrapper to canonical schema ID.
#[inline]
pub const fn canonical_space_id_from_runtime_space_id(
    runtime_space_id: thingos::space::SpaceId,
) -> CanonicalSpaceId {
    runtime_space_id.0
}

/// Leader-only mapping: PID maps to the leader Task ID for a process/job.
#[inline]
pub const fn canonical_leader_task_id_from_compat_pid(pid: u32) -> CanonicalTaskId {
    pid as CanonicalTaskId
}

/// Reverse mapping for compatibility paths. Returns `None` if the canonical ID
/// cannot be represented in the 32-bit compatibility namespace.
#[inline]
pub fn compat_pid_from_canonical_job_id(job_id: CanonicalJobId) -> Option<u32> {
    u32::try_from(job_id).ok()
}

impl TaskIdentityLayers {
    /// Identity map for a kernel-only thread (no Unix compatibility identity).
    pub const fn for_kernel_thread(runtime_task_id: RuntimeTaskId) -> Self {
        Self {
            runtime_task_id,
            canonical_task_id: canonical_task_id_from_runtime_task_id(runtime_task_id),
            compatibility: None,
            canonical_relations: None,
        }
    }

    /// Identity map for a task backed by a transitional `Process`.
    pub const fn for_process_task(
        runtime_task_id: RuntimeTaskId,
        compatibility: CompatibilityIds,
        runtime_space_id: thingos::space::SpaceId,
    ) -> Self {
        Self {
            runtime_task_id,
            canonical_task_id: canonical_task_id_from_runtime_task_id(runtime_task_id),
            compatibility: Some(compatibility),
            canonical_relations: Some(CanonicalTaskRelations {
                job: canonical_job_id_from_compat_pid(compatibility.pid),
                group: canonical_group_id_from_compat_pgid(compatibility.pgid),
                session: canonical_group_id_from_compat_sid(compatibility.sid),
                space: canonical_space_id_from_runtime_space_id(runtime_space_id),
            }),
        }
    }

    /// `true` if this layered identity points at the process/job leader task.
    pub const fn is_job_leader(self) -> bool {
        match self.compatibility {
            Some(compat) => {
                self.canonical_task_id == canonical_leader_task_id_from_compat_pid(compat.pid)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_and_canonical_task_id_round_trip() {
        let runtime = 42_u64;
        let canonical = canonical_task_id_from_runtime_task_id(runtime);
        assert_eq!(canonical, 42);
        assert_eq!(runtime_task_id_from_canonical_task_id(canonical), runtime);
    }

    #[test]
    fn compatibility_ids_map_to_canonical_relations() {
        let layers = TaskIdentityLayers::for_process_task(
            7,
            CompatibilityIds { pid: 7, pgid: 8, sid: 9 },
            thingos::space::SpaceId(11),
        );

        assert_eq!(layers.canonical_task_id, 7);
        assert_eq!(
            layers.canonical_relations,
            Some(CanonicalTaskRelations {
                job: 7,
                group: 8,
                session: 9,
                space: 11,
            })
        );
        assert!(layers.is_job_leader());
    }

    #[test]
    fn non_leader_task_is_not_job_leader() {
        let layers = TaskIdentityLayers::for_process_task(
            100,
            CompatibilityIds { pid: 7, pgid: 8, sid: 9 },
            thingos::space::SpaceId(11),
        );
        assert!(!layers.is_job_leader());
    }

    #[test]
    fn kernel_thread_has_no_compatibility_or_relations() {
        let layers = TaskIdentityLayers::for_kernel_thread(5);
        assert_eq!(layers.canonical_task_id, 5);
        assert_eq!(layers.compatibility, None);
        assert_eq!(layers.canonical_relations, None);
        assert!(!layers.is_job_leader());
    }

    #[test]
    fn canonical_job_id_to_compat_pid_handles_overflow() {
        assert_eq!(compat_pid_from_canonical_job_id(17), Some(17));
        assert_eq!(compat_pid_from_canonical_job_id(u64::from(u32::MAX) + 1), None);
    }
}

# Scheduler Identity Layering (Migration 7/12)

This document defines the identity layers used by scheduler-facing code and the
explicit helper mappings in `kernel/src/task/identity.rs`.

## Identity layers

| Layer | Type | Meaning | Stability |
|---|---|---|---|
| Runtime scheduler ID | `crate::sched::state::ThreadId` (`u64`) | ID used by runtime scheduler queues/dispatch | Stable in runtime domain |
| Canonical task ID | `thingos::kinds::generated::TaskId` (`u64`) | Canonical semantic identity for `thingos.task` | Stable canonical ID |
| Canonical job ID | `thingos::kinds::generated::JobId` (`u64`) | Canonical semantic identity for `thingos.job` | Stable canonical ID |
| Canonical group ID | `thingos::kinds::generated::GroupId` (`u64`) | Canonical semantic identity for `thingos.group` | Stable canonical ID |
| Canonical space ID | `thingos::kinds::generated::SpaceId` (`u64`) | Canonical semantic identity for `thingos.space` | Stable canonical ID |
| Unix compatibility IDs | `pid` / `pgid` / `sid` (`u32`) | Legacy compatibility namespace for Unix APIs | Compatibility-only |
| Kind identity | `thingos::kinds::generated::ThingId` (`[u8; 16]`) | Identifies kind/object references in schema payloads | Schema-level identity |

## Mapping rules (current migration phase)

- Runtime task ID and canonical task ID are a **1:1 mapping** (`u64` ↔ `u64`).
- Compatibility `pid` currently backs canonical `JobId`.
- Compatibility `pgid` currently backs canonical `GroupId`.
- Compatibility `sid` currently backs canonical session `GroupId`.
- Runtime `thingos::space::SpaceId` wrapper maps to canonical `SpaceId`.
- Compatibility `pid` maps to task identity only for the **job leader task**
  (`leader_task_id = pid as u64`).

## Explicit helpers

Use these helpers instead of ad hoc casts:

- `canonical_task_id_from_runtime_task_id`
- `runtime_task_id_from_canonical_task_id`
- `canonical_job_id_from_compat_pid`
- `canonical_group_id_from_compat_pgid`
- `canonical_group_id_from_compat_sid`
- `canonical_space_id_from_runtime_space_id`
- `canonical_leader_task_id_from_compat_pid`
- `compat_pid_from_canonical_job_id`
- `TaskIdentityLayers::{for_kernel_thread, for_process_task, is_job_leader}`

## Process adapter usage

`kernel::task::Process` exposes `identity_layers_for_tid` so scheduler and
compatibility paths can read runtime/canonical/compat IDs through a single
explicit access point.

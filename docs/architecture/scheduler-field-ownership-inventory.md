# Scheduler-Owned Field Inventory (Migration 1/12)

## Scope

Audited scheduler-facing data in:

- `kernel/src/sched/**`
- `kernel/src/task/mod.rs` (`Task`/`Thread`, `ProcessInfo`)
- `kernel/src/sched/hooks.rs` (`ProcessSnapshot`)

Classification labels used in tables:

- **hot scheduling state**
- **execution-runtime state**
- **canonical semantic state**
- **Unix compatibility projection**

---

## 1) `Task` inventory (`kernel::task::Thread<R>`, alias `Task<R>`)

| Field | Class | Notes / target owner |
|---|---|---|
| `id` | canonical semantic state | Canonical task identity (transitional backing for Task ID). |
| `state` | hot scheduling state | Scheduler lifecycle input (`Runnable/Running/Blocked/Dead`). |
| `priority` | hot scheduling state | Dynamic dispatch priority. |
| `exit_code` | canonical semantic state | Lifecycle/Job-exit truth (not scheduler-hot). |
| `exit_waiters` | execution-runtime state | Runtime wait queue mechanics. |
| `is_user` | execution-runtime state | Execution mode metadata. |
| `wake_pending` | hot scheduling state | Wake-vs-block race flag used in hot paths. |
| `pending_interrupt` | execution-runtime state | Runtime interrupt delivery state. |
| `affinity` | hot scheduling state | CPU eligibility constraint. |
| `kstack_base`, `kstack_size`, `kstack_top` | execution-runtime state | Kernel execution context storage. |
| `ctx` | execution-runtime state | Saved CPU context. |
| `aspace` | execution-runtime state | Active address-space token for dispatch. |
| `simd` | execution-runtime state | SIMD/FPU context payload. |
| `stack_info` | execution-runtime state | User stack metadata. |
| `mappings` | execution-runtime state | VM fast-path cache pointer. |
| `timeslice_remaining` | hot scheduling state | Preemption budget. |
| `last_cpu` | hot scheduling state | Locality hint for scheduling. |
| `name`, `name_len` | canonical semantic state | Canonical Task-display metadata (projection source). |
| `process_info` | canonical semantic state | Transitional link into Process/Job/Space projection backing. |
| `enqueued_at_tick` | hot scheduling state | Aging/fairness timestamp. |
| `base_priority` | hot scheduling state | Fairness baseline for boost rollback. |
| `user_fs_base` | execution-runtime state | Per-thread TLS base. |
| `detached` | execution-runtime state | Joinability runtime policy. |
| `signals` | Unix compatibility projection | Unix-thread signal compatibility state. |

---

## 2) Scheduler hot/cache inventory (`ThreadSchedFields` / `TaskSchedFields`)

Source: `kernel/src/sched/state.rs`

| Field | Class | Notes / target owner |
|---|---|---|
| `tid` | hot scheduling state | Scheduler index key for sorted cache; scheduler-owned index identity. |
| `runq_location` | hot scheduling state | Pure scheduler queue-placement state. |
| `state` | hot scheduling state | Cached mirror of `Thread::state`. |
| `priority` | hot scheduling state | Cached mirror of `Thread::priority`. |
| `affinity` | hot scheduling state | Cached mirror of `Thread::affinity`. |
| `last_cpu` | hot scheduling state | Cached mirror of `Thread::last_cpu`. |
| `timeslice_remaining` | hot scheduling state | Cached mirror of `Thread::timeslice_remaining`. |
| `enqueued_at_tick` | hot scheduling state | Cached mirror of `Thread::enqueued_at_tick`. |
| `wake_pending` | hot scheduling state | Cached mirror of `Thread::wake_pending`. |

`TaskSchedFields` is an alias to `ThreadSchedFields`; no separate ownership.

---

## 3) `ProcessInfo` inventory (`Process`, alias `ProcessInfo`)

Source: `kernel/src/task/mod.rs`

### Top-level `Process`

| Field | Class | Notes / target owner |
|---|---|---|
| `pid` | canonical semantic state | Transitional shared identity seam (future Job/Space split). |
| `lifecycle` | canonical semantic state | Lifecycle/Job backing (`ProcessLifecycle`). |
| `unix_compat` | Unix compatibility projection | Explicit Unix-compat quarantine (`ProcessUnixCompat`). |
| `thing_table` | execution-runtime state | Runtime FD/resource table (transitional backing). |
| `namespace` | canonical semantic state | Place/namespace context. |
| `cwd` | canonical semantic state | Place/cwd context. |
| `exec_path` | canonical semantic state | Authority-facing executable identity fallback. |
| `space` | canonical semantic state | Space/address-space backing (`ProcessAddressSpace`). |

### `ProcessLifecycle` fields

| Field | Class | Notes / target owner |
|---|---|---|
| `ppid` | canonical semantic state | Job parent linkage. |
| `thread_ids` | canonical semantic state | Job thread-group membership. |
| `exec_in_progress` | canonical semantic state | Job exec gate. |
| `children_done` | canonical semantic state | Job wait/reap queue payload. |
| `exit_observer_inbox` | canonical semantic state | Job exit-notification endpoint. |

### `ProcessAddressSpace` fields

| Field | Class | Notes / target owner |
|---|---|---|
| `mappings` | canonical semantic state | Space mapping ownership (shared Arc). |
| `aspace_raw` | canonical semantic state | Space raw aspace identity token. |
| `space_obj` | canonical semantic state | Canonical Space object + stable SpaceId. |

### `ProcessUnixCompat` fields

| Field | Class | Notes / target owner |
|---|---|---|
| `signals` | Unix compatibility projection | Unix process-signal compatibility state. |
| `message_inbox` | execution-runtime state | Transitional runtime inbox queue. |
| `pgid`, `sid`, `session_leader` | Unix compatibility projection | Unix session/group projection; future Group/Presence owners. |
| `argv`, `env`, `auxv` | Unix compatibility projection | Unix spawn/environment projection. |

---

## 4) Scheduler-facing snapshot inventory (`ProcessSnapshot`)

Source: `kernel/src/sched/hooks.rs`

| Field | Class | Notes / target owner |
|---|---|---|
| `pid`, `ppid`, `tid` | canonical semantic state | Identity/lifecycle snapshot projection from Task+Job backing. |
| `name` | canonical semantic state | Task name projection. |
| `state` | canonical semantic state | Task lifecycle projection for procfs/bridges. |
| `argv` | Unix compatibility projection | Unix argv projection from `Process.unix_compat`. |
| `exec_path` | canonical semantic state | Authority/path projection. |
| `exit_code` | canonical semantic state | Lifecycle/job-exit projection. |
| `pgid`, `sid`, `session_leader` | Unix compatibility projection | Unix group/session projection. |
| `cwd`, `namespace_label` | canonical semantic state | Place projection (namespace currently `"global"`). |
| `thread_states` | canonical semantic state | Job-state derivation input. |
| `space_id`, `space_mapping_count`, `space_sharing_count` | canonical semantic state | Space projection (diagnostic counts are non-atomic snapshots). |

---

## 5) Duplicated field map and recommended owner

| Semantic field | Current duplicates | Recommended target owner |
|---|---|---|
| Task identity (`tid/id`) | `Thread.id`, `ThreadSchedFields.tid`, `ProcessLifecycle.thread_ids`, `ProcessSnapshot.tid` | **Task canonical runtime record** (`Thread` transitional), with scheduler cache copy for indexing only. |
| Lifecycle state | `Thread.state`, `ThreadSchedFields.state`, `ProcessSnapshot.state`, `ProcessSnapshot.thread_states` | **Task canonical lifecycle state** on runtime task; scheduler/snapshot are mirrors/projections only. |
| Dynamic priority | `Thread.priority`, `ThreadSchedFields.priority` | **Scheduler hot cache** for dispatch-speed reads, synchronized from task runtime mutations. |
| Base priority | `Thread.base_priority` (+ implied via scheduler aging) | **Scheduler/runtime boundary**: keep on runtime task; scheduler reads to recompute effective priority. |
| Affinity | `Thread.affinity`, `ThreadSchedFields.affinity` | **Scheduler hot cache** mirror with runtime task as semantic source. |
| Last CPU | `Thread.last_cpu`, `ThreadSchedFields.last_cpu` | **Scheduler hot cache** mirror with runtime task as semantic source. |
| Timeslice budget | `Thread.timeslice_remaining`, `ThreadSchedFields.timeslice_remaining` | **Scheduler hot cache** authoritative in hot loop; runtime task kept synchronized for non-hot readers. |
| Enqueue timestamp | `Thread.enqueued_at_tick`, `ThreadSchedFields.enqueued_at_tick` | **Scheduler hot cache** authoritative for fairness accounting. |
| Wake-race flag | `Thread.wake_pending`, `ThreadSchedFields.wake_pending` | **Scheduler hot cache** authoritative for wake/block arbitration. |
| Exit status | `Thread.exit_code`, `ProcessSnapshot.exit_code` | **Job/lifecycle canonical state** (snapshot is projection). |
| Name | `Thread.name/name_len`, `ProcessSnapshot.name`, `thingos::task::Task.name` | **Canonical Task semantic metadata** with snapshot/public projection copies. |
| Process/group/session identity | `Process.pid`, `ProcessLifecycle.ppid`, `ProcessUnixCompat.{pgid,sid,session_leader}`, `ProcessSnapshot.*` | **Job + Group/Presence canonical owners**; Process/Snapshot remain adapters/projections. |
| Place context | `Process.{cwd,namespace}`, `ProcessSnapshot.{cwd,namespace_label}` | **Place canonical owner**; snapshot copy for read-only procfs/bridges. |
| Space identity/stats | `ProcessAddressSpace.space_obj` + `ProcessSnapshot.space_*` | **Space canonical owner**; snapshot is projection. |
| Unix argv/env/auxv | `ProcessUnixCompat.{argv,env,auxv}` + `ProcessSnapshot.argv` | **Unix compatibility projection boundary** only. |

---

## 6) Proposed hot scheduler cache boundary

**Boundary proposal:** scheduler hot path owns only `SchedState` + `ThreadSchedFields` + run queues (`PerCpu.runq`, `runq_location`, `current`, `need_resched`, sleep/wait queues) and may read/write only dispatch-critical fields:

- `state`
- `priority`
- `affinity`
- `last_cpu`
- `timeslice_remaining`
- `enqueued_at_tick`
- `wake_pending`

Everything else is outside hot cache boundary:

- Job/lifecycle semantics (`ppid`, `children_done`, exit observer wiring)
- Space/VM ownership (`space_obj`, mappings ownership semantics)
- Place/Authority semantics (`cwd`, namespace, exec path)
- Unix compatibility projection (`pgid/sid/session_leader`, `argv/env/auxv`, unix signals)

**Contract:**

1. Scheduler hot loop never dereferences `ProcessInfo` or Unix compatibility state.
2. Cache fields are synchronized mirrors of runtime task fields, except scheduler-owned queue placement (`runq_location`) and queue containers.
3. Snapshots (`ProcessSnapshot`) are read-only projections assembled outside dispatch-critical loops.

This is the migration seam for issues 5/12, 6/12, 8/12, and 10/12.

# Scheduler Typed-World Audit

**Issue:** Audit Scheduler Through Typed-World Lens  
**Status:** Initial audit  
**Scope:** `kernel/src/sched/*`, `kernel/src/task/*`, and typed-world architecture docs

---

## 1) Current concept inventory

### Classification summary

| Concept | Current scheduler/tasking role | Typed-world classification | Notes |
|---|---|---|---|
| `Task` | Public-facing schedulable unit in scheduler APIs and `thingos::task` | **Transitional → Canonical** | Correct canonical direction; still backed internally by `Thread<R>` |
| `Thread` / `ThreadId` / `ThreadState` / `ThreadPriority` | Internal kernel execution backing and identities | **Deprecated naming artifact** (external), **internal transitional backing** | Still leaks through aliases and docs/comments |
| `Job` | Lifecycle semantics represented by `ProcessLifecycle` and `kernel::job::bridge` | **Transitional → Canonical** | Owns parent/child linkage, exit/wait/reap, exec gating |
| `Space` | Address-space semantics represented by `ProcessAddressSpace` and `kernel::space::bridge` | **Transitional → Canonical** | Owns mappings/address-space identity, not scheduler runnable state |
| `Process` | Snapshot/procfs and compatibility-shaped spawn/wait surfaces | **Compatibility / Transitional** | Projection containing Job+Space+Task concerns |
| `PID` | Used as compatibility identity across process/task/job boundaries | **Compatibility / Transitional** | Shared identity remains a drift source |

### Required question answers (ownership semantics)

#### Is `Task` still the right canonical term?
Yes. `Task` is the right canonical scheduler-facing term. Scheduler state transitions (`Ready/Running/Blocked/Exited` via Task state mapping) and run-queue behavior are naturally Task semantics. `Thread` should remain implementation detail only.

#### Which semantics belong to Task?
- Schedulability and dispatch eligibility
- Execution context and CPU affinity/priority
- Blocking/wakeup state transitions
- Per-execution-unit interrupt/pending signal mechanics (until fully re-homed)

#### Which semantics belong to Job?
- Lifecycle identity and lineage (parent/child)
- Group-exit behavior and exec gating
- Exit status publication, wait/reap semantics
- Thread-group membership as lifecycle control data

#### Which semantics belong to Space?
- Address-space identity and mapping ownership
- VM map/protect/unmap semantics
- User stack/page-fault mapping context
- Sharing count/mapping count diagnostics

#### Which names are transitional aliases only?
- `TaskId` (currently aliasing `ThreadId` in `kernel/src/task/mod.rs`)
- `TaskPriority` (aliasing `ThreadPriority`)
- `TaskState` (aliasing `ThreadState`)
- `process_*` naming in scheduler hooks where canonical concept is Job/Space

#### Which scheduler APIs are canonical vs compatibility-facing?

**Canonical-facing scheduler APIs**
- `yield_now`, `sleep_ticks`, `block_current`, `wake_task`
- `spawn_user_thread`, `spawn_user_thread_ex`, `spawn_user_task_full`
- `task_status`, `task_wait`, `poll_task_exit`
- Core scheduler state and run-queue operations in `sched::types/state/blocking/sleep`

**Compatibility-facing or projection-heavy APIs**
- `boot_spawn_process`, `boot_spawn_process_ex`, `spawn_process_from_path`
- `spawn_process_current`, `spawn_process_ex_current`
- `waitpid_current`, `list_processes_current`, `process_info*_current`
- `ProcessSnapshot` fields carrying Unix compat (`pgid`, `sid`, `session_leader`)

---

## 2) Naming drift analysis

### A. Task-vs-Thread dual vocabulary inside scheduler/tasking
- `kernel/src/task/mod.rs` declares `TaskId = ThreadId`, `TaskState = ThreadState`, `TaskPriority = ThreadPriority`.
- This keeps compatibility but obscures that `Task` is canonical and `Thread` is backing.
- Result: reviewers and contributors can misread `Thread*` types as semantically canonical.

### B. Scheduler module docs still say “thread spawning”
- `kernel/src/sched/mod.rs` advertises both task and thread vocabulary (`spawn`: “Task and thread spawning”).
- This is transitional wording but currently unlabeled as such, so drift appears intentional rather than temporary.

### C. Process-heavy hook surface in `sched::hooks`
- Hook names and snapshots are dominated by `process_*` vocabulary while carrying canonical Job/Space data.
- `ProcessSnapshot` currently mixes canonical (`state`, `space_*`) and compatibility (`sid`, `pgid`) semantics in one shape.

### D. Spawn API split mixes canonical and compatibility intent
- `spawn_user_thread*` is canonical task-level behavior.
- `spawn_process*` helpers are projection-oriented and currently co-located with canonical scheduling operations.
- Co-location is pragmatic, but naming does not consistently mark compatibility status.

### E. Lifecycle and scheduling boundaries are conceptually correct but linguistically blurred
- `ProcessLifecycle` already models Job ownership seams well.
- However, scheduler callsites still frequently describe effects in process/thread terms even when behavior is actually Task+Job composition.

---

## 3) Concrete recommendations

### Terms to preserve
1. Preserve `Task` as the canonical public scheduler term.
2. Preserve `Job` for lifecycle ownership and `Space` for VM/address-space ownership.
3. Preserve bridge-centric wording (`kernel::job::bridge`, `kernel::space::bridge`, `kernel::task::bridge`) as the canonical projection path.

### Terms to quarantine
4. Quarantine `Thread*` naming to `kernel/src/task/*` internal backing only; require explicit transitional notes where exposed.
5. Quarantine `Process*` naming in scheduler hooks to compatibility/projection surfaces (`waitpid`, procfs/process snapshot APIs).

### Terms to rename or reinterpret
6. Reinterpret `TaskId` alias as “transitional Task identity backed by ThreadId” in the alias docs; make this explicit in scheduler-facing docs.
7. Rename scheduler module prose that says “thread spawning” to “task spawning” (with a one-line transitional note where needed).
8. Add compatibility marker language to `spawn_process*` docs (“compatibility-facing process projection API”) so canonical readers do not treat it as architectural truth.

### Comments/docs to update
9. Add a short “Canonical vs Compatibility scheduler API map” section to `kernel/src/sched/mod.rs` docs (or linked architecture doc) to classify exported functions.
10. Add an explicit note in `sched/hooks.rs` that `ProcessSnapshot` is a projection envelope containing mixed canonical + compatibility fields and should not be treated as canonical schema.
11. Cross-link this audit from `docs/architecture/concept-classification.md` and `docs/architecture/job.md` for follow-on cleanup issue generation.

---

## Follow-on cleanup issue seeds

1. **Task identity naming cleanup:** reduce direct `Thread*` type exposure in scheduler signatures and comments.
2. **Hook surface taxonomy:** split `sched::hooks` into canonical task/job/space hooks vs compatibility process hooks.
3. **Spawn API layering:** place compatibility `spawn_process*` entry points behind an explicit projection module or naming prefix.
4. **Snapshot decomposition:** derive typed `TaskSnapshot` / `JobSnapshot` / `SpaceSnapshot` views from `ProcessSnapshot` for canonical consumers.
5. **Review policy automation:** add lint/review checks blocking new non-annotated `Thread`/`Process` terminology in scheduler-facing docs.

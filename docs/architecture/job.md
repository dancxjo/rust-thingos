# Job as the Canonical Lifecycle Object

> **Status**: Authoritative lifecycle rule for process-like creation/exit/wait/reap semantics.
>
> Companion documents:
> - `docs/architecture/process-projection.md` — `Process` is projection, not ontology
> - `docs/architecture/unix-projection.md` — Unix surfaces are projected from canonical objects
> - `docs/concepts/process-lifecycle.md` — current lifecycle state-machine behavior
> - `docs/migration/process_responsibility_map.md` — extraction inventory and sequencing

---

## Rule

**Lifecycle truth belongs to `Job`.**

In ThingOS, lifecycle semantics are defined in `Job`, not `Process`:

- creation identity
- parent/child linkage
- exit semantics
- wait/reap semantics
- group-level execution continuity

`Process` remains a compatibility/transitional projection that may carry backing
state, but it is not the semantic owner of lifecycle behavior.

---

## 1) Canonical definition of `Job`

`Job` is the lifecycle owner for an execution container.

1. **Lifecycle identity**  
   `Job` is the canonical identity for lifecycle transitions (`New` → `Running` → `Exited`).

2. **Parent/child relations**  
   Parent/child linkage for spawn lineage and wait eligibility is `Job` state.

3. **Exit semantics**  
   Exit state and exit status are `Job` outcomes; compatibility process exit is derived from them.

4. **Wait/reap semantics**  
   Wait visibility, status delivery, and reaping consumption are `Job` semantics.

5. **Group-level execution continuity**  
   Thread-group continuity (leader/sibling lifecycle coupling and exec gating) is part of `Job` lifecycle control.

---

## 2) Relationship to other canonical objects

| Pair | Boundary |
|---|---|
| **Job vs Task** | `Task` is a schedulable execution unit. `Job` owns lifecycle across one-or-more Tasks (spawn lineage, exit, wait/reap, lifecycle gates). |
| **Job vs Space** | `Space` owns address-space/mapping identity. `Job` owns lifecycle identity. Shared `pid`/TGID usage is transitional and must not redefine ownership. |
| **Job vs Authority** | `Authority` owns permission/credential context. `Job` owns lifecycle/accounting; permission checks are not lifecycle truth. |
| **Job vs Group** | `Group` owns coordination (session/process-group style job control). `Job` owns creation/exit/wait/reap lifecycle. Group policies may affect running jobs but do not own lifecycle truth. |

---

## 3) Unix projection model (process lifecycle projected from `Job`)

Unix-visible process behavior is a compatibility projection:

`process lifecycle` = projection(`Job` lifecycle) + projection glue

- spawn/creation surfaces project `Job` parent/child and lifecycle identity
- `waitpid`/status surfaces project `JobWaitResult` / `JobExit`
- zombie/reap behavior projects `Job` wait/reap state transitions
- thread-group lifecycle continuity projects `Job` lifecycle gates and membership semantics

Review rule: if a lifecycle change is described first in `Process` terms and not
in `Job` terms, it is mis-scoped.

---

## 4) Migration inventory: current fields/hooks to `Job` ownership

### 4.1 ProcessLifecycle → Job semantic mapping

| Current location | Current field/hook | Intended `Job` ownership |
|---|---|---|
| `Process.lifecycle` (`ProcessLifecycle`) | `ppid` | parent/child linkage |
| `Process.lifecycle` (`ProcessLifecycle`) | `thread_ids` | group-level execution continuity / membership |
| `Process.lifecycle` (`ProcessLifecycle`) | `exec_in_progress` | lifecycle gate for exec/spawn-thread interlock |
| `Process.lifecycle` (`ProcessLifecycle`) | `children_done` | wait/reap queue ownership |
| `Process.lifecycle` (`ProcessLifecycle`) | `exit_observer_inbox` | lifecycle-exit event publication |

### 4.2 Closely-coupled lifecycle hooks to converge into Job

| Current location | Current field/hook | Intended `Job` ownership |
|---|---|---|
| `Thread<R>` | `exit_code` | canonical exit status payload |
| `Thread<R>` | `exit_waiters` | wait/reap wakeup mechanism |
| Scheduler + task-exit path (`mark_task_exited`, `waitpid`, `poll_task_exit`) | lifecycle transitions and status plumbing | canonical Job lifecycle transition and wait/reap interfaces |
| `kernel::job::bridge` | `job_*` mapping functions | canonical public projection surface while internals remain transitional |

---

## 5) Review enforcement checklist (lifecycle-specific)

Block or request changes when any answer is “no”:

- [ ] Is lifecycle behavior specified in `Job` terms first, with `Process` treated as projection/backing only?
- [ ] Do new lifecycle fields land in `ProcessLifecycle` (or a first-class `Job` object), never as new top-level `Process` lifecycle truth?
- [ ] Are exit/wait/reap semantics mapped through canonical `Job`/`JobExit`/`JobWaitResult` paths?
- [ ] Does new thread-group lifecycle logic describe `Job` ownership of continuity/gating semantics?
- [ ] If compatibility behavior changed, is the Unix process description explicitly documented as a projection from `Job`?

This checklist is applied alongside `docs/migration/review-guidelines.md`.

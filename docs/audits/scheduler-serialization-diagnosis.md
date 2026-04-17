# Scheduler architecture memo + findings (SMP serialization diagnosis)

## 1) Scheduler architecture memo

- **Run-queue structure**
  - Run queues are per-CPU: `SchedState.per_cpu[cpu].runq[priority]` (`kernel/src/sched/state.rs`).
  - Priorities are scanned locally (Realtime..Low, then Idle) in `prepare_schedule`.
  - There is no global runnable queue; however, scheduling decisions are guarded by one global `SCHEDULER` mutex.

- **CPU ownership model**
  - Each CPU has `current`, `idle_task`, and `need_resched`.
  - `Affinity::Pinned(cpu)` is enforced during pick; misrouted tasks are requeued to target CPU.
  - `Affinity::Any` placement is policy-driven:
    - during bringup: forced local CPU (`bringup_in_progress`)
    - steady state: round-robin across online secondary CPUs for process/task spawn
    - user-thread spawn currently defaults to **current CPU** for `Affinity::Any`.

- **Spawn path**
  - `spawn`/`spawn_user_task`: `pick_cpu_and_bringup()` + enqueue on chosen CPU.
  - `spawn_user_thread`: `Affinity::Any => current_cpu_index` (strong locality inheritance).
  - Remote wakeup from spawn is post-unlock (`nudge_spawned_task`) with resched IPI dedupe.

- **Wakeup path**
  - `wake_task_locked`:
    - blocked + `Affinity::Any` wakes to `last_cpu` (fallback current CPU)
    - blocked + pinned wakes to pinned CPU.
  - `wake_sleepers`:
    - `Affinity::Any` sleepers are wake-enqueued via RR online CPU policy.
  - Wakeups can request remote resched IPIs, coalesced by per-CPU pending flag.

- **Preemption path**
  - Timer/IPI call `try_resched_if_needed`.
  - If scheduler lock acquired: `schedule_point(PreemptTick)` may switch.
  - If lock contended: resched is deferred (`GLOBAL_NEED_RESCHED[cpu]`).

- **Reschedule IPI path**
  - Spawn/wake/misroute paths set pending-resched + send IPI 0x30 (deduped).
  - IPI handler calls `on_resched_ipi -> try_resched_if_needed`.

- **Idle-loop behavior**
  - `run_scheduler` loops on `yield_now`; if no work, `wait_for_interrupt`.
  - Idle wake pulses are trace-logged periodically.

- **Global locks / serialization points**
  - Main bottleneck is global `SCHEDULER` mutex around enqueue/pick/switch bookkeeping.
  - REGISTRY updates were partly de-nested/deferred, reducing but not removing contention risk.
  - `try_lock` miss path explicitly exists and can defer dispatch.

- **What must be true for “6 CPUs act like 1 busy CPU + 5 assistants”**
  - One orchestrator process dominates spawn/wakeup graph on one CPU.
  - `spawn_user_thread(Affinity::Any)` keeps children local to orchestrator CPU.
  - Frequent wakeups/dispatches hit scheduler-lock contention and defer on other CPUs.
  - Result: other CPUs run tasks sometimes, but critical dependency chain remains CPU-local.

## 2) Instrumentation added

- Per-CPU counters in scheduler state:
  - context switches
  - idle->non-idle transitions
  - timer interrupts
  - resched IPIs received
  - runnable enqueues/dequeues
  - wakeups
  - lock-blocked dispatch indicators and trylock miss bookkeeping
  - runq sample count/total (for avg runnable depth)
- Per-task scheduler CPU trace fields:
  - `last_cpu`, `wake_cpu`, `run_cpu`
- Existing lock wait/hold telemetry retained and complemented.
- 1s periodic debug summary (`SCHED-DBG`) emitted from tick path on CPU0.
- `dump_stats` extended to print per-task CPU trace fields and per-CPU diagnostics.

## 3) Reproduction tests added (focused unit-level proxies)

- **A: CPU-hog fanout proxy**
  - `test_spawn_any_affinity_fanout_after_bringup`:
  - validates post-bringup `Affinity::Any` spawn fanout across multiple CPUs.

- **B: Wakeup-storm locality proxy**
  - `test_wakeup_routes_to_last_cpu_for_any_affinity`:
  - validates unblock path re-enqueues Any-affinity blocked task to `last_cpu`.

- **C: Spawn-tree bias**
  - `test_spawn_user_thread_any_prefers_current_cpu`:
  - confirms user-thread Any-affinity locality inheritance to parent/spawner CPU.

- **D: Lock-pressure proxy**
  - `test_trylock_miss_records_pending_resched_pressure`:
  - validates per-CPU trylock miss + pending-resched miss accounting when lock is contended.

## 4) Findings report (current diagnosis)

- **What scheduler is supposed to do**
  - Per-CPU run queues, priority scheduling, remote wake via resched IPI.

- **What it actually does (from code + instrumentation design)**
  - SMP scheduling exists (per-CPU runqs and remote enqueue/IPI are real).
  - Early boot intentionally centralizes Any-affinity placement.
  - User-thread Any-affinity policy strongly preserves spawner locality.
  - Global scheduler lock remains a potential serialization point under heavy wake/schedule pressure.

- **“Single logical round-robin CPU” hypothesis**
  - **False in strong form** (design is not a single global run queue).
  - **Partly true in effect** for some workloads:
    - centralized orchestration + spawn locality + contention can collapse useful progress onto one CPU.

- **Narrowest correct diagnosis**
  - Main risk is **policy + orchestration centralization** (especially user-thread spawn locality), with **global scheduler lock contention** as secondary amplifier.

- **Recommended fixes (ordered)**
  1. Change `spawn_user_thread` Any-affinity policy from fixed current CPU to balanced/last-cpu-aware placement.
  2. Keep wakeup-to-last_cpu locality but add optional steal/rebalance when one CPU is overloaded.
  3. Use collected lock telemetry to split global scheduler lock hot path if contention remains high.
  4. Re-check sprout/orchestrator dependency chain to reduce single-service serialized startup work.

## 5) SMP 7/8 lock-splitting follow-up

- **Highest-contention hot path targeted**
  - `prepare_schedule` misroute requeue path in `kernel/src/sched/mod.rs` previously called
    `send_ipi` while the global `SCHEDULER` mutex was held.
  - This put cross-CPU nudge delivery in the enqueue/pick critical section and amplified
    lock hold time under misroute pressure.

- **Change made**
  - Misroute IPIs are now queued in `pending_prepare_schedule_ipis` under lock and sent only
    after unlocking (same pattern already used by `wake_sleepers`).
  - Lock-owning call sites (`try_resched_if_needed`, `yield_now`, `sleep_ticks`,
    `block_current`, `preempt_enable`, `resched_if_needed`, and `exit`) now drain/send this
    deferred queue after dropping `SCHEDULER`.

- **Telemetry expectation / before-after signal**
  - Under pressure workloads, lock hold histograms should shrink in the `yield_now` /
    `sleep_ticks` / preemption paths because remote IPI send latency is no longer paid while
    holding `SCHEDULER`.
  - Deferred dispatch pressure should improve indirectly (fewer long lock holds → fewer
    `try_lock` misses and blocked dispatch events).

## 6) SMP 8/8 orchestrator startup dependency de-serialization

- **Targeted startup chain audited**
  - `userspace/sprout/src/pipelines.rs::setup_ui_services`
  - Previous behavior spawned UI services in one serialized chain on one startup lane:
    - `placed -> flytrap -> blossom`

- **Before dependency graph**
  - `setup_ui_services`
    - `spawn placed`
    - `spawn flytrap`
    - `spawn blossom`

- **After dependency graph**
  - `setup_ui_services`
    - `spawn placed` (kept deterministic / first)
    - fan out in bounded parallel:
      - lane A: `spawn flytrap`
      - lane B: `spawn blossom`

- **Why this shape**
  - `placed` remains deterministic and first because it provides placement policy.
  - `flytrap` and `blossom` are independent of each other at spawn time, so they can
    start concurrently without changing critical display/input bring-up semantics.
  - Fanout is intentionally bounded to two additional startup tasks (no unbounded
    worker creation).

- **Expected diagnostics impact**
  - Startup traces should show overlapping `flytrap` + `blossom` spawn work instead of
    a single serialized UI chain, improving effective multi-CPU utilization during
    userspace bring-up.

## 7) Scheduler perf tracking issue (de-centralization + hot-path reductions)

- Tracking issue: [#311](https://github.com/dancxjo/thingos/issues/311)

### Child issue checklist (linked)

- [x] [#303](https://github.com/dancxjo/thingos/issues/303) — shrink `prepare_schedule()` lock-held work
- [x] [#304](https://github.com/dancxjo/thingos/issues/304) — remove O(n) runqueue removals in wake/block paths
- [x] [#305](https://github.com/dancxjo/thingos/issues/305) — de-centralize sleeper wakeups from CPU 0
- [x] [#306](https://github.com/dancxjo/thingos/issues/306) — simplify Any-affinity wake policy
- [ ] [#307](https://github.com/dancxjo/thingos/issues/307) — convert pending IPI dedup to bitmap
- [x] [#308](https://github.com/dancxjo/thingos/issues/308) — remove global sleep-queue scan on targeted wake
- [x] [#309](https://github.com/dancxjo/thingos/issues/309) — reduce registry coupling in picker path
- [ ] [#310](https://github.com/dancxjo/thingos/issues/310) — address sorted-thread-vector churn costs
- [x] [#312](https://github.com/dancxjo/thingos/issues/312) — follow-up closure for targeted wake sleep-queue scan removal
- [ ] [#313](https://github.com/dancxjo/thingos/issues/313) — remaining global scheduler lock contention reduction

### Phased rollout plan

- **Wave 1 (completed hot-path reductions):** #303, #304, #305, #306, #308/#312, #309
- **Wave 2 (open scalability reductions):** #307, #310
- **Wave 3 (de-centralization continuation):** #313 + follow-up per lock-contention data

### Benchmark record (SMP-heavy scenario)

Scenario: `x86_64` QEMU with `-smp 6`, scheduler telemetry enabled (`SCHED_TELEMETRY=1`), startup/orchestration-heavy boot path.

| Metric | Baseline capture | After capture | Status |
|---|---|---|---|
| Scheduler lock hold time (hist/max) | `sched_lock_metrics_snapshot_and_reset` (`prepare/yield/wake/sleep`) | same counters after each wave | Wave 1 captured in linked child issues; Wave 2/3 pending |
| Wake latency | scheduler wait/hold telemetry histograms (`wake_task`, `wake_sleepers`) | same counters after each wave | Wave 1 captured in linked child issues; Wave 2/3 pending |
| Throughput | runnable transitions + per-CPU runq depth and dispatch stats | same counters after each wave | Wave 1 captured in linked child issues; Wave 2/3 pending |

### Risk + correctness checklist (per change)

- [x] Wakeup correctness preserved (no duplicate runnable enqueue, no lost wake) in wake/block paths
- [x] Affinity routing correctness preserved (`Pinned` remains strict, `Any` keeps locality-default behavior)
- [x] Deferred IPI behavior preserves dedup/no-drop semantics
- [x] Scheduler/registry state synchronization remains coherent after picker-path decoupling
- [ ] Open-wave changes validated against stale-runq entries and task lifecycle churn (#307, #310)
- [ ] Re-check lock-contention regressions after Wave 2/3 with the same SMP-heavy scenario

### Remaining blockers and follow-up work

- `pending_prepare_schedule_ipis` still uses vector-style dedup in open path (#307).
- Sorted-thread storage churn remains open for spawn/exit-heavy workloads (#310).
- A broader global-lock contention reduction remains open (#313).

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

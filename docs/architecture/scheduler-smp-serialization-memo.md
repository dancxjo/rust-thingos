# Scheduler SMP Architecture Memo and Serialization Diagnosis

> **Status**: Architecture diagnosis memo for scheduler SMP behavior.
>
> **Related issue**: `Scheduler SMP 1/8`
> (`https://github.com/dancxjo/thingos/issues/197`)

---

## 1) Scheduler architecture (as implemented)

### Per-CPU run queues and local priority scan

- Runnable queues are per-CPU (`SchedState.per_cpu[cpu].runq[priority]`).
- Scheduling scans priorities locally in `prepare_schedule` (Realtime → Low, then Idle).
- There is no global runnable queue; placement and dequeue are still coordinated through a global scheduler lock.

### CPU ownership model and affinity routing

- Each CPU slot tracks `current`, `idle_task`, and reschedule state.
- `Affinity::Pinned(cpu)` is enforced; if a runnable is observed on the wrong CPU queue, it is routed back toward its target CPU.
- `Affinity::Any` placement is policy-based:
  - bring-up phase: localized placement while secondaries are still coming online,
  - steady state spawn paths: round-robin placement among online secondary CPUs,
  - user-thread spawn path currently inherits locality from the spawner CPU.

### Spawn, wakeup, preemption, resched IPI, and idle loop

- **Spawn**: task creation selects/targets a CPU and enqueues on that CPU; remote nudge is issued after lock release.
- **Wakeup**:
  - blocked `Affinity::Any` tends to wake to `last_cpu` (with fallback to current CPU),
  - optional overload-aware rebalance can redirect Any-affinity wakeups to a less-loaded online CPU when the preferred CPU run-queue depth is above a configurable gap,
  - pinned wakeups route to pinned target.
- **Preemption**: timer and resched-IPI paths call preemption checks; lock contention can defer immediate dispatch.
- **Resched IPI**: enqueue/wake paths can set pending-resched and send a deduped reschedule IPI to target CPU.
- **Idle loop**: CPUs repeatedly yield/schedule and then sleep waiting for interrupts when no runnable work is available.

---

### Any-affinity wake overload policy tuning

The Any-affinity wake path keeps `last_cpu` locality by default.

Build-time (compile-time) environment knobs:

- `THINGOS_SCHED_ANY_WAKE_POLICY=off|redirect|steal` (default: `off`)
- `THINGOS_SCHED_ANY_WAKE_OVERLOAD_GAP=<N>` (default: `4`)
- `THINGOS_SCHED_ANY_WAKE_OVERLOAD_STREAK=<N>` (default: `3`)

When policy is `redirect` or `steal`, wakeups stay on their preferred CPU by default and only enter least-loaded rebalance after `N` consecutive overload hits. This keeps the Any-affinity wake hot path local while still allowing optional rebalance on persistent overload. IPI signaling remains deduped via the existing pending-resched gate. These knobs are compile-time only today (via `option_env!`), not runtime-toggled.

## 2) Global serialization points and contention paths

- Global `SCHEDULER` lock serializes key scheduler bookkeeping (enqueue/pick/switch-adjacent state changes).
- Interrupt-side rescheduling uses `try_lock`; lock misses defer dispatch work until a later opportunity.
- Cross-CPU wakeups and orchestration-heavy workloads can amplify lock pressure by increasing global scheduling touch points.

---

## 3) Narrowest correct diagnosis

The scheduler design is genuinely SMP (per-CPU run queues, remote enqueue, and resched IPI are real), but some workload policies and dependency chains can still make the system behave like **one busy CPU plus assistant CPUs**.

### Assumptions required to reproduce the “1 busy CPU + assistants” effect

1. A dominant orchestrator/service dependency chain runs primarily on one CPU.
2. Work is spawned/woken in a pattern that preserves that CPU’s locality (especially user-thread `Affinity::Any` inheritance from current CPU).
3. Other CPUs receive work intermittently, but critical path dependencies remain anchored to the orchestrator CPU.
4. Scheduler lock contention rises under wake/preempt pressure, increasing deferred-dispatch windows on other CPUs.

### Policy-driven locality vs lock-contention amplification

- **Primary cause (policy/locality):**
  - locality-preserving spawn/wakeup policy can concentrate dependency-critical work on one CPU.
- **Secondary amplifier (serialization/lock):**
  - global scheduler lock contention worsens latency and reduces parallel effectiveness under pressure.

So, this is **not** a single global-run-queue scheduler in design, but it can become effectively one-CPU-dominant in specific policy + dependency conditions.

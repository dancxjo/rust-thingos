# Signals vs Typed Events / Messages

> **Status**: Architecture direction — compatibility demotion plan.
>
> This document defines Unix signals as compatibility semantics and establishes
> typed events/messages as canonical system truth.
>
> Companion documents:
> - `docs/architecture/concept-classification.md` — taxonomy (`signal` is Compatibility)
> - `docs/architecture/unix-projection.md` — Unix projection rules
> - `docs/architecture/ontology.md` — canonical owners (`Job`, `Group`, `Authority`, `Message`, `Presence`)

---

## 1. Current Signal Responsibilities Inventory

Current signal usage mixes multiple concerns:

1. **Lifecycle termination**
   - `SIGTERM`, `SIGKILL`, and related process-ending semantics.
2. **Interruption**
   - `SIGINT`-style "stop what you are doing now" requests.
3. **Job control**
   - group stop/continue and foreground/background control (`SIGTSTP`, `SIGCONT`).
4. **Child exit notification**
   - `SIGCHLD` for parent observation of child lifecycle transitions.
5. **TTY stop/continue behavior**
   - terminal-driven stop/continue flow (`SIGTTIN`, `SIGTTOU`, `SIGTSTP`, `SIGCONT`).

These are not one primitive. They are compatibility encodings of distinct
typed-world concerns.

---

## 2. Canonical Replacements

The canonical model is typed events/messages, with explicit ownership:

| Compatibility signal responsibility | Canonical model | Canonical owner |
|---|---|---|
| lifecycle termination | lifecycle event stream (`TerminateRequested`, `Terminated`, `ExitStatusReady`) | `Job` |
| interruption | typed control message to task/job inbox | `Message` + `Task`/`Job` |
| job control stop/continue | group coordination events (`GroupStopRequested`, `GroupResumed`) | `Group` |
| child exit notification | parent-observable lifecycle event (`ChildExited`) | `Job` |
| tty stop/continue behavior | presence/attachment control events (`ForegroundChanged`, `TtyReadBlocked`, `TtyWriteBlocked`) | `Presence` + `Group` |
| permission to send control notifications | explicit authorization checks | `Authority` |

Directionally:

- **Canonical truth**: typed lifecycle events and typed inbox messages.
- **Compatibility projection**: Unix signal numbers and default dispositions.

New subsystem behavior must be specified in canonical event/message terms first.
Signals are only an output/input compatibility shape.

---

## 3. Projection Policy (When Unix Signals Are Still Emitted)

Unix signal emission remains allowed only for compatibility boundaries:

1. **POSIX/Unix ABI contract**
   - If userspace ABI explicitly requires signal behavior, emit/project signals.
2. **Legacy userspace interoperability**
   - Existing apps that install handlers or expect specific signal numbers keep
     working through a bridge.
3. **Transitional bridge gaps**
   - If a typed event path is not yet fully implemented, a bounded bridge may
     temporarily emit signals, marked as compatibility.

Signals should **not** be used as architectural storage or canonical routing.
Canonical state lives in typed lifecycle/event/message owners.

Projection invariants:

- Every signal emission must map back to a canonical typed event/message cause.
- No new feature may be introduced in signal vocabulary first.
- Compatibility code stays quarantined (`ProcessUnixCompat`, bridge modules).

---

## 4. Bridge-Worthy vs Replaceable Signal Uses

### Bridge-worthy (keep as compatibility projection)

- POSIX-visible process termination notifications.
- Handler-facing app notifications for legacy userspace.
- TTY/job-control signal surfaces required by shell compatibility.

### Replaceable (move to canonical typed model)

- Internal kernel coordination currently routed as signal-style notifications.
- Non-POSIX subsystem wakeups/interruption that can be typed inbox messages.
- New lifecycle orchestration that can be expressed as structured `Job` events.

---

## 5. Signal-to-Inbox Pivot (Implemented)

### 5.1 Overview

Signal delivery now follows a **dual-delivery model** during the migration period:

1. **Inbox delivery** (canonical new model):
   `kernel::signal::inbox_bridge::deliver_signal_to_inbox` enqueues a typed
   `thingos.signal` message with `KindId::THINGOS_SIGNAL` into the target
   process inbox.

2. **Pending-bit delivery** (POSIX compatibility projection):
   `ProcessSignals::post` records the signal in the pending bitmask and wakes
   threads so legacy signal handlers still fire correctly.

Both paths run for every signal delivery (except SIGKILL/SIGSTOP; see §5.3).
Once applications are fully migrated to inbox-based event loops, step 2 will
be removed.

### 5.2 Payload Encoding (`thingos.signal`)

```
offset  size  field
──────  ────  ────────────────────────────────────────────────
   0      1   signum       — POSIX signal number (1–63)
   1      8   sender_tid   — LE u64, 0 = kernel/anonymous
   9      1   has_fault    — 1 if fault context follows, 0 otherwise
  10      8   fault_addr   — LE u64, faulting address (when has_fault=1)
  18      8   rip          — LE u64, instruction pointer (when has_fault=1)
  26      8   rsp          — LE u64, stack pointer (when has_fault=1)
```

The `thingos.signal.fault` field eliminates async-signal-safe constraints for
crash-class signal diagnostics.  A SIGSEGV message includes the exact fault
address and register state so an application's main event loop can log crash
details before the kernel terminates the process.

### 5.3 SIGKILL and SIGSTOP Bypass

SIGKILL and SIGSTOP are **not** delivered to the inbox.  They are uncatchable
by POSIX definition.  Delivering them to the inbox would allow a
buggy/malicious receiver to observe (and loop on) a forced-exit request that
must not be overridable.  These signals continue through the existing kernel
fast-path unchanged.

### 5.4 Fault-Originated Signals

Hardware-fault–sourced signals (SIGSEGV, SIGBUS, SIGILL, SIGFPE) are delivered
via `send_fault_signal_to_process` which populates the full
`SignalFaultContext` in the inbox message payload.

Exception-to-signal mapping for x86_64:

| Exception | Signal |
|---|---|
| Divide-by-zero (0), Overflow (4), FPU (16), SIMD (19) | SIGFPE |
| Bound Range (5), General Protection (13), Page Fault | SIGSEGV |
| Invalid Opcode (6) | SIGILL |
| Alignment Check (17) | SIGBUS |

---

## 6. Follow-On Work Areas

1. **Lifecycle path**
   - Normalize `SIGCHLD` and termination handling around canonical `Job` event
     publication, then project to signals only at compatibility edges.
2. **Group/TTY path**
   - Move job-control and tty stop/continue decisions behind `Group`/`Presence`
     events and keep signal numbers as shell-facing projection only.
3. **Authority path**
   - Centralize signal-send permission checks as explicit `Authority` policy
     reused by typed control-message delivery.
4. **Message path (completed)**
   - Typed inbox messages are now delivered for every non-uncatchable signal.
   - Fault-class signals include rich register/address context in the payload.
   - Removal of the legacy pending-bit path deferred until applications migrate.

---

## 7. Decision Summary

- `signal` is **Compatibility**, not architectural truth.
- Canonical direction is **typed events + typed messages**.
- Unix signals remain a **projection policy surface** for compatibility, not the
  system's fundamental event model.
- The `KindId::THINGOS_SIGNAL` message kind is the **canonical asynchronous
  notification path** for signal-like events; `ProcessUnixCompat.signals` is the
  **POSIX compatibility projection**.

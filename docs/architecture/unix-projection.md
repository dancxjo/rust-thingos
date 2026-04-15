# Unix Projection Model over the Typed-World Ontology

> **Status**: Authoritative reference — Phase 9 baseline.
>
> This document defines how Unix-visible concepts are projected from canonical
> typed-world concepts.  It exists to prevent compatibility surfaces from being
> mistaken for the underlying architecture.
>
> Companion documents:
> - `docs/architecture/concept-classification.md` — canonical/compatibility/transitional taxonomy
> - `docs/migration/concept-mapping.md` — detailed Unix → ThingOS lexicon
> - `docs/migration/bridge_architecture.md` — bridge layer conventions
> - `docs/concepts/unix-compat.md` — Unix session/env quarantine boundary
> - `docs/concepts/thingos-guardrails.md` — non-negotiable architecture rules

---

## Purpose

ThingOS organises system resources around a **typed-world ontology**: every
first-class object has a kind, an identity, and explicit relationships.  The
architectural primitives are `Thing`, `Place`, `Person`, `Authority`, `Group`,
`Job`, `Space`, `Task`, and `Message`.

Unix semantics — pathnames, file descriptors, processes, signals, sessions —
are **projected forms** layered on top of that ontology for compatibility.
They are not architectural truth.  Without an explicit projection model,
compatibility surfaces silently become the de facto system model, blocking the
long-term design.

This document establishes:

1. **Projection rules** — the invariants that govern every Unix-surface.
2. **Canonical mapping table** — each major Unix concept and its typed-world
   interpretation.
3. **Worked examples** — three current subsystems explained through the model.
4. **Review checklist** — a per-change guide for evaluating whether new API is
   "typed-first, Unix-second".

---

## 1. Projection Rules

> These rules apply to all code in `kernel/`, `abi/`, `bran/`, `stem/`, and
> `userspace/`.  They are not guidelines; they are constraints.

### R1 — Unix concepts do not define architectural truth

A Unix concept (process, signal, session, fd, pathname, …) is always a
**view** of one or more canonical typed-world objects.  The canonical object
exists independently of whether any Unix projection is exposed.

### R2 — A compatibility interface must project from typed-world concepts

No compatibility surface may be implemented by adding state directly to a
Unix-shaped struct.  All state behind a Unix surface must live in a canonical
typed-world owner (`Job`, `Space`, `Place`, `Group`, `Authority`, `Task`,
`Message`) and be exposed through a **bridge module**.

### R3 — New functionality must not be invented in Unix vocabulary

When a new OS capability is needed, it is specified using typed-world
vocabulary first.  The Unix projection (if any) is a secondary derivation.
Design reviews are conducted against the canonical concept, not the projection.

### R4 — Projection is explicit and bounded

Every Unix-projection surface is:

- visually distinct in code (annotated `// LEGACY COMPAT:` or lives in a
  named compatibility layer such as `ProcessUnixCompat`),
- traceable to a bridge module that is the single conversion point,
- documented in this file or a companion document linked from here.

### R5 — Compatibility bridges are transient

A bridge exists because the internal backing state has not yet been promoted to
a first-class object.  The bridge is expected to shrink as promotion progresses.
A bridge that grows is a regression.

---

## 2. Canonical Mapping Table

The table below covers every Unix concept mentioned in the issue, plus
additional concepts present in the current codebase.

| Unix concept | Typed-world interpretation | Current bridge / location |
|---|---|---|
| **pathname** | Address of a `Thing` within a projected `Place` (VFS tree is a `Place`-shaped view over the global mount table) | `kernel::vfs` path resolution; `kernel::place::bridge` |
| **file descriptor (fd)** | One reference form for a `Thing`; a Unix-compatible integer handle into the process FD table | `Process.fd_table` (quarantined); planned `Handle` concept (Phase 9+) |
| **process** | Compatibility projection over `Job` + `Space` + `Authority` + `Place` + one or more `Task`s | `Process` struct in `kernel/src/task/mod.rs`; decomposed via five bridge modules |
| **signal** | Compatibility projection over typed event/message semantics; decomposes into `Group` (job-control), `Authority` (dispositions), and `Message` (IPC) | `Process.unix_compat.signals` (`ProcessSignals` / `ThreadSignals`); quarantined in `ProcessUnixCompat` |
| **cwd** | One cursor within a `Place` projection (current working directory = the active position in a `Place`'s VFS tree) | `Process.cwd` → `kernel::place::bridge::place_from_snapshot` |
| **mount namespace** | One projected aspect of `Place`; the VFS tree visible from within a `Place` (currently a global stub: `NamespaceRef`) | `Process.namespace` (unit-struct stub); future: per-`Place` namespace |
| **session / process group** | Compatibility projection over `Group` (Foreground/Background kind) + `Presence` (terminal attachment) + `Place` coordination | `Process.unix_compat.pgid` / `.sid` / `.session_leader`; bridged via `kernel::group::bridge` |
| **PID** | Numeric handle; today backs both `TaskId` and `Job` identity (split deferred) | Low-level `pid: u32` fields; future: separate `JobId` + `TaskId` |
| **exec / argv / env** | Structured spawn record destined for `Job`; `env` blob is a future `Place`-context object | `Process.unix_compat.argv` / `.env` / `.auxv`; quarantined |
| **controlling TTY** | `Presence` (terminal attachment relationship between a `Person` and a `Place`) | `devfs::ConsoleTtyState::controlling_sid`; quarantined; future: `SYS_TTY_ATTACH` → `Presence` |
| **fork** | Eliminated; no `SYS_FORK` exists | Replaced by `SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC` |

---

## 3. Worked Examples

Three current subsystems are explained through the projection model.

---

### 3.1 — Process Lifecycle (Unix `fork`/`wait` → `Job`)

#### Unix surface

A program calls `fork()`, creating a child process with the same address space
and file descriptors.  The parent later calls `wait()` to reap the child's exit
status.

#### Typed-world reality

There is no `fork`.  The canonical lifecycle object is **`Job`** — a typed
first-class object that records creation, running, and exit.  A new execution
unit is created with `SYS_SPAWN_PROCESS[_EX]`, which allocates a fresh `Job`,
`Space`, `Authority`, `Place`, and at least one `Task`.

Exit-status collection is modelled as a `JobWaitResult`:

```rust
// thingos/src/job.rs — canonical output type
pub enum JobWaitResult {
    Exited(JobExit),   // job has finished; exit status available
    Running,           // job is still alive
}
```

#### Bridge

`kernel::job::bridge::job_from_snapshot(snap, tasks)` converts a
`ProcessSnapshot` and its `ThreadState` slice into a canonical
`thingos::job::Job`.  Procfs, IPC callers, and syscall handlers must all go
through this bridge — never read `ProcessLifecycle` fields directly.

```
ProcessLifecycle   (kernel-internal)
        │
        ▼
kernel::job::bridge::job_from_snapshot
        │
        ▼
thingos::job::Job  ──► procfs /proc/<pid>/job  |  IPC  |  SYS_WAIT response
```

#### Why this matters

The Unix surface (`fork`, `wait`, `SIGCHLD`) is implemented entirely as a
wrapper over `Job`.  If new lifecycle semantics are needed (e.g. structured
exit reasons, cooperative shutdown), they are added to `Job`, not to the
`Process`-shaped compatibility code.

---

### 3.2 — Signal Delivery (Unix signal → `Group` + `Authority` + `Message`)

#### Unix surface

A program calls `kill(pid, SIGTERM)`.  The kernel delivers the signal
asynchronously.  The target process may have installed a handler with
`sigaction`.

#### Typed-world reality

A Unix signal encodes three orthogonal concerns:

| Signal aspect | Typed-world owner | Example |
|---|---|---|
| Job-control stop/continue | `Group` (Foreground kind) | SIGTSTP, SIGCONT, SIGTTIN, SIGTTOU |
| Permission to send a signal | `Authority` (capability context) | `kill(-1, sig)` requires authority |
| Asynchronous notification payload | `Message` (typed event envelope) | SIGUSR1, SIGUSR2, application signals |

Currently all three aspects are quarantined inside `Process.unix_compat.signals`
(`ProcessSignals` and `ThreadSignals`) because the canonical replacement
concepts are not yet fully live.

#### Bridge (transitional)

`ProcessSignals` and `ThreadSignals` hold the current state.  Future work will
route:

- job-control signals through `kernel::group::bridge` and a `Group` membership
  check,
- authority checks for `kill()` through `kernel::authority::bridge`,
- application-level signals through `thingos::message::Message` delivery.

```
ProcessUnixCompat.signals   (quarantined — kernel internal)
        │
        │  (future)
        ├──► kernel::group::bridge     → Group membership / TTY foreground check
        ├──► kernel::authority::bridge → Authority.can_send_to(target)
        └──► Message delivery          → thingos::message::Message inbox
```

#### Why this matters

`kill(0, SIGTERM)` (signal own process group) already reads
`process.unix_compat.pgid`, which is itself quarantined because the proper
`Group` replacement is not yet complete.  Every call site that touches
`unix_compat.signals`, `.pgid`, or `.sid` is a visible signal that typed-world
replacement work is outstanding.

---

### 3.3 — Working Directory and Namespace (Unix `chdir`/`chroot` → `Place`)

#### Unix surface

A process calls `chdir("/home/user")`, changing its working directory.  A
container manager calls `chroot("/container/root")` to isolate the root.  A
container runtime calls `unshare(CLONE_NEWNS)` to create a private mount
namespace.

#### Typed-world reality

All three concepts are aspects of **`Place`** — the world-context object that
answers "in what world does this execution occur?":

| Unix concept | `Place` field | Current kernel backing |
|---|---|---|
| `cwd` | `Place.cwd` | `Process.cwd: Arc<Mutex<String>>` |
| VFS root | `Place.root` | `Process.root` (currently always `"/"`) |
| Mount namespace | `Place.namespace` | `Process.namespace: NamespaceRef` (global stub) |

#### Bridge

`kernel::place::bridge::place_from_snapshot(snap)` converts a
`ProcessSnapshot` into a canonical `thingos::place::Place`:

```rust
// kernel/src/place/bridge.rs
pub fn place_from_snapshot(snap: &ProcessSnapshot) -> thingos::place::Place {
    thingos::place::Place {
        cwd: snap.cwd.clone(),
        namespace: snap.namespace_label.clone(),
        root: alloc::string::String::from("/"),  // PROVISIONAL: always "/"
    }
}
```

Procfs exposes this at `/proc/<pid>/place`.

```
Process.cwd / .namespace / .root   (kernel-internal)
        │
        ▼
kernel::place::bridge::place_from_snapshot
        │
        ▼
thingos::place::Place  ──► procfs /proc/<pid>/place  |  IPC
```

#### Why this matters

When per-process namespace isolation is introduced, only `Process.namespace`
(and its stub `NamespaceRef`) needs to be replaced with a real namespace
object.  The `Place` canonical type, the bridge, and all public consumers
remain unchanged.  The Unix compatibility layer (`chdir`, `chroot`, `unshare`)
becomes a thin wrapper over `Place` mutation rather than a direct `Process`
field mutation.

---

## 4. Review Checklist for New Work

Use the following checklist when proposing or reviewing any change that touches
a Unix-visible surface:

### Step 1 — Identify the canonical concept

- [ ] What is the **typed-world owner** of the state being added or changed?
      (Choose from: `Thing`, `Place`, `Person`, `Authority`, `Group`, `Job`,
      `Space`, `Task`, `Message`, `Presence`.)
- [ ] Is this concept already canonical (use freely) or still transitional
      (mark `// PROVISIONAL:`)?  Consult
      `docs/architecture/concept-classification.md`.
- [ ] If the concept is "Unresolved", document the assumption explicitly and
      link it to a tracking issue before merging.

### Step 2 — Identify the bridge

- [ ] Does a bridge module exist for this concept?
      (See `docs/migration/bridge_architecture.md` — Bridge Inventory table.)
- [ ] If a bridge exists, is all new mapping logic going **through** the bridge
      rather than reading `Process`/`Thread` fields directly?
- [ ] If a new bridge is needed, does it follow the conventions in
      `docs/migration/bridge_architecture.md`?

### Step 3 — Identify the Unix projection

- [ ] What Unix surface does this change expose or modify?
      (e.g. a syscall, a procfs path, an fd operation, a signal, a pathname.)
- [ ] Is the Unix surface implemented as a **thin wrapper** over the canonical
      concept rather than as direct state manipulation on a Unix-shaped struct?
- [ ] Is the compatibility code visually distinct?
      - Compatibility state lives in `ProcessUnixCompat` and is accessed as
        `process.unix_compat.FIELD`.
      - Compatibility code is annotated `// LEGACY COMPAT:`.
- [ ] Does the change add new state to `ProcessUnixCompat` (or any Unix-compat
      layer) without a concrete compatibility requirement?  If so, redirect to
      the canonical concept.

### Step 4 — Typed-first, Unix-second check

Answer these two questions and record them in the PR description:

> **Canonical concept**: *What typed-world object does this change primarily
> affect?*

> **Unix projection**: *What Unix surface is exposed, and why is this surface
> a projection rather than canonical truth?*

If you cannot articulate a clear canonical concept, the design needs to be
revisited before the Unix surface is specified.

---

## 5. Anti-Patterns to Avoid

The following patterns have been observed and must be rejected in review:

| Anti-pattern | Why it is wrong | Correct approach |
|---|---|---|
| Adding a new field to `Process` (top-level, not a subdivision) | Grows the Unix god-object; unclear canonical owner | Identify the canonical owner (`Job`, `Space`, `Place`, etc.) and add the field there |
| Using `unix_compat.pgid` or `.sid` in new coordination logic | These are compatibility projections, not Group truth | Route through `kernel::group::bridge` |
| Reading `Process.cwd` directly in a new path-resolution code | Bypasses the Place abstraction | Read `Place.cwd` or call `kernel::place::bridge` |
| Inventing a new `SYS_*` syscall using Unix vocabulary as the design anchor | Bakes Unix semantics into new canonical surface | Design the canonical `Thing`/`Message`/`Authority` first; define the syscall as a projection |
| Expanding `ProcessSignals` for new notification types | Signals are a legacy projection; new notifications are `Message` | Use `thingos::message::Message` and the `Message` delivery path |
| Referring to `Thread` in new public types or documentation | `Thread` is a deprecated naming artifact | Use `Task` |
| Using `fork` semantics anywhere | `SYS_FORK` does not exist and must not be introduced | Use `SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC` |

---

## 6. Evaluating New APIs: "Typed-First, Unix-Second"

When a new API is proposed, the following test determines whether it is
typed-first:

1. **Remove all Unix terminology** from the API description.  Can the API still
   be completely specified using typed-world vocabulary (`Thing`, `Place`,
   `Job`, `Space`, `Authority`, `Group`, `Task`, `Message`, `Presence`)?

2. **Identify the canonical home** for every new piece of state.  Is every
   piece of state owned by a named canonical object?

3. **Identify the projection surface**.  After the canonical design is complete,
   how is it exposed through a Unix-compatible surface?  The projection surface
   should be derivable from, not equivalent to, the canonical design.

If step 1 fails — if the API only makes sense in Unix terms — the design must
be reworked at the typed-world level before any code is written.

---

## Related Documents

- `docs/architecture/concept-classification.md` — canonical/compatibility/transitional taxonomy
- `docs/architecture/process-projection.md` — `Process` as compatibility projection (not ontology)
- `docs/architecture/presence.md` — Presence design reference; session/tty/embodiment semantics
- `docs/migration/concept-mapping.md` — detailed Unix → ThingOS lexicon and migration guidance
- `docs/migration/bridge_architecture.md` — bridge layer design, conventions, and guardrails
- `docs/migration/review-guidelines.md` — actionable PR review checklist
- `docs/concepts/unix-compat.md` — Unix session/env quarantine boundary (ProcessUnixCompat)
- `docs/concepts/thingos-guardrails.md` — non-negotiable architecture guardrails
- `docs/concepts/process-lifecycle.md` — process lifecycle design
- `docs/kernel/signals.md` — signal subsystem implementation
- `docs/concepts/namespaces.md` — namespace semantics and roadmap
- `thingos/src/lib.rs` — canonical public crate; transitional mapping table
- `kernel/src/task/mod.rs` — `Process` struct and `ProcessUnixCompat` quarantine
- `kernel/src/place/bridge.rs` — Place bridge (Phase 8)
- `kernel/src/group/bridge.rs` — Group bridge (Phase 4)
- `kernel/src/job/bridge.rs` — Job bridge (Phase 3)
- `kernel/src/authority/bridge.rs` — Authority bridge (Phase 7)

# Concept Mapping: Legacy Unix → ThingOS

> **Status**: Living reference — Phase 9 baseline.
> This is the canonical lexicon for the Thing-OS conceptual migration.
> All naming decisions in new code, docs, and reviews should be grounded here.
>
> Companion documents:
> - `docs/architecture/ontology.md` — canonical typed-world ontology (architectural truth)
> - `docs/architecture/unix-projection.md` — Unix compatibility projection model over the ontology
> - `docs/architecture/concept-classification.md` — master classification table (Canonical / Compatibility / Transitional / Deprecated)
> - `docs/migration/review-guidelines.md` — PR review rules derived from this mapping
> - `docs/migration/process_responsibility_map.md` — field-level decomposition of `Process`
> - `docs/migration/bridge_architecture.md` — bridge layer design and conventions
> - `docs/concepts/thingos-guardrails.md` — architecture guardrails checklist

---

## Purpose

This document is the **single source of truth** for conceptual translation from
legacy Unix/Linux vocabulary to the ThingOS typed-world model.

It exists to:

- eliminate naming ambiguity across code and docs
- provide clear migration guidance for contributors and agents
- establish unambiguous review criteria ("is this using the right concept?")
- make conceptual entropy visible and reducible over time

This mapping is intentionally **non-peer**:

- **canonical typed-world truth** defines architecture
- **compatibility projection** defines Unix-facing behavior only
- **transitional decomposition seam** marks where legacy objects are being split
- **deprecated naming artifact** marks terms that must not spread

When in doubt about terminology, consult this document first, then verify the
canonical meaning in `docs/architecture/ontology.md` and projection boundaries
in `docs/architecture/unix-projection.md`.

---

## Canonical vs Compatibility Concepts

Treat this section as the constitutional split: canonical typed-world concepts
define architectural truth; Unix terms are compatibility projections only.

### Canonical typed-world concepts (architectural truth)

| Concept | Classification | Notes |
|--------|----------------|-------|
| `Thing` | Canonical typed-world truth | Core identity object across kernel and userspace |
| `Kind` | Canonical typed-world truth | Type/category semantics for `Thing` |
| `Form` | Canonical typed-world truth | Representation shape and projection form |
| `Place` | Canonical typed-world truth | World/context boundary (cwd/root/namespace projections) |
| `Person` | Canonical typed-world truth | Actor identity in typed-world ontology |
| `Authority` | Canonical typed-world truth | Permission and credential boundary |
| `Task` | Canonical typed-world truth | Schedulable execution unit |
| `Job` | Canonical typed-world truth | Lifecycle/accounting container |
| `Space` | Canonical typed-world truth | Address-space and memory-mapping ownership |
| `Group` | Canonical typed-world truth | Coordination/job-control domain |

### Emerging canonical concepts (under active decomposition)

| Concept | Classification | Notes |
|--------|----------------|-------|
| `Presence` | Transitional decomposition seam (canonical target) | Attachment/session/controlling TTY context is being extracted |
| `Message` / Event | Transitional decomposition seam (canonical target) | Typed notification model is still being split and stabilized |

### Compatibility-facing projections (bridge vocabulary)

`process`, `thread`, `signal`, `file descriptor`, `session`, `process group`,
`cwd`, `root`, `pathname`, and similar Unix terms are compatibility-facing
projection forms. They MAY appear in bridge/adaptor/compatibility layers and in
explicit legacy references, but they MUST NOT define new kernel architectural
meaning.

---

## Projection Rules

When translating Unix semantics, preserve the canonical object as source of
truth and treat Unix terms as view/projection forms:

- Unix path (`pathname`) projects a `Thing` relation resolved within a `Place`.
- Unix FD projects a resource/`Thing` reference through the handle-table seam.
- Unix process projects `Job + Space + Authority + Place + Task(s)`.
- Unix signals project typed coordination/notification semantics split across
  `Group`, `Authority`, and `Message`/Event boundaries.

---

## 1. Canonical Mapping Table

Read this table **left-to-right**: Unix terms are compatibility vocabulary; the
canonical column defines architectural reality.

| Legacy Unix term (compat vocabulary) | Canonical typed-world truth (defines reality) | Conceptual status | Transitional decomposition seam | Deprecated naming artifact | Notes |
|--------------------------------------|-----------------------------------------------|-------------------|---------------------------------|----------------------------|-------|
| Thread | Task | Canonical typed-world truth | `kernel::task::Thread<R>` backing `thingos::task::Task` | `Thread` outside kernel internals | Schedulable execution unit; canonical name is `Task` |
| Process | Job + Space + Authority + Place + Task(s) | Compatibility projection | `Process`, `ProcessLifecycle`, `ProcessAddressSpace` | — | No direct canonical peer; decomposes into first-class concepts |
| Address space | Space | Canonical typed-world truth | `ProcessAddressSpace` | — | First-class object target; extraction seam already named |
| File descriptor | Handle / Thing reference model (pending) | Compatibility projection | `Process.fd_table` | — | Integer FD remains Unix surface; canonical handle model is unresolved |
| Signal | Group + Authority + Message/Event split | Compatibility projection | `ProcessSignals`, `ThreadSignals` | — | Job-control/disposition/IPC aspects split across canonical concepts |
| Process group | Group | Compatibility projection | `pgid` via `kernel::group::bridge` | — | Group is the canonical coordination concept |
| Session | Group (`Foreground` kind) | Compatibility projection | `sid`, `session_leader` via group bridge | `Session` as a standalone core concept | Session semantics are absorbed into `Group` |
| PID | Task ID + Job ID split | Compatibility projection | `Process.pid` (TGID dual-use) | `PID` as canonical identity model | Split deferred until `Job`/`Space` are first-class |
| UID/GID | Authority (credentials) | Canonical typed-world truth | authority bridge path (field extraction pending) | — | Credential context belongs to `Authority` |
| Capabilities/privs | Authority | Canonical typed-world truth | authority bridge path (field extraction pending) | — | Permission context belongs to `Authority` |
| Working directory | Place (cwd cursor) | Compatibility projection | `Process.cwd` via `kernel::place::bridge` | — | CWD is one projection of `Place` |
| Mount namespace | Place (namespace projection) | Compatibility projection | `Process.namespace` / `NamespaceRef` | — | Namespace view is a projected aspect of `Place` |
| argv / env / auxv | Spawn record (not yet first-class) | Transitional decomposition seam | execution-context fields in `Process` | — | Migration target exists conceptually but extraction is unresolved |
| Fork | Spawn + exec model (`SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC`) | Deprecated naming artifact | — | `fork` / `Fork` | `SYS_FORK` is intentionally absent |
| TTY / controlling terminal | Presence (emerging) | Transitional decomposition seam | quarantined in process signal/session state | — | Canonical `Presence` integration is deferred |
| Port (IPC) | Port / Inbox (unresolved ontology) | Transitional decomposition seam | IPC boundary under active design | — | See `docs/ipc/inbox_vs_port_semantics.md` |

### Relationship types

| Type                  | Meaning |
|-----------------------|---------|
| **Equivalent**        | Same core semantics; renamed or made explicit |
| **Equivalent (refined)** | Same concept; ThingOS version is more precise or composable |
| **Split**             | One old concept becomes multiple new ones |
| **Merged**            | Multiple old concepts unify into one |
| **Eliminated**        | Concept removed entirely; use alternate mechanism |
| **Transitional**      | Mapping not yet stable; mark explicitly |
| **Unresolved**        | Open question; do not assume a mapping |

---

## 2. Detailed Concept Entries

### Thread → Task

**Legacy Thread:**
- Schedulable execution unit
- Implicitly bound to a `Process` (shares address space, file table, signals)
- Has stack, registers, and context
- Identified by a `ThreadId` (tid)
- State machine: Runnable → Running → Blocked → Exited

**ThingOS Task:**
- Schedulable execution unit
- References a `Space` explicitly (rather than inheriting from an owning process)
- May belong to a `Job` for grouping and lifecycle accounting
- Does not assume process membership (kernel threads have no `Job`)
- Identified by a `TaskId`/`ThreadId`
- State machine: Ready → Running → Blocked → Exited (same semantics, clarified naming)

**Current transitional backing:** `kernel::task::Thread<R>` is the internal implementation.
The public canonical surface is `thingos::task::Task`, produced by `kernel::task::bridge`.

**Relationship:** Equivalent (refined)

**Key differences:**
- Task does not imply process membership; `job` field is `Option<u32>`
- Task references Space explicitly (once Space is first-class)
- `ThreadId` and `TaskId` are currently the same type; may diverge

**Migration guidance:**
- Use `Task` in all new public-facing code and documentation
- Use `Thread<R>` only inside `kernel/src/task/` internals
- Do not introduce new `Thread`-named types outside the kernel internal layer
- Access task state via `kernel::task::bridge`, not `Thread` fields directly

---

### Process → Job + Space + Authority + Place + Task(s)

**Legacy Process:**
- Owns address space (virtual memory mappings)
- Owns one or more threads
- Owns the file descriptor table
- Carries credentials (uid, gid, capabilities — *not yet in ThingOS*)
- Carries signal dispositions and pending signal sets
- Carries session/group membership (`pgid`, `sid`)
- Carries working directory and namespace reference
- Carries spawn-time context (argv, env, auxv)
- Is the identity anchor for most kernel operations

**ThingOS decomposition (target):**

```text
Process (legacy)
 ├── Thread(s)        → Task(s)              [stable]
 ├── Address space    → Space                [target; seam: ProcessAddressSpace]
 ├── Lifecycle state  → Job                  [target; seam: ProcessLifecycle]
 │    ├── ppid
 │    ├── thread_ids
 │    ├── exec_in_progress
 │    └── children_done (waitpid queue)
 ├── Credentials      → Authority            [target; bridge in place]
 │    ├── name / exec_path  (current provisional backing)
 │    └── uid/gid/caps      (not yet added; planned prerequisite)
 ├── File table       → Handle table         [unresolved; fd_table quarantined]
 ├── Signals          → Event/Message + Authority + Group  [unresolved; quarantined]
 ├── cwd / namespace  → Place               [target; bridge in place]
 └── pgid / sid / session_leader → Group    [target; bridge in place]
```

**Relationship:** Split (into multiple first-class concepts)

**Migration guidance:**
- Do not introduce new `Process`-like god objects that aggregate multiple concerns
- Prefer composition: `Task` + `Space` + `Authority` + optional `Job`
- When adding new lifecycle state, add it to `ProcessLifecycle` (extraction seam for `Job`)
- When adding new VM/mapping state, add it to `ProcessAddressSpace` (extraction seam for `Space`)
- When adding new credential/permission state, add it to the authority bridge path
- The word `process` may appear in compatibility layers, Unix-compat code, and explicit
  legacy-semantic references — mark these with `// LEGACY COMPAT` or similar

---

### Address Space → Space

**Legacy address space:**
- Implicit owner: the `Process`
- Contains virtual memory mappings (anonymous, file-backed, MMIO)
- Identified by the process's page table root (a hardware token)
- No first-class kernel object; accessed via `Process.mappings`

**ThingOS Space (target):**
- First-class kernel object
- Explicit owner of all virtual memory mappings
- May be shared across multiple Tasks (shared-memory model)
- Contains `mappings: Arc<Mutex<MappingList>>` and `aspace_raw: u64`
- Identified by a `SpaceId`

**Current transitional backing:**
- `ProcessAddressSpace` subdivision inside `Process` (`Process.space`)
- No bridge module yet; `kernel::space::bridge` is planned

**Relationship:** Equivalent (explicit)

**Migration guidance:**
- Do not add new mapping/VM state directly to `Process`; add it to `Process.space`
- When `Space` is introduced as a first-class object, the `ProcessAddressSpace`
  subdivision becomes the promotion seam
- References to "address space" in comments should note the future `Space` concept

---

### File Descriptor → Handle / FD

**Legacy file descriptor:**
- Small integer index into the process's open-file table
- May reference files, pipes, sockets, devices, etc.
- Inherited across fork; closed-on-exec semantics

**ThingOS Handle / FD:**
- The FD integer representation is likely preserved at the POSIX compatibility surface
- A first-class `Handle` concept wraps the underlying resource reference
- Handle lookup is owned by `kernel::handle::bridge` over transitional tables
- Process `thing_table` remains the fd-compat table during migration

**Current state:**
- `thing_table: ThingTable` lives in `Process` as fd compatibility storage
- `kernel::handle::bridge` is the canonical lookup/conversion boundary
- Raw fd-style probing is being migrated behind that bridge

**Relationship:** Transitional (probably Equivalent with clarified model)

**Migration guidance:**
- New resource resolution code should use `kernel::handle::bridge` helpers
- Keep fd integer semantics at explicit compat boundaries only
- Continue migrating direct `thing_table`/IPC mixed probing into the bridge

---

### Signal → Event / Message (split)

**Legacy signal:**
- Asynchronous notification delivered to a process or thread
- Has three roles that ThingOS disaggregates:
  1. **Job control** (SIGSTOP, SIGCONT, SIGTTOU/SIGTTIN) → `Group`
  2. **Permission/disposition** (signal handlers, masks) → `Authority`
  3. **Inter-process notification** (kill, raise) → `Message`/Event system

**ThingOS direction (unresolved):**
- Job-control signals: `Group` (coordination domain)
- Signal dispositions and masks: `Authority` (permission context)
- Software notifications (IPC signals): `Message` or Event system
- The exact split boundary depends on IPC design (#44 alignment)

**Current state:**
- `ProcessSignals` and `ThreadSignals` are quarantined inside `Process`/`Thread`
- No bridge module yet; complex split deferred

**Relationship:** Split (unresolved boundary)

**Migration guidance:**
- Do not add new signal-like state to `Process` without consulting this document
- Use `Message` for new inter-task notification needs
- Mark any new signal-adjacent code with `// LEGACY COMPAT: signals`

---

### Process Group / Session → Group

**Legacy process group / session:**
- Process group (`pgid`): collection of processes receiving the same job-control signals
- Session (`sid`): collection of process groups sharing a controlling terminal
- Session leader: first process in a session

**ThingOS Group:**
- Coordination domain for purposes of signal delivery and TTY job control
- `GroupKind` distinguishes `Background` from `Foreground`
- Session semantics are folded into `Group`; no separate `Session` concept

**Current transitional backing:**
- `pgid`, `sid`, `session_leader` fields in `Process`
- Bridged through `kernel::group::bridge`

**Relationship:** Equivalent (refined) for Group; Merged for Session → Group

**Migration guidance:**
- Use `thingos::group::Group` for new group-awareness code
- Access group state via `kernel::group::bridge::group_from_snapshot`
- Do not read `pgid`/`sid` directly from `Process` in new code outside the bridge

---

### PID → Task ID / Job ID (split)

**Legacy PID:**
- Unique process identifier
- Doubles as thread-group leader ID (TGID) in Linux
- Used for signal delivery, waitpid, /proc paths

**ThingOS split (target):**
- **TaskId**: identity of a schedulable execution unit
- **Job ID**: identity of a lifecycle container (currently backed by PID/TGID)

**Current state:**
- `Process.pid` is the TGID and serves as both lifecycle identity and provisional address-space tag
- Split deferred until `Job` and `Space` are first-class

**Relationship:** Split (transitional)

See `docs/architecture/scheduler-identity-layering.md` for the current
runtime/canonical/compatibility ID conversion rules and helper APIs.

---

### Fork → (Eliminated)

**Legacy fork:**
- Creates a copy-on-write clone of the calling process

**ThingOS:**
- `SYS_FORK` does not exist and will not be added
- New processes are created with `SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC`
- POSIX `posix_spawn` semantics are emulated at the userspace level

**Relationship:** Eliminated

**Migration guidance:**
- Never add `SYS_FORK` or COW address-space duplication to the kernel
- If porting Unix code that uses fork-exec, translate to spawn + exec
- See `docs/concepts/thingos-guardrails.md` §4 for full rationale

---

## 3. Process Decomposition Map

```text
Process (legacy kernel object)
 │
 ├── Thread(s)
 │    └─→ Task(s)                      kernel::task::bridge  ✓ bridged
 │
 ├── Lifecycle state
 │    ├── ppid, thread_ids,
 │    │   exec_in_progress,
 │    │   children_done
 │    └─→ Job                          kernel::job::bridge   ✓ bridged
 │                                     ProcessLifecycle      ✓ grouped (seam)
 │
 ├── Address space
 │    ├── mappings, aspace_raw
 │    └─→ Space                        (planned)
 │                                     ProcessAddressSpace   ✓ grouped (seam)
 │
 ├── Credentials (planned)
 │    ├── uid, gid, caps (future)
 │    └─→ Authority                    kernel::authority::bridge  ✓ bridged
 │
 ├── File descriptor table
 │    ├── fd_table
 │    └─→ Handle table                 (not yet introduced)
 │
 ├── Signals
 │    ├── ProcessSignals, ThreadSignals
 │    ├── Job-control → Group
 │    ├── Dispositions → Authority
 │    └── IPC signals → Message        (all unresolved; quarantined)
 │
 ├── World context
 │    ├── cwd, namespace
 │    └─→ Place                        kernel::place::bridge  ✓ bridged
 │
 └── Coordination domain
      ├── pgid, sid, session_leader
      └─→ Group                        kernel::group::bridge  ✓ bridged
```

---

## 4. Naming Rules

These rules are **reviewable and enforceable**. PRs that violate them require
explicit justification.

### Banned from new kernel model code

| Term        | Banned context | Allowed context |
|-------------|----------------|-----------------|
| `Thread`    | New public types, new public functions, docs describing the canonical model | `kernel/src/task/` internals, `// LEGACY COMPAT`, compatibility layers |
| `Process`   | New canonical types, new public API shapes | Compatibility layers, `// LEGACY COMPAT`, explicit Unix-semantic references |
| `fork`/`Fork` | Any kernel code | Documentation describing why it is absent |
| `pid` (as a concept name) | New canonical types and docs | Low-level kernel numeric fields, `// LEGACY COMPAT` |

### Required in new code

| Semantic role              | Required term  | Where |
|----------------------------|----------------|-------|
| Schedulable execution unit | `Task`         | All new public types and docs |
| VM / memory identity       | `Space`        | All new VM-facing code |
| Lifecycle container        | `Job`          | All new lifecycle/accounting code |
| Permission context         | `Authority`    | All new credential/permission code |
| World/filesystem context   | `Place`        | All new cwd/namespace code |
| Coordination domain        | `Group`        | All new session/pgid code |

### Rust code examples

```rust
// BAD — introduces a Thread-named type outside the kernel internal layer
pub struct Thread { /* ... */ }

// GOOD — uses the canonical name
pub struct Task { /* ... */ }

// BAD — introduces a Process-like god object
pub struct Process {
    pub threads: Vec<Task>,
    pub address_space: Space,
    pub credentials: Authority,
    /* ... */
}

// GOOD — compose first-class concepts
pub struct Task { /* ... */ }
pub struct Space { /* ... */ }
pub struct Job { /* ... */ }
// compose them explicitly at call sites
```

### Transitional vocabulary (permitted explicitly)

When describing code that intentionally bridges old and new models, use explicit
transitional language rather than silently mixing terms:

- `process-like` — when describing something with Process-level scope but no
  clean canonical equivalent yet
- `Unix process semantics` — when describing legacy POSIX behavior being preserved
- `legacy process behavior` — when explaining compatibility choices
- `// PROVISIONAL:` — comment marker for bridge code that relies on transitional state
- `// LEGACY COMPAT:` — comment marker for code that preserves old semantics

---

## 5. Open Questions / Unresolved Mappings

The following areas are explicitly unresolved. Contributors must not assume a
stable mapping. Additions or resolutions should update this document.

| Area | Question | Dependency |
|------|----------|------------|
| Process → Job boundary | Is `Job` the direct equivalent of process lifecycle, or does it decompose further? | Process extraction phases |
| FD vs Handle vs Thing ref | What is the long-term resource reference model? Integer FD? Object handle? Thing reference? | Handle-table concept introduction |
| Signal split boundary | Where exactly does job-control end and IPC notification begin? | IPC design (#44) |
| Port vs Inbox semantics | Are ports and inboxes the same concept? | `docs/ipc/inbox_vs_port_semantics.md`, #46 |
| Place vs namespace/cwd/root | Is `Place` purely namespace ownership, or does it also define cwd/root/execution-world semantics? | Namespace work |
| Presence vs session/controlling TTY | Is `Presence` the canonical owner for attachment semantics that Unix modeled as session + controlling TTY, and if so does it own controlling TTY references directly or via a projection layer? | Presence model introduction |
| Kind/Form projection boundary | How do `Kind`/`Form` map to ABI types, wire formats, filesystem representation, and debug views? | ABI + debug view alignment work |
| Space sharing model | Can multiple Tasks share a `Space`? What is the ownership model? | Space extraction |
| Authority granularity | Does every Task have its own `Authority`, or is it shared within a `Job`? | Authority extraction |

---

## 6. Integration with Workflow

- This document is linked from `CONTRIBUTING.md`.
- All PRs touching `kernel/`, `abi/`, `bran/`, `stem/`, or `userspace/` should
  consult the naming rules in §4.
- The companion document `docs/migration/review-guidelines.md` provides a
  checklist form of the rules for use during PR review.
- When opening follow-on issues for unresolved mappings, reference this document
  in the issue body.

### Subsystem audit baseline (completed)

`kernel/src/task/` has been audited against this mapping:

- Canonical `Task` type is exposed through `kernel::task::bridge`.
- Transitional `Process` + `Thread<R>` internals are explicitly documented as seams.
- Inline references point reviewers back to this mapping for decomposition rules.

---

## Related Documents

- `docs/architecture/ontology.md` — canonical typed-world ontology (architectural truth)
- `docs/architecture/unix-projection.md` — Unix compatibility projection model over canonical ontology
- `docs/architecture/concept-classification.md` — master classification table (Canonical / Compatibility / Transitional / Deprecated)
- `docs/migration/review-guidelines.md` — actionable PR review checklist derived from this mapping
- `docs/migration/process_responsibility_map.md` — field-level decomposition and extraction sequencing
- `docs/migration/bridge_architecture.md` — bridge layer design and conventions
- `docs/migration/authority_inventory.md` — credential/permission field inventory
- `docs/migration/process_execution_context_inventory.md` — execution-context field inventory
- `docs/concepts/thingos-guardrails.md` — architecture guardrails (spawn+exec, VFS-first, etc.)
- `docs/concepts/process-object.md` — `Process` / `Thread<R>` struct design
- `docs/concepts/process-lifecycle.md` — state machine, exec, zombie semantics
- `docs/ipc/inbox_vs_port_semantics.md` — IPC port/inbox design discussion
- `kernel/src/task/mod.rs` — primary `Process` + `Thread<R>` structs
- `thingos/src/task.rs` — canonical public `Task` type
- `thingos/src/job.rs` — canonical public `Job` type

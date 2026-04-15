# ThingOS Concept Classification

> **Status**: Living reference — Phase 9 baseline.
> Use this table to decide whether a term belongs in new code, lives in a
> compatibility layer, is expected to disappear, or is a naming artifact that
> must not spread further.
>
> Companion documents:
> - `docs/migration/concept-mapping.md` — canonical Unix → ThingOS lexicon
> - `docs/migration/review-guidelines.md` — PR review rules derived from the mapping
> - `docs/concepts/thingos-guardrails.md` — architecture guardrails checklist

---

## Classification Key

| Classification | Meaning |
|---|---|
| **Canonical** | Architectural truth; use freely in new code and documentation |
| **Compatibility** | Must only appear in compatibility / Unix-compat layers |
| **Transitional** | Bridge concept; expected to be replaced or promoted; mark new uses with `// PROVISIONAL:` |
| **Transitional → Canonical** | Currently transitional but on the path to becoming canonical; treat as canonical in new code |
| **Deprecated naming artifact** | Legacy name that must not spread; use the canonical replacement instead |
| **Unresolved** | Open question; do not assume a stable definition |

---

## Master Classification Table

| Concept | Classification | Notes |
|---|---|---|
| `Thing` | **Canonical** | Core referent; any first-class object in the system |
| `Kind` | **Canonical** | Authoritative schema / type descriptor |
| `Form` | **Canonical** | Representation or projection of a Thing |
| `Place` | **Canonical** | Context of relation and visibility; cwd/namespace world-context |
| `Person` | **Canonical** | Actor; entity that acts upon Things |
| `Authority` | **Canonical** | Power/capability context; carries credentials and permission scope |
| `Group` | **Transitional → Canonical** | Coordination domain; replaces Unix process-group and session; bridged via `kernel::group::bridge` |
| `Message` | **Canonical** | Communication envelope; canonical inter-entity notification primitive |
| `Space` | **Transitional → Canonical** | Address-space identity; extraction seam is `ProcessAddressSpace`; becomes first-class in Phase 9 |
| `Job` | **Transitional → Canonical** | Lifecycle identity of an execution container; extraction seam is `ProcessLifecycle`; bridged via `kernel::job::bridge` |
| `Task` | **Transitional → Canonical** | Schedulable execution unit; public canonical surface in `thingos::task`; backed by kernel-internal `Thread<R>` |
| `Presence` | **Canonical / Emerging** | Deferred embodiment model; schema is stable; runtime integration deferred until Place and Group are stable |
| `Process` | **Compatibility / Transitional** | Unix projection; decomposes into `Job + Space + Authority + Place + Task(s)`; new code must not introduce new `Process`-shaped god objects |
| `Thread` | **Deprecated naming artifact** | Historical backing type (`kernel::task::Thread<R>`); must not appear in new public types or docs; use `Task` instead |
| `fd` | **Compatibility** | Unix file-descriptor integer; preserved at the POSIX compatibility surface only; `fd_table` is quarantined in `Process` |
| `signal` | **Compatibility** | Unix asynchronous event projection; decomposes into `Group` (job-control), `Authority` (dispositions), and `Message` (IPC); quarantined in `ProcessSignals` |
| `Session` | **Deprecated naming artifact** | Unix session concept; absorbed into `Group` (Foreground kind); `sid`/`session_leader` fields are quarantined |
| `Fork` | **Deprecated naming artifact** | Eliminated; `SYS_FORK` does not exist; use `SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC` |
| `PID` | **Compatibility / Transitional** | Unix numeric process ID; today backs both `TaskId` and Job ID; split deferred until `Job` and `Space` are first-class |
| `TaskId` | **Transitional** | Naming-sensitive; currently aliases the kernel `ThreadId`; will map to the identity of a schedulable `Task` once the split from Job ID is complete |
| `Port` | **Unresolved** | IPC surface concept; semantics under active design; see `docs/ipc/inbox_vs_port_semantics.md` |
| `Inbox` | **Unresolved** | IPC delivery concept; may merge with or remain distinct from `Port`; see `docs/ipc/inbox_vs_port_semantics.md` |
| `Handle` | **Unresolved** | Tentative replacement for the Unix fd model; a first-class resource reference; handle-table concept not yet introduced |
| `Channel` | **Unresolved** | IPC conduit concept; relationship to `Port` and `Inbox` not yet settled; see `docs/concepts/channel_semantics.md` |
| `KindClass` | **Canonical** | Coarse enumeration of first-class object categories (`Thing`, `Place`, `Person`); lives in `thingos::KindClass` |

---

## Rationale Notes

### Why `Process` is Compatibility / Transitional

`Process` is a Unix concept that bundles address-space ownership, lifecycle,
credentials, file-descriptor table, signal state, and session membership into a
single struct.  ThingOS disaggregates these concerns into orthogonal first-class
objects (`Space`, `Job`, `Authority`, `Place`, `Group`).  The word `process` may
appear in:

- **Compatibility layers** — code that explicitly emulates POSIX/Unix semantics.
- **`// LEGACY COMPAT:`-annotated code** — transitional bridges that are expected
  to be removed once the target concept is promoted.
- **Documentation** — when explaining why something is absent or how a migration
  is progressing.

New canonical code must not introduce new `Process`-shaped aggregates.

### Why `Thread` is a Deprecated Naming Artifact

`Thread<R>` is the kernel-internal backing type for a schedulable execution unit.
It lives in `kernel/src/task/` only.  The public name is `Task`.  Any appearance
of `Thread` outside `kernel/src/task/` or explicit `// LEGACY COMPAT:` annotations
is a bug.

### Why `Task` is Transitional → Canonical

`Task` is the canonical public name, but the underlying implementation (`Thread<R>`)
is still present.  `thingos::task::Task` is produced by `kernel::task::bridge` and
is the correct reference in all new code.  Once `Thread<R>` is fully replaced,
`Task` will be unambiguously canonical.

### Why `Space` and `Job` are Transitional → Canonical

Both concepts are defined and bridged, but the extraction from `Process` is
incomplete.  New code must use `Space` and `Job` vocabulary; the transitional
qualifier signals that the backing structs (`ProcessAddressSpace`,
`ProcessLifecycle`) are still inside `Process` rather than being standalone objects.

### Unresolved Cases

`Port`, `Inbox`, `Handle`, and `Channel` are explicitly unresolved.  Contributors
must not assume a stable meaning for these terms.  When an open question is settled,
this table should be updated and the corresponding issue closed.

---

## How to Use This Table in Reviews

1. **Is the new term Canonical?** — Use it freely.
2. **Is the new term Compatibility?** — Is the code in a compatibility or Unix-compat
   layer?  If yes, acceptable.  If no, request a canonical alternative.
3. **Is the new term Transitional (→ Canonical)?** — Mark uses with `// PROVISIONAL:`
   and confirm the code is moving toward the canonical direction.
4. **Is the new term a Deprecated naming artifact?** — Block the PR and request the
   canonical replacement.
5. **Is the new term Unresolved?** — Require an explicit note explaining why a
   decision was made and a pointer to the tracking issue.

---

## Related Documents

- `docs/migration/concept-mapping.md` — detailed Unix → ThingOS lexicon and migration guidance
- `docs/migration/review-guidelines.md` — actionable PR review checklist
- `docs/migration/bridge_architecture.md` — bridge layer design and conventions
- `docs/concepts/thingos-guardrails.md` — architecture guardrails (spawn+exec, VFS-first, etc.)
- `docs/concepts/process-object.md` — `Process` / `Thread<R>` struct design
- `docs/ipc/inbox_vs_port_semantics.md` — IPC port/inbox design discussion
- `thingos/src/lib.rs` — canonical public crate; transitional mapping table

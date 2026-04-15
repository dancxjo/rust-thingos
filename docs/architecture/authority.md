# Authority: Canonical Capability and Permission Context

> **Status**: Authoritative design reference — Phase 9+
>
> This document defines `Authority` as the canonical typed-world concept for
> capability, permission, and action legitimacy in ThingOS.  It exists to keep
> compatibility credentials (uid/gid, mode bits, session assumptions) as
> projections, not architectural truth.
>
> Companion documents:
> - `docs/architecture/ontology.md` — canonical concept boundaries
> - `docs/architecture/unix-projection.md` — Unix compatibility projection model
> - `docs/architecture/presence.md` — embodiment/session boundary
> - `docs/migration/authority_inventory.md` — current-state authority inventory

---

## 1. Canonical Definition

`Authority` is the canonical answer to:

> *Under what power is this action being performed?*

`Authority` is a first-class permission context.  It is the typed-world home for
legitimacy checks on actions, resource use, and capability-bearing operations.

### 1.1 What Authority owns

- Principal/authority identity for an action context (today represented by `name`)
- Active capabilities/rights set (today `capabilities: list<string>`)
- Future delegation/provenance chain (who granted this authority)
- Future policy bindings used by authorization gates

In current code, this maps to `thingos::authority::Authority` with bridge logic
in `kernel::authority::bridge`.

### 1.2 What Authority allows

Conceptually, Authority gates:

- Resource access decisions (open/read/write/invoke)
- Spawn/exec legitimacy and privileged lifecycle operations
- Permission to affect other execution units (signal/control operations)
- Capability transfer via permission-bearing references/handles

### 1.3 How Authority differs from Person and Presence

| Concept | Core question | Scope |
|---|---|---|
| `Person` | *Who is the actor?* | Durable identity |
| `Authority` | *What is this actor allowed to do here/now?* | Permission context |
| `Presence` | *How/where is this actor embodied or attached?* | Session/attachment context |

A `Person` acts **through** an `Authority`.  
A `Presence` may provide context to an authority decision, but is not itself a
capability grant.

---

## 2. Relationship Model

### 2.1 Person acts through Authority

A Person does not directly authorize actions.  Every permissioned operation is
evaluated against an Authority context.  This preserves identity (`Person`) and
legitimacy (`Authority`) as separate architectural axes.

### 2.2 Authority and Thing references

Thing references (including path/procfs/handle forms) can be permission-bearing:
possession may permit use, but legitimacy still conceptually lands in
Authority checks.  The check model is:

1. Resolve a Thing reference.
2. Evaluate requested operation against Authority.
3. Permit/deny and (future) record provenance.

### 2.3 Authority and Place

`Place` constrains *what world is visible* (cwd/namespace/root).  
`Authority` constrains *what can be done in that visible world*.

Effective permission is therefore contextual: `Authority` × `Place` (and, where
relevant, `Presence`/`Group` inputs).

### 2.4 Authority may constrain Form exposure

A Thing may have multiple Forms (text, binary, projected compatibility views).
Authority may constrain:

- whether a Form is visible at all
- whether a Form is readable vs writable
- whether redaction/projection is required for this Authority

This keeps "what representation can be seen/used" under canonical permission
control rather than ad hoc caller checks.

---

## 3. Unix Projection Analysis

Unix credentials are compatibility projections over Authority-oriented meaning.

| Unix concept | Authority interpretation | Notes |
|---|---|---|
| uid/gid | Identity facets that may become Authority principal fields | Not architectural root truth |
| permission bits (`rwx`) | One policy form consulted by Authority checks | Bits are representation; Authority is decision context |
| ownership checks (`st_uid`, `st_gid`) | Relationship query between Authority principal set and Thing policy | Must conceptually land in Authority gate |
| session privilege assumptions | Presence/Group context inputs to Authority decisions | Session membership alone does not grant authority |

Guardrail: introducing uid/gid-compatible fields is acceptable only as
Authority projections, not as a return to Unix-first design.

---

## 4. Where Future Checks Should Conceptually Land

| Decision type | Canonical check owner | Transitional location today |
|---|---|---|
| Privileged syscall (`reboot`, future admin ops) | `Authority` gate | `kernel::authority::bridge::check_privilege` (stub in Phase 7) |
| Cross-process signaling legitimacy | `Authority` (+ `Group`/`Presence` context where needed) | `kernel/src/syscall/handlers/signal.rs` with documented TODOs |
| Resource-open/access legitimacy | `Authority` + Thing policy + Place scope | Mixed/partially implicit across VFS/device paths |
| Spawn/exec legitimacy and inheritance | `Authority` ownership/delegation rules | Transitional process fields and bridge layer |
| Handle/reference transfer | `Authority` transfer/delegation semantics | Not yet explicit; currently implicit via possession |

This table is the review lens: if a change adds permission logic, reviewers
should ask whether the conceptual owner is `Authority`.

---

## 5. Current Subsystem Analysis Through the Authority Lens

### 5.1 Signal delivery (`kill` path)

Current state (`kernel/src/syscall/handlers/signal.rs`):

- `sys_kill` supports PID/PGID routing.
- It currently performs no sender/receiver authority relationship check.
- `pid == 0` path reads caller `pgid` from process Unix-compat state.
- File-level comments already mark this as transitional and direct future logic
  to `crate::authority::bridge`.

Authority lens interpretation:

- **Canonical owner of "may A signal B?" is Authority**, not raw process fields.
- `Group`/`Presence` can provide targeting context (process group, foreground),
  but final legitimacy belongs to Authority.
- Future implementation should route check points through explicit Authority
  gates before signal delivery.

---

## 6. Open Implementation Questions

1. **Principal shape**: What stable principal identifier should replace current
   name-derived authority identity?
2. **Capability model**: Should capabilities remain strings, evolve to typed IDs,
   or support both (typed core + string compatibility)?
3. **Delegation semantics**: How is capability transfer represented for spawned
   jobs/tasks and handle passing?
4. **Check API design**: Should authorization be expressed as
   `can(action, target, context)` or operation-specific predicates?
5. **Policy source of truth**: Where do Authority policies live (static config,
   runtime service, signed manifests, mixed model)?
6. **Auditing/provenance**: What event model records allow/deny decisions and
   delegation chains?
7. **Compatibility boundary**: How should uid/gid/session compatibility values be
   projected for POSIX-facing apps without becoming internal truth again?
8. **Form governance**: At what layer should per-Form redaction/visibility be
   enforced for procfs/debug/introspection surfaces?

These questions remain intentionally open; they should be resolved explicitly in
follow-on issues rather than hidden in ad hoc checks.

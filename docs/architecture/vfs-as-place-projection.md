# VFS as Place Projection and Thing Access Form

> **Status**: Design note and review guidance.
>
> This note clarifies how ThingOS VFS semantics map into the typed-world
> ontology. It complements:
> - `docs/architecture/ontology.md`
> - `docs/architecture/unix-projection.md`

---

## Purpose

ThingOS treats the VFS as a strong, practical substrate for interaction — but
not as ontology. The canonical model remains: **Things** (with **Kinds**) in
**Places**, exposed through one or more **Forms**.

The VFS is one projection of `Place` and one access form for `Thing`.

---

## 1. Projection model

### 1.1 Path lookup projects a relation in Place

Resolving a path (`/a/b/c`) is a projection operation over `Place`: it follows
visible relations from one Thing to another in the caller-visible namespace.

`lookup("/dev/tty0")` is therefore not "discovering truth from files"; it is
materializing a Place relation through filesystem syntax.

### 1.2 A node is an access form to a Thing

A VFS node (`VfsNode`) is one access form for an underlying Thing. Different
forms (VFS node, typed handle, IPC envelope) can refer to the same Thing.

### 1.3 `readdir` projects visible relations

Directory listing is a projection of which relations are currently visible in a
Place projection. It is a view, not an exhaustive ontology dump.

### 1.4 `stat` is one metadata form

`stat` provides one metadata form (`VfsStat` / `FileStat`) optimized for
filesystem compatibility. Canonical concept metadata is not required to be
defined only in `stat` terms.

### 1.5 `poll` is one event/wait form

`poll` readiness is one wait/event projection over underlying typed events and
resource state. It is an important substrate API, but not the only canonical
event model.

---

## 2. Non-goal boundary

**Non-goal:** "everything canonical must be only a filesystem concept."

Canonical concepts must be definable in typed-world terms even when no path,
inode-like metadata, or fd/poll surface is present. VFS projections are
required where they are the chosen compatibility/access substrate, but they do
not define system truth.

---

## 3. Current subsystem examples

These mounts are concrete Place projections currently used by ThingOS:

- **`/dev`** — device Things exposed as filesystem access forms (`open`, I/O,
  `poll` readiness).
- **`/proc`** — process/job/task state projected into readable compatibility
  forms; one representation of canonical lifecycle/place/group data.
- **`/sys`** — system and driver-oriented control/inspection forms projected via
  filesystem nodes.
- **`/services`** — service discovery and rendezvous projected into a mounted
  namespace for runtime composition.
- **`/session`** — session/runtime coordination surface (e.g. desktop files) as
  Place-visible state for UI/userland components.

---

## Review guidance

When reviewing VFS changes:

1. Identify the canonical owner first (`Thing`, `Place`, `Authority`, etc.).
2. Confirm the VFS behavior is an explicit projection from that owner.
3. Reject changes that introduce new meaning only in file/fd vocabulary.
4. Prefer adding canonical semantics first, then deriving VFS representation.


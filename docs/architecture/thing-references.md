# Thing References and Compatibility Reference Forms

> **Status**: Design and review guidance.
>
> This note defines Thing reference semantics independent of Unix, and clarifies
> how file descriptors/handles fit as compatibility-facing Forms.
>
> Companion documents:
> - `docs/architecture/ontology.md`
> - `docs/architecture/unix-projection.md`
> - `docs/architecture/vfs-as-place-projection.md`

---

## Purpose

ThingOS architecture is typed-world first: `Thing` identity is canonical truth.
A **Thing reference** is the semantic relation "subject may act on this Thing
through a specific authority/context".  That relation exists independent of any
particular encoding.

An fd/handle value is therefore a **reference Form** (projection/encoding), not
the referent itself and not the architectural identity of the object.

---

## 1. Canonical definition: Thing reference semantics (Unix-independent)

A Thing reference is:

1. A relation to a canonical `Thing` identity.
2. Scoped by context (`Place`, task/job context, and authority boundary).
3. Constrained by rights/capabilities (`Authority` and policy).
4. Potentially typed by intended operation set (read/write/control/message/etc.).

A Thing reference is **not**:

- a Unix fd integer,
- a path string,
- a socket number,
- or any single transport surface.

Those are encodings of the relation.

---

## 2. Reference Forms

Thing references may be projected through multiple Forms:

1. **fd / open-file reference**  
   Integer index in an fd table (`Process.fd_table` today), representing an
   open reference Form for a Thing (file/device/socket/channel endpoint).

2. **Capability-bearing message attachment**  
   IPC message payload/attachment carries a transferable reference Form with
   authority constraints.

3. **Typed relation handle**  
   A future typed handle (`Handle`) that explicitly carries Kind/rights metadata
   while still representing a reference relation, not architectural truth.

4. **Future non-filesystem references**  
   Additional Forms may appear (runtime registries, direct relation IDs,
   typed service/session references) without changing canonical semantics.

Multiple Forms can simultaneously refer to the same Thing.

---

## 3. Relationship to VFS and IPC

### VFS

VFS nodes and paths expose one access Form in a `Place` projection. `open()`
creates/returns a projected reference Form (fd) to an underlying Thing.

VFS does not define the referent; it exposes one representation of it.

### IPC

Channels/sockets/messages expose transport Forms for passing data and, where
supported, passing references. IPC attachments project Thing references across
task boundaries under authority checks.

IPC transport semantics do not define Thing identity; they carry/reference it.

### Thing tables and fd tables

Existing "thing table"/fd-table structures are allocation and lookup machinery
for reference Forms. They are compatibility/runtime indexing layers, not the
ontology source of truth for what the Thing is.

---

## 4. Review rules (typed-first, projection-second)

When reviewing changes to VFS, IPC, handles, or fd code:

1. **Identify the canonical Thing reference semantics first.**  
   What Thing is being referenced, in what scope, with what authority?

2. **Treat fd/handle as projection Form second.**  
   Reject designs where new semantics exist only because of fd integer shape.

3. **Do not introduce new architectural truth through fd semantics.**  
   New capabilities must be defined in typed-world terms, then projected.

4. **Keep VFS/IPC as exposure layers.**  
   Paths, sockets, and channels expose/carry references; they do not define
   the underlying referent model.

5. **Preserve multi-Form reasoning.**  
   If a proposal only works for one reference Form, require an explicit reason
   or redesign toward Form-independent reference semantics.


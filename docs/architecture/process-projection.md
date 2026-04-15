# Process as Compatibility Projection (Not Root Ontology)

> **Status**: Authoritative review rule for Issue:
> _Reframe Process as Unix Compatibility Projection Rather Than Architectural Truth_.

`Process` is **not** the architectural center of ThingOS. It is a compatibility-facing
projection assembled from deeper canonical concepts:

- `Job` (lifecycle)
- `Space` (address-space ownership)
- `Authority` (permission context)
- `Place` (world/visibility context)
- `Task`(s) (schedulable execution units)

New architectural meaning must land in those canonical owners first. `Process` is a
transitional shell and bridge backing object.

---

## 1) Top-level `Process` responsibility inventory

Field ownership classification for `kernel/src/task/mod.rs::Process`:

| Field | Classification | Canonical owner / notes |
|---|---|---|
| `pid` | lifecycle + space identity seam | Shared transitional identity (`Job` + `Space`) until split |
| `lifecycle` | lifecycle / Job | `ProcessLifecycle` extraction seam |
| `space` | address-space / Space | `ProcessAddressSpace` extraction seam |
| `namespace` | place | feeds `Place::namespace` via `kernel::place::bridge` |
| `cwd` | place | feeds `Place::cwd` via `kernel::place::bridge` |
| `exec_path` | authority (transitional) | authority-name fallback today; should not become generic process truth |
| `unix_compat` | Unix compatibility only | explicit quarantine (`signals`, `pgid`, `sid`, `argv`, `env`, `auxv`, etc.) |
| `thing_table` | transitional baggage | compatibility/resource table pending canonical Handle/Authority home |

`Task` ownership remains in `Thread<R>` (compatibility name) / `Task` concept, not in
top-level `Process` fields.

---

## 2) Allowed vs. disallowed future usage

### Allowed

- Compatibility projection helpers over canonical state.
- Bridge assembly glue needed to produce `Job` / `Space` / `Authority` / `Place` views.
- Transitional storage that is explicitly marked as extraction seam or baggage.

### Disallowed

- Introducing new architectural truth directly into top-level `Process`.
- Treating `Process` fields as canonical public API semantics.
- Adding new lifecycle/space/place/authority semantics without first defining them in
  canonical objects and bridge surfaces.

Rule of thumb for review: **if a change says “Process means X now,” it is wrong unless
X is explicitly compatibility-only.**

---

## 3) Review rules (must enforce)

When a PR touches `Process`:

- [ ] Is the new meaning owned by `Job`, `Space`, `Authority`, `Place`, or `Task` first?
- [ ] If `Process` changed, is it only projection/bridge wiring or explicitly transitional?
- [ ] If Unix compatibility state changed, is it confined to `unix_compat` with justification?
- [ ] Was any new top-level `Process` field added? If yes, block unless strictly transitional
      and tied to an extraction plan.
- [ ] Does documentation keep stating that `Process` is projection, not ontology?

---

## 4) Code anchor

- `kernel/src/task/mod.rs` contains the in-code enforcement comments and subdivisions.
- `docs/migration/review-guidelines.md` contains the PR checklist entries reviewers apply.

# How to Think in ThingOS

> **Audience**: contributors and coding agents making design, API, or review decisions.
>
> **Goal**: keep architectural truth typed-first, and keep Unix compatibility explicit and bounded.

ThingOS is not "Unix with new names." It is a typed-world system with Unix compatibility surfaces.

- **Architectural truth** lives in canonical concepts (`Thing`, `Place`, `Authority`, `Task`, `Job`, `Space`, `Message`, …).
- **Compatibility behavior** (`process`, `fd`, `path`, `signal`, `session`) is a projection over that truth.

If you remember one rule: **design in canonical concepts first; project Unix second (if needed).**

---

## 1) Truth layer vs compatibility layer

### Truth layer (what the system *is*)

- First-class typed objects with explicit ownership and relations.
- Stable meaning independent of any Unix API shape.
- Source of review decisions and long-term architecture.

### Compatibility layer (what legacy interfaces *see*)

- Unix-like surfaces exposed for portability and migration.
- Derived views over canonical state.
- Must not become the place where new architectural meaning is invented.

Quick test:

- If removing a Unix surface would destroy your concept, your concept is not canonical yet.

---

## 2) Canonical typed-world concepts (working set)

Use these as the default vocabulary when proposing designs:

- `Thing`: first-class managed object concept (architectural term, even if concrete implementation type names change).
- `Kind` / `Form`: what a Thing is vs how it is represented.
- `Place`: visibility/context world (where Things are reachable).
- `Authority`: permission/capability context.
- `Task`: schedulable execution unit.
- `Job`: lifecycle container (creation/exit/wait semantics).
- `Space`: address-space ownership context.
- `Group`: coordination domain (job control direction).
- `Message`: typed communication/event envelope.

If your design cannot name a canonical owner for new state, stop and resolve ownership first.

---

## 3) How to evaluate a new design idea

Run this check in order:

1. **Canonical owner first**
   - Which canonical concept owns the new meaning?
2. **Typed API before Unix API**
   - Can the behavior be expressed without Unix terms?
3. **Projection boundary**
   - If Unix compatibility is needed, where is the single bridge/projection point?
4. **Guardrail alignment**
   - Does it respect scheduler-first, userland drivers, VFS-first, spawn+exec?
5. **Bridge pressure**
   - Does the change shrink compatibility debt, or make compatibility state grow?

Reject designs that start with "add field to `Process`/fd/session and figure ontology later."

---

## 4) What Unix concepts mean here

Treat Unix words as *views*, not ontology:

- **process** → projection over `Job + Space + Authority + Place + Task(s)`
- **thread** → historical implementation name; public concept is `Task`
- **fd** → compatibility reference form for a `Thing`
- **path** → address/projection within a `Place`-shaped VFS view
- **signal** → compatibility projection over typed coordination/message semantics
- **session / process group** → compatibility projection over `Group` (+ `Presence` direction)

Unix terms are allowed at explicit compat boundaries. They are not where new core meaning should originate.

---

## 5) Common conceptual traps

1. **"Unix exists, so Unix is architecture"**
   - Wrong. Compatibility presence is not architectural authority.
2. **"Process is the root object"**
   - Wrong. `Process` is transitional projection glue.
3. **"Path is identity"**
   - Wrong. Path is one projection route, not object ontology.
4. **"Add to compat now, untangle later"**
   - Usually becomes permanent debt. Require canonical ownership now.
5. **"No immediate Unix mapping, so the feature is invalid"**
   - Invert that: canonical design first; Unix mapping is optional and secondary.

---

## 6) Example: process is not the root object

Bad framing:

- "Process owns lifecycle, address space, permissions, cwd, and scheduling semantics."

ThingOS framing:

- `Job` owns lifecycle semantics.
- `Space` owns address-space semantics.
- `Authority` owns permission semantics.
- `Place` owns visibility/context semantics.
- `Task` owns schedulable execution semantics.
- `Process` (where present) is a compatibility/transitional shell that projects these together.

Review implication: new semantics should land in the canonical owner, then be bridged if a process-shaped interface is required.

---

## 7) Example: path is a projection, not ontology

Bad framing:

- "The path *is* the resource."

ThingOS framing:

- The resource is a `Thing` with a `Kind`.
- A path is a context-dependent route to that Thing inside a `Place` projection.
- Different places/projections can expose different routes to the same underlying object.

Review implication: don't attach canonical identity or authority semantics to string paths alone.

---

## 8) PR/review quick prompts (humans + agents)

Before approving or merging:

- What canonical concept owns this new meaning?
- Is any Unix term being used as architecture rather than projection?
- Is compatibility wiring explicit, bounded, and documented?
- Did this change reduce or increase compatibility debt?
- If all Unix names were hidden, would the core design still be coherent?

If these answers are weak, the design is not ready.

---

## Related references

- `docs/architecture/ontology.md`
- `docs/architecture/unix-projection.md`
- `docs/architecture/concept-classification.md`
- `docs/concepts/thingos-guardrails.md`
- `docs/migration/review-guidelines.md`

# ThingOS

ThingOS is a typed-world operating system project.

Its architecture is organized in two layers:

1. **Typed-world ontology (truth layer)** — canonical system meaning
2. **Unix compatibility (projection layer)** — adaptation for existing software and conventions

Unix is a projection surface in ThingOS, **not** the ontology.

---

## 1) Typed-world ontology

In ThingOS, first-class system meaning is defined in typed concepts.

Core concepts include:

- **Thing**
- **Kind**
- **Form**
- **Place**
- **Person**
- **Authority**
- **Presence**
- **Task**
- **Job**
- **Group**
- **Space**
- **Message**

These concepts are canonical: design and review decisions are anchored to them first.

Reference: [`docs/architecture/ontology.md`](docs/architecture/ontology.md)

---

## 2) Canonical concepts in practice

At a high level:

- A **Thing** is a first-class object.
- A **Kind** defines structure and meaning.
- A **Form** is how a Kind is represented.
- A **Person** acts through **Authority**, inhabits a **Place** via **Presence**, and manipulates **Things**.
- Execution decomposes into **Task** (runs), **Job** (lifecycle), and **Group** (coordination).

The goal is typed-first ownership of meaning, not ad-hoc Unix-shaped state.

---

## 3) Unix projection model

ThingOS still exposes Unix-visible surfaces (pathnames, file descriptors, processes, signals, sessions, etc.) for compatibility.

Those surfaces are treated as **projections** derived from canonical typed-world concepts. They are implementation bridges, not architectural truth.

Reference: [`docs/architecture/unix-projection.md`](docs/architecture/unix-projection.md)

---

## 4) Current implementation status (honest snapshot)

Current kernel/userspace code still contains substantial Unix-shaped compatibility structures and naming (for example, process/fd/path/session surfaces).

That does **not** mean Unix is the model; it means the project is in an active transition where compatibility bridges exist while canonical typed ownership is being tightened.

The architecture docs are the source of truth for intended direction and review standards:

- [`docs/architecture/ontology.md`](docs/architecture/ontology.md)
- [`docs/architecture/unix-projection.md`](docs/architecture/unix-projection.md)
- [`docs/architecture/concept-classification.md`](docs/architecture/concept-classification.md)
- [`docs/concepts/thingos-guardrails.md`](docs/concepts/thingos-guardrails.md)

---

## 5) Near-term direction

Near-term work continues to:

- keep new capability design typed-first
- confine Unix semantics to explicit projection/compatibility layers
- reduce compatibility-bridge surface area as canonical concepts mature
- make subsystem ownership clearer against canonical concepts

In short: **typed-world ontology remains canonical; Unix remains projection.**

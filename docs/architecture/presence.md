# Presence: First-Class Model for Session, Attachment, and Embodiment Semantics

> **Status**: Authoritative design reference — Phase 9+
>
> This document defines `Presence` as a canonical typed-world concept and maps
> Unix session, controlling-terminal, and attachment semantics onto it.  It
> exists so that compatibility projections related to session membership, TTY
> ownership, foreground/background interaction, and future GUI embodiment have a
> named, typed home rather than being scattered across Unix-compat quarantine
> fields.
>
> Companion documents:
> - `docs/architecture/concept-classification.md` — canonical/compatibility/transitional taxonomy
> - `docs/architecture/unix-projection.md` — full Unix → typed-world projection model
> - `docs/concepts/presence.md` — schema overview and deferred integration rationale
> - `docs/migration/concept-mapping.md` — detailed Unix → ThingOS lexicon
> - `docs/concepts/unix-compat.md` — Unix session/env quarantine boundary

---

## 1. What Is Presence?

`Presence` is the canonical answer to the question:

> *How does an entity participate in the system at a particular moment — where,
> in what social context, through what kind of attachment?*

A `Presence` record describes a single **situated participation** of a subject
(typically a `Person`, later any entity) within the system's typed-world model.
It is the home for all semantics that belong to neither "where execution happens"
(`Place`) nor "who is capable of acting" (`Authority`) nor "what execution is
running" (`Job`/`Task`), but instead to the **person-in-place relationship**
itself.

Key properties of Presence:

| Property | Meaning |
|---|---|
| `subject` | The entity that is present (who) |
| `place` | The world-context in which it is present (where) |
| `group` | The coordination domain anchoring this presence (what context) |
| `mode` | Quality of presence: Active, Latent, Remote, Projected, Deferred, Unavailable |
| `embodiment` | How the presence is manifested: Direct, Remote, Inherited, Symbolic, Deferred |
| `observed_at` | When this record was last known valid (optional Unix ms) |

Presence is defined first as a **schema and conceptual boundary** and second as
a runtime system.  Runtime integration is deferred until `Place` and `Group`
foundations are stable.  See `docs/concepts/presence.md` for the full schema.

---

## 2. How Presence Differs from Person

`Person` identifies *who* the entity is — a stable, durable identity that exists
whether or not the entity is currently interacting with the system.

`Presence` describes a *momentary or session-scoped participation* of a `Person`
within the system.  The same `Person` may have zero, one, or many `Presence`
records simultaneously (local terminal session, remote SSH session, GUI session).

| Concept | Answers | Durable? | Cardinality |
|---|---|---|---|
| `Person` | Who is this entity? | Yes — stable identity | One per actor |
| `Presence` | How is this entity participating right now? | No — session/attachment scoped | Many per `Person` |

A `Person` does not require an active `Presence` to exist.  A `Presence` always
references a `Person` (or future generic entity) as its `subject`.

---

## 3. How Presence Differs from Place

`Place` is the canonical *world-context* for a running execution unit.  It
answers: *in what filesystem/namespace context does this execution occur?*  It
carries `cwd`, `namespace`, and `root`.

`Presence` is the *person-in-place relationship*.  It answers: *how is this
entity attached to, or participating within, a given place?*  It is not
execution state — it is interaction state.

| Concept | Answers | Owner |
|---|---|---|
| `Place` | In what world does this execution occur? | Owned by / associated with a `Job`/`Task` |
| `Presence` | How is a `Person` participating in a place? | Owned by the session or attachment mechanism |

A `Place` exists even when no `Person` is present (daemon processes, background
jobs).  A `Presence` always relates a subject to a place or context; without a
subject it has no meaning.

`Presence` may reference a `Place` but does not own it and does not require
Place runtime semantics to be complete.

---

## 4. How Presence Relates to Authority

`Authority` carries *what the entity is permitted to do* — credentials,
capability context, and permission scope.

`Presence` describes *how the entity is situated*, not what it may do.
Being present at a terminal does not, by itself, grant capabilities.  Authority
is a separate axis.

However, Presence and Authority interact in the projection layer:

- **Session credentials** — in Unix, the controlling-terminal process acquires
  the ability to send `SIGHUP` to the session.  In the typed-world model that
  interaction is an explicit `Authority` check, informed by `Presence`
  (which process group is foregrounded).  The authority check is the canonical
  gate; Presence provides the context.
- **Foreground privilege** — in Unix, only the foreground process group may read
  from the controlling tty.  Modelled canonically: `Group` (foreground kind)
  determines which `Task`s are eligible; `Presence` names the attachment
  relationship; `Authority` enforces the read permission.
- **Login session context** — PAM/credential establishment at login creates both
  an `Authority` (the credential context) and a `Presence` (the terminal
  attachment relationship).  They are distinct records with distinct lifetimes.

> **Guardrail**: `Presence` must not carry authority semantics directly.  "Active
> presence" is a mode flag, not a capability grant.  Authority decisions that
> use Presence as *input context* belong in the `Authority` layer, not in the
> Presence schema itself.

---

## 5. Unix Semantics Projected from Presence

The following table maps each Unix session / tty / job-control concept to its
typed-world owner.  Current kernel location (quarantined in
`ProcessUnixCompat`) is shown alongside the intended canonical home.

| Unix concept | Typed-world owner | Current bridge / location | Migration path |
|---|---|---|---|
| **session ID (SID)** | `Group` (session kind) + `Presence` (terminal attachment) | `Process.unix_compat.sid`; quarantined | Extract to `Group` membership; `Presence` names the attachment |
| **session leader** | `Presence` (role flag on the attachment record) | `Process.unix_compat.session_leader`; quarantined | Encode as a `Presence` role on the session-leader attachment |
| **process group ID (PGID)** | `Group` (foreground/background kind) | `Process.unix_compat.pgid`; bridged via `kernel::group::bridge` | Already partially migrated; complete once `Group` is canonical |
| **foreground process group** | `Group` membership + `Presence` (controlling-terminal attachment) | `devfs::ConsoleTtyState::controlling_sid`; quarantined | `Presence` names the tty attachment; `Group` foreground flag gates tty I/O |
| **background process group** | `Group` (background kind) | Same quarantine as above | Same migration path |
| **controlling TTY** | `Presence` (terminal attachment between a `Person` and a `Place`) | `devfs::ConsoleTtyState::controlling_sid`; quarantined | `SYS_TTY_ATTACH` → creates a `Presence` record linking subject to tty `Place` |
| **interactive shell attachment** | `Presence` (`embodiment: Direct`, `mode: Active`) | Implicit in session state; no canonical record | First explicit use of a `Presence` record at session start |
| **SIGHUP on session leader exit** | `Group` (signal fanout) + `Presence` (which sessions are affected) | `ProcessSignals`; quarantined | Route through `Group` membership check informed by `Presence` records |
| **SIGTTIN / SIGTTOU (background I/O stop)** | `Group` (foreground check) | `ProcessSignals`; quarantined | Route through `Group` foreground membership; `Presence` informs which group owns the tty |

### 5.1 — Session Leader

#### Unix surface

`setsid()` creates a new session, making the calling process the session leader
with a new SID equal to its PID.  The session leader is the only member of the
new session initially.  On exit, the session leader's terminal sends `SIGHUP` to
the foreground process group.

#### Typed-world interpretation

A **session** is a `Group` of the session kind.  The **session leader** is the
attachment point: the entity whose `Presence` record was first in that `Group`,
with a role flag indicating leadership.

```
Unix setsid()
    │
    ├──► creates Group (session kind)        [kernel::group::bridge]
    └──► creates Presence {                  [future: kernel::presence::bridge]
             subject: calling Person/Task,
             group:   new session Group,
             mode:    Active,
             embodiment: Direct,
             role:    SessionLeader,         ← role flag (future field)
         }
```

The session-leader role is a property of the `Presence` record, not a flag on
`Process` or `Group`.

#### Migration note

`Process.unix_compat.session_leader` is the current quarantined backing.  Once
`Presence` is introduced as a live runtime concept, `setsid()` will be
implemented by creating both a `Group` record and a `Presence` record, and the
`session_leader` field will be removed from `ProcessUnixCompat`.

---

### 5.2 — Controlling TTY

#### Unix surface

When a session leader opens a terminal device that is not already a controlling
terminal of another session, the kernel automatically assigns that terminal as
the controlling terminal of the session.  A session has at most one controlling
terminal at any time.

#### Typed-world interpretation

The **controlling terminal** is a `Presence` record that describes the
attachment between the session's principal `Person` (or the session `Group`)
and the terminal `Place` (the VFS node of the terminal device).

```
Unix open("/dev/pts/0", O_RDWR)  →  controlling-tty assignment
    │
    └──► SYS_TTY_ATTACH (future syscall) creates Presence {
             subject:    session Group (or Person of session leader),
             place:      Place representing /dev/pts/0,
             group:      session Group,
             mode:       Active,
             embodiment: Direct,
         }
```

The `Place` node for the terminal (`/dev/pts/0`, `/dev/tty`, etc.) is
filesystem-visible.  The `Presence` record lives in a session registry
(future: `/session/` or `/run/sessions/`), not inside the tty device state.

#### Current location

`devfs::ConsoleTtyState::controlling_sid` currently holds the controlling SID
as a raw integer inside the device state.  This field is quarantined.  Future
work replaces it with a `Presence` lookup: "which `Presence` record names this
tty `Place` as its attachment point?"

---

### 5.3 — Foreground and Background Process Groups

#### Unix surface

`tcsetpgrp(fd, pgid)` sets the foreground process group of the terminal
identified by `fd`.  Only tasks in the foreground process group may read from
the terminal without generating `SIGTTIN`.

#### Typed-world interpretation

The **foreground/background distinction** is a property of `Group` membership
relative to a `Presence` (the tty attachment).

- The **foreground group** is the `Group` (foreground kind) associated with the
  active `Presence` on the terminal.
- **Background groups** are all other `Group`s in the same session.
- `SIGTTIN` / `SIGTTOU` are routed by checking whether the calling `Task`'s
  `Group` membership matches the foreground group recorded on the `Presence`.

```
tcsetpgrp(fd, pgid)
    │
    └──► mutates Presence.foreground_group   ← future field on Presence
             (points to Group of given pgid)
```

> **Note**: In the current model, foreground state is implicit in
> `devfs::ConsoleTtyState` and `ProcessUnixCompat.pgid`.  The canonical model
> moves it to a `Presence` field or a `Group` attribute on the session's
> `Presence` record.

---

### 5.4 — Interactive Shell Attachment

#### Unix surface

An interactive shell is a process whose stdin, stdout, and stderr are connected
to a terminal and that enables job control.  There is no explicit syscall to
"be interactive"; the shell detects ttyness via `isatty()` and calls `setpgid()`
and `tcsetpgrp()` to manage job control.

#### Typed-world interpretation

An **interactive shell** is a `Task` whose session has a `Presence` record
with:

```
Presence {
    subject:    Person (logged-in user) or Task (shell process),
    place:      Place of the terminal (/dev/pts/N),
    group:      session Group,
    mode:       Active,
    embodiment: Direct,
}
```

The "interactive" quality is captured by `embodiment: Direct` and `mode:
Active`.  Non-interactive execution (batch jobs, daemons) either has no
`Presence` record or has `embodiment: Deferred` / `mode: Latent`.

---

### 5.5 — Future GUI and Terminal Embodiment

#### Problem

A GUI session or a terminal emulator window is not adequately modelled by the
Unix controlling-tty concept.  A user may have:

- A Wayland/X compositor session (embodiment through display server).
- Multiple terminal windows (multiple `Presence` records, same `Person`).
- A remote desktop session (embodiment through a proxied display channel).
- An embedded terminal inside a GUI application.

#### Typed-world projection

Each of these is a distinct `Presence` record with a different `embodiment`
and `place`:

| Attachment type | `embodiment` | `place` | `mode` |
|---|---|---|---|
| Physical terminal | `Direct` | `/dev/tty0` | `Active` |
| Pseudo-terminal (ssh, xterm) | `Remote` | `/dev/pts/N` | `Active` or `Remote` |
| GUI compositor session | `Direct` (future: `Graphical`) | `/services/compositor` or display Place | `Active` |
| Remote desktop | `Remote` | `/services/rdp-session/N` or similar | `Remote` |
| Background/daemon | *(no Presence, or `Deferred`)* | — | `Latent` or `Deferred` |

The `EmbodimentKind` enum will be extended as the device/avatar/body model
matures.  `Direct` and `Remote` are the initial anchors; `Graphical` and other
kinds are future work.

> **Guardrail**: Do not collapse all embodiment stories into "has terminal / no
> terminal".  The `embodiment` field is deliberately extensible.

---

## 6. Unresolved Design Questions

The following questions are explicitly open.  They must not be answered
unilaterally in implementation without updating this document and a design
review.

1. **Who creates and destroys `Presence` records?**
   A syscall (`SYS_TTY_ATTACH`), a session daemon, or the kernel login path?
   The lifetime management mechanism is not yet defined.

2. **Where are `Presence` records stored?**
   Options: `/session/` VFS subtree, `/run/sessions/`, a kernel registry, or a
   userspace session daemon.  VFS-first architecture suggests a mounted path;
   the exact mount point is undecided.

3. **What is the `Presence` ↔ `Group` cardinality?**
   Can one `Presence` span multiple `Group`s?  Can one `Group` have multiple
   `Presence` records?  The current schema allows both (`group` is optional and
   singular), but multi-group presence is a plausible future need.

4. **Does session leadership move?**
   Unix session leaders cannot change.  In the Presence model, leadership is a
   role flag.  Can that flag transfer (e.g. on session-leader exit)?  If so,
   who transfers it?

5. **How is Presence invalidated / expired?**
   A `Presence` record may become stale.  The `observed_at` field provides a
   hint but not a guarantee.  A proper liveness protocol has not been designed.

6. **What is the canonical SIGHUP projection?**
   SIGHUP is currently generated by the kernel on session-leader exit.  In the
   canonical model it should be a `Group`-level event routed through the
   `Message` system.  The exact routing path — which `Presence` record triggers
   it, which `Group` members receive it — is undecided.

7. **Can a daemon process have a Presence?**
   Daemons detach from the controlling terminal (via `setsid()`, `/dev/null`
   redirects).  Should they have a `Presence` with `mode: Latent` / `embodiment:
   Deferred`, or no `Presence` at all?  The current schema supports both; a
   policy decision is needed.

8. **Is `Presence` per-Person or per-Task?**
   The current `subject` field is `ref<thingos.person>`.  Some Unix session
   semantics attach to processes (Tasks / Jobs), not to people.  A future phase
   may generalise `subject` to any entity.  Until then, the mapping for
   `Task`-scoped attachment (e.g. a daemon that owns a tty) is unclear.

---

## 7. Non-Goals / Guardrails

- **Do not** let `Presence` carry authority semantics.  "Active" is a mode, not
  a capability.
- **Do not** turn `Presence` into a scheduler primitive or execution residency
  concept.
- **Do not** couple `Presence` creation to `Group` or `Place` runtime
  readiness; the schema must remain valid with `place: None, group: None`.
- **Do not** add a "one presence per subject" constraint at this layer.
- **Do not** collapse `EmbodimentKind` to a boolean "has tty / no tty".
- **Do not** implement `Presence` as a stealth integration layer that silently
  drives routing, scheduling, or authority before `Place` and `Group` are stable.

---

## 8. Migration Inventory

| `ProcessUnixCompat` field | Presence role | Migration target | Status |
|---|---|---|---|
| `session_leader: bool` | Session-leader role flag | `Presence.role: SessionLeader` (future field) | Quarantined |
| `sid: u32` | Session Group ID | `Presence.group → Group (session kind)` | Quarantined |
| `pgid: u32` | Process group (foreground/background) | `Group` (foreground kind) — already partially bridged | Partially bridged |
| `devfs::ConsoleTtyState::controlling_sid` | Controlling-tty attachment | `Presence { subject, place: tty-Place, group: session-Group }` | Quarantined |

---

## Related Documents

- `docs/architecture/concept-classification.md` — Presence is **Canonical / Emerging**
- `docs/architecture/unix-projection.md` — canonical mapping table; Presence appears in session/tty rows
- `docs/concepts/presence.md` — schema overview, deferred integration rationale, field documentation
- `docs/migration/concept-mapping.md` — "TTY / controlling terminal → Presence" entry
- `docs/migration/bridge_architecture.md` — bridge conventions applicable to a future presence bridge
- `docs/concepts/unix-compat.md` — Unix session/env quarantine boundary (`ProcessUnixCompat`)
- `docs/kernel/signals.md` — signal subsystem; job-control signals will route through Presence context
- `thingos/src/presence.rs` — canonical Rust schema types
- `kernel/src/task/mod.rs` — `ProcessUnixCompat` quarantine fields (`sid`, `session_leader`, `pgid`)
- `kernel/src/group/bridge.rs` — Group bridge (partially covers session/pgid)

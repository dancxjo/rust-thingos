# Unix Session and Environment Compatibility Quarantine

> **Status**: Active quarantine boundary — Phase 9 baseline.
> This document explains why Unix session/env concepts are isolated, which
> behaviors still depend on them, and how they relate to the future
> `Place / Presence / Group` work.
>
> Companion documents:
> - `docs/migration/process_execution_context_inventory.md` — full execution-context field map
> - `docs/migration/process_responsibility_map.md` — canonical Process field decomposition
> - `docs/migration/authority_inventory.md` — credential/permission inventory
> - `kernel/src/task/mod.rs` — `ProcessUnixCompat` struct (the primary quarantine container)

---

## Purpose

ThingOS is migrating away from the Unix model of execution context.  A Unix
process silently accumulates session state, environment variables, controlling
TTY relationships, and process-group membership as ambient inherited context.
ThingOS distributes these concerns explicitly across **Place**, **Presence**,
and **Group** — but these concepts are not yet fully introduced.

In the interim, Unix-derived state must continue to work correctly for
compatibility.  This document explains:

1. **What is quarantined** — which concepts are behind the explicit boundary.
2. **Why they are quarantined** — the architectural rationale.
3. **Which behaviors depend on them** — what would break if they were removed.
4. **How they stay isolated** — the structural boundary in code.
5. **Where they are going** — future intended ownership.

---

## 1. What is quarantined

All Unix-derived compatibility state lives in one place:

```
Process.unix_compat   (type: ProcessUnixCompat)
```

This struct is defined in `kernel/src/task/mod.rs`.  Its fields are:

| Field            | Unix concept                   | Current role                                     |
|------------------|--------------------------------|--------------------------------------------------|
| `signals`        | Per-process signal state       | Dispositions, pending set, stop/alarm state      |
| `message_inbox`  | Process inbox                  | Typed message queue (prototype)                  |
| `pgid`           | Process group ID               | Group signal routing, job-control, `kill(0/neg)` |
| `sid`            | Session ID                     | TTY attachment, job-control, `setsid`            |
| `session_leader` | Session-leader flag            | TTY foreground ownership proxy (heuristic)       |
| `argv`           | Argument vector                | Spawn-time arguments read by `/proc`             |
| `env`            | Environment variable map       | Inherited KEY=VALUE environment blob             |
| `auxv`           | ELF auxiliary vector           | AT\_\* entries for dynamic linking               |

In addition, TTY/console device state that is *not* on `Process` but is
tightly coupled to the quarantined concepts:

| Location                        | Field                | Unix concept                 |
|---------------------------------|----------------------|------------------------------|
| `devfs::ConsoleTtyState`        | `controlling_sid`    | Which session owns `/dev/console` |
| `devfs::ConsoleTtyState`        | `foreground_pgid`    | Which pgid holds TTY foreground |
| `devfs::ConsoleCaller`          | (transient)          | Caller's pgid/sid/session_leader used for job-control |

---

## 2. Why they are quarantined

### These concepts are not architectural truth in ThingOS

Unix session semantics encode a 1970s login model: a terminal session
identifies a person, a controlling TTY anchors the session, and shell
pipelines are coordinated through process groups.  ThingOS does not treat any
of these relationships as intrinsic.

The long-term model uses:

- **Place** — to answer "in what world does execution happen?" (cwd,
  namespace, visibility boundary).
- **Presence** — to describe person-in-place embodiment and active terminal
  participation.
- **Group** — as an explicit coordination domain replacing ad-hoc pgid/session
  membership.

Unix session/env concepts are **provisionally retained** only because their
functional replacements have not yet been introduced.

### Preventing conceptual spread

Without an explicit boundary, Unix session concepts tend to silently diffuse
through kernel code as "the natural way" to handle execution context.  The
quarantine makes every use of legacy state visible at code review: accessing
`unix_compat.pgid` is a clear signal that a call site is using transitional
compatibility state, not a first-class architectural concept.

---

## 3. Which behaviors currently depend on quarantined state

The following behaviors rely on quarantined state and would break if it were
removed without a replacement:

| Behavior | Quarantined fields | Syscall / path |
|---|---|---|
| `kill(0, sig)` — signal own process group | `unix_compat.pgid` | `sys_kill` |
| `kill(-pgid, sig)` — signal a process group | `unix_compat.pgid` | `sys_kill` |
| `setpgid(pid, pgid)` — set process group | `unix_compat.pgid`, `unix_compat.sid` | `sys_setpgid` |
| `setsid()` — create new session | `unix_compat.sid`, `unix_compat.pgid`, `unix_compat.session_leader` | `sys_setsid` |
| `getpgrp()` — get process group | `unix_compat.pgid` | `sys_getpgrp` |
| Job-control SIGTTIN/SIGTTOU | `unix_compat.pgid`, `unix_compat.sid`, `unix_compat.session_leader` | devfs TTY read/write |
| TTY controlling session acquisition | `unix_compat.sid`, `unix_compat.session_leader` | `ConsoleNode::maybe_acquire_controlling_tty` |
| `TIOCSPGRP`/`TIOCGPGRP` | `unix_compat.pgid` | devfs ioctl |
| procfs `/proc/<pid>/status` | `pgid`, `sid`, `session_leader` in `ProcessSnapshot` | procfs |
| `sys_env_get/set/list` | `unix_compat.env` | process syscall handlers |
| `sys_argv_get` | `unix_compat.argv` | process syscall handlers |
| `sys_auxv_get` | `unix_compat.auxv` | process syscall handlers |
| Signal dispositions/masks | `unix_compat.signals` | `sys_sigaction`, `sys_sigprocmask`, `sys_sigpending` |
| `fork`/`exec` env inheritance | `unix_compat.env` | `ProcessUnixCompat::inherit` |
| `fork`/`exec` pgid/sid inheritance | `unix_compat.pgid`, `unix_compat.sid` | `ProcessUnixCompat::inherit` |

---

## 4. How the isolation is enforced in code

### Structural boundary

Unix-derived state **must not appear** as top-level fields in `Process`.  All
such state must live inside `Process.unix_compat` (type `ProcessUnixCompat`).

```
Process {
    pid,
    lifecycle,            // ← Job domain
    space,                // ← Space domain
    fd_table,             // ← Resource table
    cwd, namespace,       // ← Place domain
    unix_compat {         // ← QUARANTINE BOUNDARY
        signals,
        message_inbox,
        pgid, sid, session_leader,
        argv, env, auxv,
    },
}
```

Any access to legacy session/env state is immediately visible as
`process.unix_compat.FIELD`, making every such access identifiable in code
review.

### Rules for new code

1. **Do not add fields to `ProcessUnixCompat`** unless forced by a concrete
   Unix compatibility requirement.  All new state should use Task / Job /
   Group / Place / Authority vocabulary.
2. **Do not add Unix session/env fields to top-level `Process`** directly.
   Rejected in code review.
3. **Do not read `pgid`, `sid`, or `session_leader` from `ProcessSnapshot`
   directly** in new coordination or visibility code; use
   `kernel::group::bridge::group_kind_from_snapshot` instead.
4. **Do not deepen job-control signal entanglement** (SIGTTOU/SIGTTIN/SIGTSTP).
   Any unavoidable compatibility code must remain inside
   `unix_compat.signals`.
5. **Comments in quarantined code** must explain the future migration target
   (see `ProcessUnixCompat` doc comment for the full table).

### Bridge modules

Quarantined state surfaces to external consumers **only through bridge
modules**:

| Bridge | Purpose | Quarantined source |
|--------|---------|-------------------|
| `kernel::group::bridge` | Process group / session → `Group` | `pgid`, `sid`, `session_leader` |
| `kernel::place::bridge` | cwd / namespace → `Place` | `cwd`, `namespace` |
| `kernel::authority::bridge` | exec path → `Authority::name` | `exec_path` |
| `kernel::job::bridge` | lifecycle → `Job` | `lifecycle.*` |
| `kernel::spawn::bridge` | typed immutable spawn context | `spawn_record.(argv,auxv)` |

New public-facing context code must go through a bridge module, not read
`unix_compat.*` directly.

---

## 5. Where quarantined concepts are going (future homes)

| Quarantined concept | Future ThingOS home | Phase / prerequisite |
|---|---|---|
| `pgid` — process group | `Group` first-class object | Phase 5 |
| `sid` — session ID | `Group / Presence / Place` | Phase 5 (after Presence) |
| `session_leader` — TTY foreground flag | `Group / Presence` | Phase 5 |
| SIGTTOU/SIGTTIN job-control | `Group / Presence` | After Group + Presence |
| `env` — environment blob | `Place` or `Authority` context | After Place-facing env design |
| `spawn_record.argv` — argument vector | Structured spawn record → `Job` | `SpawnRecord` introduced (Phase 9) |
| `spawn_record.auxv` — ELF auxiliary vector | Structured spawn record → `Job` | `SpawnRecord` introduced (Phase 9) |
| Signal dispositions/pending | `Message / Inbox / Group broadcast` | After signal authority work |
| `controlling_sid` | `Presence` (terminal attachment) | After Presence |
| `foreground_pgid` | `Group` | Phase 5 |
| Implicit TTY acquisition | Explicit `SYS_TTY_ATTACH` → `Presence` | After Presence |

### Place

`Place` owns the world-context boundary: cwd, VFS namespace view, and
filesystem root.  The environment blob (`env`) maps naturally here because
`PATH`, `HOME`, and locale variables directly affect what world the process
sees.  However, a Place-facing environment design does not exist yet; the env
blob remains quarantined until that design is done.

### Presence

`Presence` will describe person-in-place embodiment — the active terminal
connection, UI participation, and console ownership.  The controlling-TTY
relationship (`controlling_sid`), TTY acquisition, and job-control stop signals
all belong to Presence once it exists.

### Group

`Group` will be the explicit coordination domain.  Process groups (`pgid`),
sessions (`sid`), `session_leader`, `foreground_pgid`, and group-directed
signals will all migrate here.  `kernel::group::bridge` already bridges the
current Unix-shaped state into a canonical `Group` vocabulary (Phase 4).

---

## 6. Constraints

- **Do not remove** quarantined behavior in this issue or any follow-on that
  does not explicitly scope a replacement.
- **Do not redesign** terminals, shells, or signal semantics here.
- **Do not conflate Place with Presence**: world-context (cwd, namespace) is
  Place; terminal attachment and person-in-place embodiment is Presence.
- **Do not deepen** Unix session/env assumptions in the core model.

---

## Related documents

- `docs/migration/process_execution_context_inventory.md` — execution-context field inventory with migration status
- `docs/migration/process_responsibility_map.md` — canonical Process field decomposition
- `docs/migration/authority_inventory.md` — credential/permission inventory
- `docs/migration/boundary_audit.md` — bridge compliance audit
- `docs/concepts/thingos-guardrails.md` — architecture guardrails for all kernel changes
- `docs/concepts/process-object.md` — Process / Thread struct design
- `docs/concepts/namespaces.md` — namespace behaviour matrix and roadmap
- `kernel/src/task/mod.rs` — `ProcessUnixCompat` struct and quarantine commentary
- `kernel/src/group/mod.rs` and `bridge.rs` — Group bridge (Phase 4)
- `kernel/src/place/bridge.rs` — Place bridge (Phase 8)
- `kernel/src/signal/mod.rs` — signal subsystem (quarantined unix compat)
- `kernel/src/vfs/devfs.rs` — `ConsoleTtyState`, `ConsoleCaller` (quarantined TTY state)

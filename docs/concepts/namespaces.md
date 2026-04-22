# Namespace Semantics and Roadmap

> **Status:** Mount namespaces are now identity-scoped in the kernel mount
> layer. The global namespace remains the default, while isolated namespace
> identities get copy-on-first-use mount tables. This document defines what is
> and is not isolated today, and the remaining roadmap gaps.

---

## 1. Behaviour Matrix — What Is Global vs Isolated Today

| Resource | Status | Owner | Notes |
|---|---|---|---|
| **File descriptor table** | ✅ Isolated | `Process.fd_table` | Per-process, inherited-by-clone at spawn |
| **Current working directory** | ✅ Isolated | `Process.cwd` | Per-process, inherited at spawn/exec |
| **Environment variables** | ✅ Isolated | `Process.env` | Per-process, inherited at spawn |
| **Virtual memory mappings** | ✅ Isolated | `Process.mappings` | Per-process address space |
| **Mount namespace** | ✅ Identity-scoped | `NamespaceRef` + `vfs::mount` namespace tables | Global by default; isolated IDs get per-namespace mount tables |
| **`SYS_FS_MOUNT` effect** | ✅ Namespace-local | `vfs::mount::mount()` | Mounts only affect the caller's namespace table |
| **`SYS_FS_UMOUNT` effect** | ✅ Namespace-local | `vfs::mount::umount()` | Unmounts only affect the caller's namespace table |
| **PID namespace** | ⚠️ Global stub | N/A | Single flat PID space, no nesting |
| **Network namespace** | ⚠️ Not implemented | N/A | No networking stack yet; will be per-namespace when added |
| **IPC namespace** | ⚠️ Global stub | `GLOBAL_HANDLE_TABLE` | Handle table is currently process-local; port IDs are global |

### Key terms

* **Global default** — processes start in namespace `global` unless explicitly
  assigned a different `NamespaceRef`.

* **Isolated** — each process owns an independent copy; changes in one process
  do not propagate to others.

---

## 2. `NamespaceRef` API Contract

`NamespaceRef` (defined in `kernel/src/vfs/mod.rs`) is the kernel type that
represents a process's view of the VFS mount table.

### Current contract

`NamespaceRef` carries a stable namespace id. `vfs::mount` uses that identity
to select a namespace-local mount table for mount/umount/lookup operations.

* Every call to `NamespaceRef::global()` returns namespace id `1`.
* `NamespaceRef::isolated()` allocates a new namespace id.
* A namespace table is initialized from the global table on first use, then
  diverges independently.
* `Process.namespace` is cloned at spawn time via `inherit_process_info`.

### Behaviour guarantees (valid now and after real isolation lands)

| Guarantee | Rationale |
|---|---|
| Path resolution through `NamespaceRef` is deterministic | Required for correctness |
| `SYS_FS_OPEN` uses the calling process's `namespace` field | Plumbing is in place; no call-site change needed when per-process namespaces are added |
| A process cannot observe a mount it has explicitly unmounted | `umount` removes the entry from whichever table the namespace resolves to |
| `SYS_FS_MOUNT`/`SYS_FS_UMOUNT` in global namespace require mount privilege | Global namespace changes remain privileged |
| `SYS_FS_MOUNT`/`SYS_FS_UMOUNT` in isolated namespace do not require global mount capability | Namespace owners can mutate their own isolated table |

### Non-guarantees (explicitly not promised today)

* **Snapshot-on-spawn**: spawning a child does not give it a private copy of
  the mount table; both parent and child see subsequent mounts.

---

## 3. Staged Implementation Roadmap

### Milestone NS-1 — Owned mount table (ACT V prerequisite)

**Goal:** `NamespaceRef` contains an `Arc<MountTable>` instead of being a unit
struct.  All processes still share the same `Arc`, so observable behaviour is
unchanged.

**Changes required:**

* Add `MountTable` struct wrapping the current global mount list in
  `kernel/src/vfs/mount.rs`.
* Change `NamespaceRef` to `pub struct NamespaceRef(Arc<MountTable>)`.
* Replace all `NamespaceRef::global()` call sites with a constructor that
  returns the shared singleton.
* VFS resolution helpers (`vfs::path::resolve`) accept `&NamespaceRef` and
  delegate to `namespace.0`.

**Acceptance criteria:**

- [ ] All existing tests pass unchanged.
- [ ] `NamespaceRef::global()` returns a value backed by a real mount table.
- [ ] No change to externally visible syscall behaviour.

---

### Milestone NS-2 — Copy-on-write namespace at spawn

**Goal:** `SYS_SPAWN_PROCESS_EX` can optionally give the child its own private
mount table, initialized as a shallow copy of the parent's.

**Changes required:**

* Add `SpawnFlags::CLONE_NEWNS` (or equivalent field in
  `abi::types::SpawnProcessExReq`) so callers can opt into a private
  namespace.
* In `sched::spawn::inherit_process_info`, branch on the flag: share the
  parent's `Arc<MountTable>` (existing behaviour) or `Arc::new(parent.clone())`
  for a private copy.
* `SYS_FS_MOUNT` / `SYS_FS_UMOUNT` modify `process.namespace.0` (the
  process-local mount table) rather than the global singleton.

**Acceptance criteria:**

- [ ] A process spawned without `CLONE_NEWNS` behaves identically to today.
- [ ] A process spawned with `CLONE_NEWNS` has its own mount table; mounts
  do not propagate back to the parent.
- [ ] Documentation and a unit test cover both cases.

---

### Milestone NS-3 — Namespace ownership model hardening

**Goal:** Tighten namespace ownership semantics beyond current global-vs-isolated
policy and provide explicit ownership metadata.

**Changes required:**

* Add explicit namespace ownership metadata (principal/process ownership).
* Enforce ownership checks for cross-namespace operations if such APIs are added.

**Acceptance criteria:**

- [x] An unprivileged process receives `EPERM` when trying to mount into the
  global namespace.
- [x] A process with a private namespace may freely mount within it.

---

### Milestone NS-4 — PID and IPC namespaces

**Goal:** Nested PID spaces (containers) and scoped IPC handle tables.

**Notes:**

* PID namespaces require a mapping layer in `sys_get_tid`, `sys_waitpid`, and
  `/proc`.
* IPC namespace scoping means `GLOBAL_HANDLE_TABLE` becomes per-namespace.
* This milestone is intentionally deferred until NS-2 and NS-3 are stable.

---

## 4. How to Detect Isolated Namespace Context in Code

Callers that need to branch on global vs isolated namespace behavior can use a
runtime check:

```rust
// kernel/src/vfs/mod.rs
impl NamespaceRef {
    /// Returns `true` when this is not the global namespace identity.
    pub fn is_isolated(&self) -> bool {
        self.id() != 1
    }
}
```

Callers can use `process.namespace.is_isolated()` to apply namespace-specific
policy (for example privilege gates that only apply to global namespace
mutations).

---

## 5. Related Issues and Documents

* **This issue** — Clarify namespace semantics and roadmap
  (stub vs implemented isolation)
* `docs/concepts/process-object.md` — Process / Thread ownership model
* `docs/concepts/thingos-guardrails.md` — Architectural guardrails (VFS-first,
  spawn+exec model)
* `kernel/src/vfs/mod.rs` — `NamespaceRef` struct and global mount init
* `kernel/src/vfs/mount.rs` — Namespace-scoped mount table implementation
* `kernel/src/sched/spawn.rs` — `inherit_process_info` (namespace inheritance
  at spawn time)

Follow-up implementation work should be tracked in separate issues linked back
to this document.

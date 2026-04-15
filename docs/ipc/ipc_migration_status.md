# IPC Migration Status — Thing-OS

This document is the single entry point for understanding the IPC model in
Thing-OS: which primitives exist, what the recommended patterns are, which
APIs are deprecated, and what migration work remains.

Cross-reference: issue #46 (Inbox/Port convergence), issue #107 (this audit).

---

## 1. IPC Substrate Overview

Thing-OS IPC is built on a **plural substrate model** — multiple primitives,
each with a clear job, unified under a common VFS-first readiness model.

| Primitive | Syscall family | Semantic role | Pollable via `SYS_FS_POLL`? |
|-----------|---------------|---------------|--------------------------|
| **Channel** | `SYS_CHANNEL_*` | Endpoint-connected message/stream transport | ✅ via `SYS_FD_FROM_HANDLE` |
| **Pipe** | `SYS_PIPE` / `SYS_FS_*` | One-way byte stream | ✅ natively |
| **Unix socket** | `SYS_SOCKET` / `SYS_BIND` / etc. | Bidirectional byte-stream endpoint | ✅ natively |
| **Inbox** | internal | Receiver-owned typed delivery queue | ✅ via `InboxNode` |
| **Memfd** | `SYS_MEMFD_CREATE` | Bulk data / zero-copy shared buffer | ✅ natively |
| **Futex** | `SYS_FUTEX_*` | In-process sync primitive (not IPC) | ✗ (use within address space only) |

**Readiness rule**: all IPC resources that need multiplexing participate in
`SYS_FS_POLL`.  There is no separate per-primitive wait syscall for new code.

---

## 2. Terminology Glossary

| Term | Meaning |
|------|---------|
| **Channel** | User-visible name for the bounded message/stream endpoint. Backed internally by a `Port` ring. |
| **Port** | Kernel-internal implementation term for the channel ring and wait queues. Not a user-facing API name. |
| **Inbox** | Receiver-owned arrival queue, distinct from channel. Ownership-first semantics, no required peer endpoint. |
| **Thing** | A kernel-managed capability reference (analogous to POSIX "file descriptor" or Win32 "HANDLE"). |
| **FD** | A VFS-facing integer thing number used with `SYS_FS_*` calls. Channels bridge to FDs via `SYS_FD_FROM_HANDLE`. |
| **VFS-first** | Design principle: all waitable resources participate in the VFS poll/waiter contract. |

---

## 3. Recommended Patterns

### 3.1 Readiness multiplexing

Use `SYS_FS_POLL` for all readiness needs.  Every IPC object is reachable
through it:

- Pipes and sockets — natively VFS-backed; pass their thing number directly.
- Channel ends — bridge via `SYS_FD_FROM_HANDLE` first, then poll the
  resulting VFS thing.
- Inbox queues — wrap in `InboxNode`; poll the node FD.
- Mixed sources — combine all of the above in one `SYS_FS_POLL` call.

For heterogeneous waits that mix FDs with task-exit, IRQs, or timeouts, use
`SYS_WAIT_MANY` with `WaitKind::Fd` for all FD-backed sources.

### 3.2 Message passing

- **Discrete messages** (commands, events, RPC, capability passing): use a
  channel (`SYS_CHANNEL_SEND_ALL`, `SYS_CHANNEL_SEND_MSG`).
- **Byte streams** (stdio, pipelines): use a pipe or Unix socket.
- **Bulk data** (pixel buffers, audio rings): use a memfd; pass the fd over a
  channel.
- **Typed delivery to a process/job** (lifecycle events, process messages):
  use an inbox.

### 3.3 Capability passing

Use `SYS_CHANNEL_SEND_MSG` / `SYS_CHANNEL_RECV_MSG` to pass things
atomically alongside data.  The old `SYS_CHANNEL_SEND_HANDLE` single-thing
API is deprecated; use the msg variants instead.

---

## 4. Deprecation Table

| API | Status | Migration path |
|-----|--------|---------------|
| `SYS_CHANNEL_WAIT` | **Deprecated** | Bridge with `SYS_FD_FROM_HANDLE`, then use `SYS_FS_POLL` |
| `SYS_CHANNEL_SEND_HANDLE` | **Deprecated** | Use `SYS_CHANNEL_SEND_MSG` with handles array |
| `WaitKind::GraphOp` (= 6) | **Deprecated**, returns `ENOSYS` | No replacement; graph-op watches are removed |
| `WaitKind::RootWatch` (= 2) | **Deprecated**, returns `ENOSYS` | Use `WaitKind::Fd` with a VFS-backed watch node |
| Raw port handles in driver boot args | **Legacy pattern** | Use `channel_create` + `SYS_FD_FROM_HANDLE`; prefer `spawn_process_ex` FD inheritance |
| `add_port_readable` in `WaitSet` | **Deprecated** | Use `add_fd_readable` with a bridged FD |

---

## 5. Inbox / Port Convergence Status (Issue #46)

The Inbox and Port/Channel primitives are **distinct** abstractions that share
a common readiness model.  They are not collapsed into one — see
`docs/ipc/inbox_vs_port_semantics.md` for the full semantic analysis.

**Layered convergence strategy** (Option C) was chosen:
- Shared internal queue mechanics (backpressure, wakeups, diagnostics).
- Two distinct public views: `PortView` (connection transport) and
  `InboxView` (ownership arrival queue).
- Unified readiness via `SYS_FS_POLL`.

### Roadmap status

| Phase | Step | Status |
|-------|------|--------|
| A — Spec lock | Semantics doc (`inbox_vs_port_semantics.md`) | ✅ Done |
| A — Spec lock | Strategy doc (`convergence_strategy.md`) | ✅ Done |
| B — Shared core | Extract shared internal queue trait | ⬜ Pending |
| B — Shared core | Migrate backpressure/wakeup/metrics to core | ⬜ Pending |
| **C — Readiness** | **`InboxNode` VFS wrapper (poll + waiter hooks)** | **✅ Done** — `kernel/src/vfs/inbox_node.rs` |
| C — Readiness | Expose inbox FD acquisition path to userspace | ⬜ Pending |
| C — Readiness | Tests: mixed poll sets (file + channel + inbox FDs) | ⬜ Pending |
| D — Deprecation | Document migration off `SYS_CHANNEL_WAIT` | ✅ Done (this document + `channel_semantics.md`) |
| D — Deprecation | Userspace `channel_wait` → `fd_from_handle + fs_poll` | ⬜ Pending |

---

## 6. Driver / Service Migration Status

| Component | Legacy pattern | Migration status |
|-----------|---------------|-----------------|
| `ps2_mouse` driver | `channel_send_all` (raw port send) | ✅ Migrated to `vfs_write` on channel FD |
| `ps2_kbd` driver | `channel_send_all` (raw port send) | ✅ Migrated to `vfs_write` on channel FD |
| `bristle` (mouse input) | `add_port_readable` + `channel_recv` | ✅ Migrated to `add_fd_readable` + `vfs_read` |
| `bristle` (kbd input) | `add_port_readable` + `channel_recv` | ✅ Migrated to `add_fd_readable` + `vfs_read` |
| `bristle` (output path) | `channel_send_all` to bloom/echo | ⬜ Pending |
| `sprout` channel setup | Integer boot args for raw handles | ⬜ Pending (use `channel_create_fds` + FD inheritance) |

---

## 7. See Also

- `docs/concepts/ipc.md` — IPC primitive overview and decision matrix
- `docs/concepts/readiness.md` — unified poll/readiness model (all types)
- `docs/concepts/channel_semantics.md` — channel specification with deprecation notes
- `docs/ipc/inbox_vs_port_semantics.md` — semantic analysis of Inbox vs Port
- `docs/ipc/convergence_strategy.md` — layered convergence architecture and roadmap
- `docs/concepts/ipc_cookbook.md` — practical recipes for driver and service authors
- `docs/mouse-migration.md` — mouse input path migration example
- `docs/keyboard-migration.md` — keyboard input path migration example
- `kernel/src/vfs/inbox_node.rs` — InboxNode implementation

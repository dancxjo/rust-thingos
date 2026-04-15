# IPC Migration Status — Thing-OS

This document tracks the overall migration of IPC mechanisms in Thing-OS from
the legacy channel/port-centric model toward the plural substrate model
described in `docs/concepts/ipc.md`.

See `docs/ipc/convergence_strategy.md` for the strategic rationale and
roadmap.  See `docs/ipc/inbox_vs_port_semantics.md` for the semantic
analysis of Inbox vs Port.

---

## 1. Current Architecture Overview

Thing-OS IPC is built on a **plural substrate**:

| Primitive | Kernel module | Status |
|-----------|--------------|--------|
| **Channel** (byte-stream + msg) | `kernel/src/ipc/port.rs` | ✅ Stable |
| **Pipe** (anonymous byte stream) | `kernel/src/ipc/pipe.rs` | ✅ Stable |
| **Unix socket** (bidirectional byte stream) | `kernel/src/ipc/unix_socket.rs` | ✅ Stable |
| **Inbox** (typed ownership-first queue) | `kernel/src/inbox/mod.rs` | ✅ Stable |
| **Shared message queue core** | `kernel/src/ipc/msgqueue.rs` | ✅ Prototype |
| **Memfd** (shared memory / bulk data) | `kernel/src/vfs/` (memfd nodes) | ✅ Stable |
| **VFS poll** (`SYS_FS_POLL`) | `kernel/src/syscall/handlers/vfs.rs` | ✅ Stable |

### Transport selection guide

| Use case | Recommended primitive |
|----------|-----------------------|
| Discrete commands / events / RPC | Channel (`SYS_CHANNEL_*`) |
| Sequential byte streams, stdio | Pipe (`SYS_PIPE`) |
| Bidirectional byte-stream endpoint IPC | Unix socket (`SYS_SOCKET`) |
| Typed process/job lifecycle delivery | Inbox (`SYS_MSG_SEND`) |
| Bulk data, zero-copy ring buffers | Memfd (`SYS_MEMFD_CREATE`) |
| Readiness multiplexing | `SYS_FS_POLL` across all of the above |
| Legacy compat / in-kernel use | Channel only — do not use for new external protocols |

> **Rule**: channels are the right choice for discrete structured messages and
> capability passing.  Do not use them as byte streams, and do not introduce new
> driver or service protocols that are channel-only without a VFS/FD readiness
> path.

---

## 2. Migration Status by Area

### 2.1 Kernel IPC Primitives

| Item | Status | Notes |
|------|--------|-------|
| Channel / Port byte-stream path | ✅ Stable | `SYS_CHANNEL_SEND` / `SYS_CHANNEL_RECV` |
| Channel structured message path | ✅ Stable | `SYS_CHANNEL_SEND_MSG` / `SYS_CHANNEL_RECV_MSG` |
| `SYS_CHANNEL_WAIT` | ⛔ Deprecated | Use `SYS_FD_FROM_HANDLE` + `SYS_FS_POLL` instead |
| `SYS_CHANNEL_SEND_HANDLE` | ⛔ Deprecated | Use `SYS_CHANNEL_SEND_MSG` which bundles atomically |
| `SYS_CHANNEL_RECV_HANDLE` | ⛔ Deprecated | Use `SYS_CHANNEL_RECV_MSG` which receives atomically |
| Pipe (`SYS_PIPE` / `SYS_FS_*`) | ✅ Stable | Full VFS/FD readiness via `SYS_FS_POLL` |
| Unix socket | ✅ Stable | Full VFS/FD readiness via `SYS_FS_POLL` |
| Inbox (`SYS_MSG_SEND`, `SYS_MSG_BROADCAST`) | ✅ Stable | Typed delivery; VFS FD bridge available via `InboxNode` |
| `InboxNode` VFS wrapper | ✅ Done | `kernel/src/vfs/inbox_node.rs`; enables `SYS_FS_POLL` on inboxes |
| Shared `KernelMessageQueue` core | ✅ Prototype | `kernel/src/ipc/msgqueue.rs`; full migration to shared core pending |
| Channel VFS bridge (`SYS_FD_FROM_HANDLE`) | ✅ Stable | Exposes channel handles as pollable FDs |
| `SYS_FS_POLL` unified readiness | ✅ Stable | Works across pipes, sockets, channels (via FD bridge), inbox nodes |

### 2.2 Drivers

| Driver | IPC model used | Migration status |
|--------|---------------|-----------------|
| `virtio_netd` | Channel-heavy (VFS RPC over channels) | ⬜ Pending — VFS RPC is acceptable; poll path via `SYS_FD_FROM_HANDLE` |
| `ata_disk` | Channel-based (uses `SYS_CHANNEL_WAIT`) | ⬜ Pending — migrate `channel_wait` → `fd_from_handle + fs_poll` |
| `ahci_disk` | Channel-based (uses `SYS_CHANNEL_WAIT`) | ⬜ Pending — same as ata_disk |
| `display_fake` | Channel-based | ⬜ Pending — VFS/FD migration deferred |
| Other drivers | Mix of channels and VFS | ⬜ Assess per driver |

> Note: VFS RPC over channels (`SYS_FS_MOUNT` provider protocol) is a canonical
> use case and will remain channel-based.  The pending work is replacing
> `SYS_CHANNEL_WAIT` with `SYS_FS_POLL` in the blocking loops.

### 2.3 Userspace Libraries

| Component | Status | Notes |
|-----------|--------|-------|
| `stem::syscall::channel` | ✅ Stable | Core channel syscall wrappers |
| `stem::syscall::vfs::vfs_poll` | ✅ Stable | Unified readiness API |
| `stem::syscall::vfs::pipe` | ✅ Stable | Pipe creation |
| `ipc_helpers` (RPC helpers) | ✅ Stable | Request/reply abstractions over channels |
| Userspace `channel_wait` wrappers | ⬜ Pending | Migrate callers to `fd_from_handle + vfs_poll` |

### 2.4 Documentation

| Document | Status | Notes |
|----------|--------|-------|
| `docs/concepts/ipc.md` | ✅ Done | Canonical primitive overview |
| `docs/concepts/ipc_cookbook.md` | ✅ Done | Practical recipes per primitive |
| `docs/concepts/channel_semantics.md` | ✅ Done | Full channel spec |
| `docs/concepts/channels_vs_pipes.md` | ✅ Done | When to use each |
| `docs/concepts/readiness.md` | ✅ Done | Unified readiness model |
| `docs/concepts/memfd.md` | ✅ Done | Shared memory / bulk transfer |
| `docs/ipc/convergence_strategy.md` | ✅ Active | Layered convergence strategy |
| `docs/ipc/inbox_vs_port_semantics.md` | ✅ Done | Semantic analysis |
| `docs/ipc/ipc_migration_status.md` | ✅ This file | Migration tracking |
| Channel deprecation guidance | ✅ Done | `channel_semantics.md` §8 + `abi/src/numbers.rs` comments |

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

### 3.2 New service or driver protocols

When writing a new service or driver:

1. **Prefer VFS-first**: expose state as files under `/dev`, `/services`,
   or `/run`.  Clients read/write/poll those files directly.
2. **Use channels for discrete commands**: `SYS_CHANNEL_SEND_MSG` for typed
   requests; `SYS_CHANNEL_RECV_MSG` for replies with capability passing.
3. **Use pipes for byte streams**: never stream raw audio, video, or bulk
   bytes over a channel.
4. **Use memfd for bulk data**: pass a `MemFdRef` in the channel message
   payload; the receiver maps the memfd for zero-copy access.
5. **Add a readiness path**: every blocking source must be pollable via
   `SYS_FS_POLL`.  If using channels, expose them via `SYS_FD_FROM_HANDLE`.
   If using inboxes, open them as `InboxNode`-backed FDs.
6. **Do not use `SYS_CHANNEL_WAIT`**: it is deprecated.  Use
   `SYS_FD_FROM_HANDLE` + `SYS_FS_POLL` instead.

### 3.3 Delivery semantics

| Delivery type | Primitive | Notes |
|---------------|-----------|-------|
| Point-to-point command | Channel (`SEND_MSG`) | One sender, one receiver; reply via same or separate channel |
| Broadcast to process group | `SYS_MSG_BROADCAST` | Inbox-targeted; membership snapshot at send time |
| Typed process/job event | `SYS_MSG_SEND` → Inbox | Ownership-first; receiver-keyed by PID |
| Byte stream pipeline | Pipe | No message boundaries; sequential |
| Bidirectional session | Unix socket pair | Bytestream; filesystem-addressable |

---

## 4. Deprecation Register

| Syscall / API | Status | Replacement |
|---------------|--------|-------------|
| `SYS_CHANNEL_WAIT` | ⛔ Deprecated | `SYS_FD_FROM_HANDLE` + `SYS_FS_POLL` |
| `SYS_CHANNEL_SEND_HANDLE` | ⛔ Deprecated | `SYS_CHANNEL_SEND_MSG` (atomic) |
| `SYS_CHANNEL_RECV_HANDLE` | ⛔ Deprecated | `SYS_CHANNEL_RECV_MSG` (atomic) |
| `stem::syscall::channel_wait` | ⬜ Migration pending | `fd_from_handle` + `vfs_poll` |

Deprecated syscalls remain in the ABI for compatibility but will not receive
new features.  New code must not use them.

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
| A — Spec lock | Land semantics doc (`inbox_vs_port_semantics.md`) | ✅ Done |
| A — Spec lock | Land strategy doc (`convergence_strategy.md`) | ✅ Done |
| A — Spec lock | Glossary note: Port = connection-first, Inbox = ownership-first | ✅ Done (in strategy doc §1) |
| B — Shared core | Introduce `KernelMessageQueue` prototype | ✅ Done (`kernel/src/ipc/msgqueue.rs`) |
| B — Shared core | Migrate Inbox and Port message path onto shared core | ⬜ Pending (Phase B/C full migration) |
| C — Readiness | Add inbox VFS wrapper node (`InboxNode`) | ✅ Done (`kernel/src/vfs/inbox_node.rs`) |
| C — Readiness | Expose inbox FD acquisition syscall / path-open | ⬜ Pending |
| C — Readiness | Tests for mixed poll sets (files + channels + inbox FDs) | ⬜ Pending |
| D — Deprecation | Document migration off `SYS_CHANNEL_WAIT` | ✅ Done (this document + `channel_semantics.md`) |
| D — Deprecation | Userspace `channel_wait` → `fd_from_handle + fs_poll` | ⬜ Pending |

---

## 6. Blockers and Dependencies

| Blocker | Affects | Notes |
|---------|---------|-------|
| Inbox FD acquisition path not yet syscall-exposed | C — Readiness step 2 | `InboxNode` exists in kernel; needs syscall to open it from userspace |
| Driver `SYS_CHANNEL_WAIT` usage | Driver migration | ata_disk, ahci_disk, display_fake; low urgency |
| No mixed-poll-set tests | C — Readiness step 3 | Add to `userspace/poll_mux` or a dedicated test |
| Full Inbox + Port convergence onto `KernelMessageQueue` | B — Shared core | Low urgency; prototype validates feasibility |

---

## 7. Cross-References

- Primitive overview and transport selection: `docs/concepts/ipc.md`
- Practical recipes: `docs/concepts/ipc_cookbook.md`
- Convergence strategy and roadmap: `docs/ipc/convergence_strategy.md`
- Semantic analysis: `docs/ipc/inbox_vs_port_semantics.md`
- Unified readiness contract: `docs/concepts/readiness.md`
- Channel specification: `docs/concepts/channel_semantics.md`
- Shared queue prototype: `kernel/src/ipc/msgqueue.rs`
- Inbox VFS node: `kernel/src/vfs/inbox_node.rs`

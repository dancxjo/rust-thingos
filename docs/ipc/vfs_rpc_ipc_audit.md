# VFS RPC IPC Transport — Audit and Migration Guide

This document audits the current VFS RPC channel-based protocol, identifies
issues with the implicit `resp_port`-in-header design, and provides per-service
substrate recommendations and a concrete migration path.

---

## 1. Current Transport Design

### 1.1 Wire protocol

Every VFS RPC request sent from the kernel to a provider carries a 7-byte
header (defined in `abi/src/vfs_rpc.rs`):

```rust
#[repr(C, packed)]
pub struct VfsRpcReqHeader {
    pub resp_port: u32,  // write end of the kernel's private response channel
    pub op: u8,          // VfsRpcOp discriminant
    pub _pad: [u8; 2],
}
```

The kernel sends this header followed by op-specific payload bytes over the
provider channel.  The provider reads the header, extracts `resp_port`, and
sends its response back to `resp_port` using `channel_send_all`.

### 1.2 Channel topology (per provider mount)

```
  ┌─────────┐      req channel           ┌──────────────┐
  │  kernel │  ──[req_write → req_read]──▶│   provider   │
  │         │                             │  (userspace) │
  │         │◀─[resp_write ← resp_read]──  │              │
  └─────────┘      resp channel           └──────────────┘
```

- `req_write` — kernel-held write end; kernel sends `[resp_port][op][payload]`
- `req_read`  — provider-held read end; provider polls for RPC requests
- `resp_write` (`resp_port`) — kernel-private write handle **sent inside each
  request message**
- `resp_read`  — kernel-held read end; kernel blocks waiting for the response

### 1.3 Kernel implementation

`kernel/src/vfs/provider.rs` → `ProviderChannel::rpc()`:

1. Creates the response port once at mount time (`ProviderFs::new`).
2. For each VFS operation, packs `resp_write_handle` into the header, sends it
   together with the op payload.
3. Blocks synchronously on `resp_read` for a single-message reply.
4. The round-trip is serialised per-request; there is no in-flight pipelining.

---

## 2. Issues with the Current Design

| # | Issue | Impact |
|---|-------|--------|
| 1 | `resp_port` embedded in message | Provider receives a raw handle number that it did not create.  A buggy provider that stores or forwards this handle could accidentally address the kernel's private channel. |
| 2 | `resp_port` always the same value | 4 bytes wasted in every request header. The kernel could remove the field and teach the provider the response handle once at mount time. |
| 3 | Drivers bypass `ProviderLoop` helper | `virtio_netd`, `virtio_sound`, and `netd` all parse `VfsRpcReqHeader` by hand and build responses as raw byte vectors, duplicating framing logic that `ProviderLoop` already encapsulates. |
| 4 | `channel_send` (non-atomic) used in some drivers | `virtio_netd` and `netd` call `channel_send` rather than `channel_send_all`, allowing partial writes that corrupt the response framing. |
| 5 | Blocking `next_request()` unsuitable for event loops | `ProviderLoop::next_request()` was the only variant before this migration; drivers that must interleave hardware polling with VFS RPC handling could not use it and fell back to raw `channel_try_recv`. |

---

## 3. Migration Plan

### 3.1 Short-term (completed in this branch)

#### A — `ProviderLoop::try_next_request()` (non-blocking variant)

**File**: `libs/ipc_helpers/src/provider.rs`

Added a non-blocking counterpart to `next_request()` that calls
`channel_try_recv` and returns `Ok(None)` on `EAGAIN`.  Event-loop drivers
use this to poll for pending VFS RPC work without blocking hardware polling.

#### B — `ProviderResponse::ok_poll(revents)` helper

Added the missing poll-response builder alongside the existing `ok_u64`,
`ok_stat`, `ok_read`, `ok_written` helpers so that all standard response types
can be constructed without raw byte manipulation.

#### C — `drivers/virtio_netd` migration (prototype)

**Files**: `drivers/virtio_netd/src/vfs_provider.rs`, `main.rs`,
`Cargo.toml`

- Replaced raw `VfsRpcReqHeader` parsing with `ProviderLoop::try_next_request`.
- Replaced all `send_resp`/`send_err`/`send_ok_*` helpers with `ProviderResponse::*`.
- All handler functions (`handle_lookup`, `handle_stat`, etc.) now return
  `ProviderResponse` instead of calling `send_resp` directly.
- Main loop calls `ProviderLoop::send_response(req.resp_port, resp)`, which
  uses the atomic `channel_send_all`.
- Removed the direct `abi::errors` EAGAIN check; `ProviderLoop` handles it.

#### D — `drivers/virtio_sound` migration

**Files**: `drivers/virtio_sound/src/main.rs`, `Cargo.toml`

- Replaced the five hand-rolled `resp_ok_*/resp_err` builders with
  `ProviderResponse::*`.
- `dispatch_rpc` now returns `(ProviderResponse, bool)` instead of
  `(Vec<u8>, bool)`.
- `dispatch_device_call` similarly returns `(ProviderResponse, bool)`.
- Main event loop uses `ProviderLoop::try_next_request` + `send_response`.

#### E — `userspace/netd` — FIXME comment

Added a `## IPC substrate — migration note` section to
`userspace/netd/src/vfs_provider.rs` with a `FIXME` directing future work
to use `ProviderLoop`.  The full migration is deferred because `netd`'s VFS
provider is significantly more complex (TCP/UDP socket state, dynamic handle
allocation) and warrants its own PR.

### 3.2 Medium-term — explicit dual-channel mount

The cleanest removal of `resp_port`-in-header requires changing the
`SYS_FS_MOUNT` syscall signature so that the provider supplies **both** the
request read channel and the response write channel at mount time:

```
// Current
SYS_FS_MOUNT(req_write_handle, path_ptr, path_len) → 0

// Proposed
SYS_FS_MOUNT(req_write_handle, resp_read_handle, path_ptr, path_len) → 0
```

The kernel would store `resp_read_handle` alongside the existing
`req_write_handle` and remove `resp_port` from `VfsRpcReqHeader` (saving 4
bytes per request and eliminating the capability-in-payload concern).

The provider would hold `resp_write_handle` itself and call
`channel_send_all(resp_write_handle, &response)` directly.

`ProviderLoop` would be updated to accept a `(req_read, resp_write)` pair.
All providers would require a small update to `new()` and their channel setup.

**Prerequisite**: coordinate with any in-flight work on `SYS_FS_MOUNT` — see
the overarching IPC migration issue for ordering.

### 3.3 Long-term per-service substrate recommendation

| Service | Current substrate | Recommended substrate | Notes |
|---------|-------------------|----------------------|-------|
| `kernel` → any VFS provider | Channel + `resp_port` in header | Dual-channel (see §3.2) | Remove `resp_port` from header; one-time setup cost |
| `display_bootfb` | Channel + `ProviderLoop` | No change needed | Already uses the canonical helper |
| `virtio_netd` | Channel + raw header (**migrated**) | Channel + `ProviderLoop` | Done in this branch |
| `virtio_sound` | Channel + raw header (**migrated**) | Channel + `ProviderLoop` | Done in this branch |
| `iso9660d` | Channel + `ProviderLoop` | No change needed | Already uses the canonical helper |
| `ipc_provider_demo` | Channel + `ProviderLoop` | No change needed | Already uses the canonical helper |
| `userspace/netd` | Channel + raw header | Channel + `ProviderLoop` | FIXME added; full migration in follow-up PR |
| PCM audio stream (`out0`) | Channel (bulk PCM over VFS Write) | Pipe or memfd ring | See `channels_vs_pipes.md` §6 — anti-pattern |
| Network RX/TX stream | Channel (bulk frames over VFS Read/Write) | Pipe (rx/tx) | Length-prefixed frames over pipe avoids channel overhead |

---

## 4. Invariants to Preserve During Migration

1. **Wire compatibility**: `VfsRpcReqHeader` format must not change until all
   kernel and provider code is updated atomically (kernel and providers must
   agree on the header layout).

2. **Atomic response sends**: always use `channel_send_all` (not
   `channel_send`) for VFS RPC responses.  Partial writes corrupt the framing.

3. **Non-blocking providers**: providers running event loops MUST use
   `try_next_request` (or `channel_try_recv`) to avoid blocking hardware
   polling.  Blocking providers (simple file-shaped services) may use
   `next_request`.

4. **EAGAIN handling**: `try_next_request` returns `Ok(None)` on empty channel.
   Do not retry in a tight spin-loop without yielding — use `sleep_ms(1)` or
   `yield_now()`.

---

## 5. Code Locations

| Path | Role |
|------|------|
| `abi/src/vfs_rpc.rs` | Wire types: `VfsRpcReqHeader`, `VfsRpcOp`, size constants |
| `kernel/src/vfs/provider.rs` | Kernel-side provider channel: `ProviderFs`, `ProviderChannel` |
| `kernel/src/syscall/handlers/vfs.rs` | `sys_fs_mount` handler |
| `libs/ipc_helpers/src/provider.rs` | `ProviderLoop`, `ProviderRequest`, `ProviderResponse` |
| `drivers/display_bootfb/src/vfs_provider.rs` | Reference: canonical `ProviderLoop` usage |
| `drivers/virtio_netd/src/vfs_provider.rs` | Migrated: `ProviderLoop` prototype |
| `drivers/virtio_sound/src/main.rs` | Migrated: `ProviderLoop` with hardware event loop |
| `userspace/iso9660d/src/main.rs` | Reference: canonical `ProviderLoop` usage |
| `userspace/netd/src/vfs_provider.rs` | Pending: FIXME comment added |
| `userspace/ipc_provider_demo/src/main.rs` | Reference: minimal provider skeleton |

---

## 6. See Also

- `docs/concepts/ipc.md` — IPC primitive overview and decision matrix
- `docs/concepts/vfs_rpc_provider.md` — provider lifecycle and wire protocol
- `docs/concepts/channels_vs_pipes.md` — channel vs pipe, §6 known deviations
- `docs/concepts/ipc_cookbook.md` — practical recipes including provider skeleton
- `abi/src/vfs_rpc.rs` — wire types
- `libs/ipc_helpers/src/provider.rs` — `ProviderLoop` API

# Thing-OS Concurrency Architecture

This document defines the canonical three-layer concurrency model for
Thing-OS.  All new services, drivers, and runtime crates must follow these
layers.  The [concurrency-index.txt] in the repository root maps every
current usage to its layer.

---

## The Three Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  Layer 3 — InboxLoop / Actor / Service                          │
│  Blessed userspace / service API                                │
│  ipc_helpers::inbox, InboxReceiver, sprout supervision          │
├─────────────────────────────────────────────────────────────────┤
│  Layer 2 — WakeSet / vfs_poll                                   │
│  Runtime-level readiness aggregation                            │
│  stem::wait_set::WaitSet, stem::syscall::vfs::vfs_poll          │
├─────────────────────────────────────────────────────────────────┤
│  Layer 1 — WaitSource / WaitKind / wait_many                    │
│  Kernel-level wait primitive                                    │
│  abi::wait, kernel::syscall::handlers::wait, SYS_WAIT_MANY      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Layer 1 — Kernel Wait Primitive

**Crates / files:** `abi/src/wait.rs`, `kernel/src/syscall/handlers/wait.rs`,
`stem/src/syscall/wait.rs`

`WaitMany` is the single kernel mechanism that blocks a task until one or more
event sources become ready.  It accepts an array of `WaitSpec` descriptors and
returns an array of `WaitResult` values.

| Type | Role |
|------|------|
| `WaitKind::Fd` | Wait on any VFS file descriptor (pipe, socket, inbox node, port FD bridge, device node). **Primary kind for all new code.** |
| `WaitKind::TaskExit` | Wait for a child task to exit. |
| `WaitKind::Irq` | Wait for a hardware interrupt handle to fire. |
| `WaitKind::Timeout` | Internal: global timeout for a `wait_many` call. |
| `WaitKind::Port` *(deprecated)* | Legacy port handle wait. Bridge via `SYS_FD_FROM_HANDLE`, then use `WaitKind::Fd`. |
| `WaitKind::RootWatch` *(deprecated → ENOSYS)* | Removed. Use `WaitKind::Fd` on a watch FD. |
| `WaitKind::GraphOp` *(deprecated → ENOSYS)* | Removed. Use FD readiness. |

**Rule:** nothing above Layer 1 should call the raw `SYS_WAIT_MANY` syscall or
construct `WaitSpec` arrays directly.  Use `WaitSet` (Layer 2) instead.

---

## Layer 2 — WakeSet (Runtime Readiness Aggregation)

**Crates / files:** `stem/src/wait_set.rs`, `stem/src/syscall/vfs.rs`
(`vfs_poll`)

Layer 2 provides two equivalent readiness APIs that sit on top of
`wait_many`:

### `WaitSet`

```rust
use stem::wait_set::{WaitSet, WaitEvent};

let mut ws = WaitSet::new();
let tok_inbox = ws.add_fd_readable(inbox_fd)?;
let tok_irq   = ws.add_irq(irq_handle)?;

for ev in ws.wait(Some(Duration::from_secs(1)))? {
    if ev.token() == tok_inbox && ev.is_readable() { /* drain inbox */ }
    if ev.token() == tok_irq   && ev.is_irq()      { /* service irq */ }
}
```

`WaitSet` is suitable for drivers and low-level runtime crates that need to
multiplex a heterogeneous set of sources (IRQs, FDs, task-exit signals)
without allocating per-source threads.

### `vfs_poll`

```rust
use stem::syscall::vfs::vfs_poll;
use abi::syscall::PollHandle;

let mut fds = [
    PollHandle { handle: inbox_fd as i32, events: POLLIN, revents: 0 },
    PollHandle { handle: net_sock as i32, events: POLLIN, revents: 0 },
];
let n = vfs_poll(&mut fds, timeout_ms)?;
```

`vfs_poll` is the POSIX-compatible poll interface.  It is equivalent to
`WaitSet` for FD-only use cases and is preferred in higher-level services that
may need POSIX portability.

**Deprecated Layer 2 APIs** (do not use in new code):

| API | Replacement |
|-----|-------------|
| `WaitSet::add_port_readable` | `vfs_fd_from_port` then `add_fd_readable` |
| `WaitSet::add_port_writable` | `vfs_fd_from_port` then `add_fd_writable` |
| `WaitSet::add_vfs_watch`     | `add_fd_readable` |
| `WaitSet::add_graph_op`      | `add_fd_readable` on an event FD |

---

## Layer 3 — InboxLoop / Actor / Service

**Crates / files:** `kernel/src/inbox/mod.rs`, `kernel/src/vfs/inbox_node.rs`,
`stem/src/syscall/message.rs`, `libs/ipc_helpers/src/inbox.rs`

Every long-running userspace service is an **inbox-backed actor**.  The actor
owns a process inbox (exposed as a pollable VFS FD via `/proc/self/inbox`) and
processes typed messages in a loop.

### Canonical service loop

```rust
use ipc_helpers::inbox::{InboxReceiver, KindId};
use stem::syscall::message::msg_inbox_open_self;
use stem::syscall::vfs::vfs_poll;
use abi::syscall::PollHandle;

fn run() {
    let inbox_fd = msg_inbox_open_self().expect("open inbox FD");
    let rx = InboxReceiver::new(4096);
    let mut fds = [PollHandle { handle: inbox_fd as i32, events: POLLIN, revents: 0 }];

    loop {
        vfs_poll(&mut fds, u64::MAX).ok();
        loop {
            match rx.try_recv() {
                Ok(msg) => dispatch(msg),
                Err(_)  => break,  // inbox empty
            }
        }
    }
}
```

For services that also need to watch files or device nodes, the inbox FD can
be combined with other FDs in the same `vfs_poll` or `WaitSet` call.

### Sending to a service

```rust
use ipc_helpers::inbox::{send_typed, KindId};

send_typed(target_pid, KindId::MY_EVENT, payload)?;
```

### When to use inbox vs port

| Criterion | Inbox (Layer 3) | Port (legacy Layer 2/C) |
|-----------|-----------------|-------------------------|
| Recipient identity | Process by PID | Explicit handle exchange |
| Connection setup | None | `port_create` + publish handle |
| Fan-out | `msg_broadcast` to pgid | Manual fan-out to N handles |
| Bulk data | Inline payload (≤ 4 KB) or `SharedMemoryRef` handle | Byte-stream ring buffer |
| Good for | Lifecycle events, control messages, typed signals | Streaming data, legacy VFS providers |

### Sprout supervision contract

Sprout supervises every service as an actor with:
1. A **declared set of facets** (mount paths, published services) sent as
   inbox messages on startup.
2. A **Ready** message (`KindId::DRIVER_READY`) delivered to Sprout's inbox
   when the service has finished initialising.
3. Restart / liveness probing via `WaitKind::TaskExit` on the task ID, with
   a re-spawn on unexpected exit.

---

## Rules for New Code

1. **New services must not call `wait_many` directly.**  Use `vfs_poll` or
   `WaitSet` (Layer 2) with an inbox FD from `msg_inbox_open_self`.

2. **New VFS providers must be inbox-backed actors.**  The provider port
   (required by the kernel VFS protocol) is a single FD added to the actor's
   `WaitSet`; dispatch is inbox-driven for control messages and port-driven
   only for the narrow kernel RPC path.

3. **Driver control paths must be inbox messages.**  A driver's main loop
   polls its inbox FD; requests arrive as typed messages with a
   `SharedMemoryRef` pointing to a DMA buffer where payload is too large for
   inline delivery.

4. **Bulk data uses memfd / ring / DMA buffers passed by handle.**  The
   control message carries a `SharedMemoryRef`; the receiver maps the handle
   once and accesses the data directly.  Do not copy bulk data through port
   rings.

5. **Ports, channels, sockets, and pipes are projections over inbox/wake
   machinery.**  They are valid for POSIX compatibility and stdio pipelines,
   but should not be invented as primary IPC channels for new services.

6. **Sprout supervises services as actors with inboxes and declared facets.**
   New services register with Sprout via an inbox `Ready` message; Sprout
   never polls port FDs for driver readiness in new code.

---

## Migration Target: sprout ManagedTask boot handshake

The first identified migration to the inbox-actor model is
`sprout/src/task.rs` — `ManagedTask`.

**Current (port-based):** each supervised task carries four `PortHandle`
fields (`drv_req_write`, `drv_resp_read`, `boot_req_read`,
`boot_resp_write`) plus a bridged `resp_fd`.  Sprout polls `resp_fd` to
detect driver readiness.

**Target (inbox-based):**
- The driver sends `KindId::DRIVER_READY` to Sprout's PID via `msg_send`
  when initialisation completes.
- Facet registration (mount paths, capability names) travels as structured
  payload in the same message (or a follow-up `KindId::FACET_REGISTER`
  message).
- Sprout's event loop adds only its own inbox FD to the `WaitSet` / `vfs_poll`
  call; no per-task port FDs are required.
- `ManagedTask` drops all four `PortHandle` fields; the `resp_fd` field is
  removed.

This migration requires changes only to `sprout/` and `cambium/`; no kernel
changes are needed because all required primitives (`msg_send`,
`msg_inbox_open_self`, `vfs_poll`) already exist.

---

## Related Documents

- `abi/src/wait.rs` — canonical `WaitKind`, `WaitSpec`, `WaitResult` types
- `stem/src/wait_set.rs` — `WaitSet` implementation and API docs
- `libs/ipc_helpers/src/inbox.rs` — `InboxReceiver` and send helpers
- `docs/ipc/` — IPC convergence strategy and migration phases
- `concurrency-index.txt` — complete inventory of every concurrency pattern
  usage site in the codebase

# Inbox vs Port Semantics (Issue 46)

Status: Draft design analysis
Scope: kernel and userspace messaging semantics in current Thing-OS tree

## 1. Minimal canonical semantics

### Inbox

Definition:
A bounded FIFO queue owned by a receiving identity (task/process/job observer), optimized for arrival ownership and typed message delivery.

Ownership model:
- Kernel object identity: `InboxId` in a global inbox registry.
- Concrete object: `Inbox` (`Arc`-managed).
- Typical binding: process/job-facing delivery targets (for example typed process messages and job-exit observers).

Delivery guarantees:
- FIFO order per inbox.
- Whole-message enqueue (no partial write).
- Bounded capacity; full inbox rejects enqueue (`Full`/`EAGAIN` style behavior in callers).
- At-most-once enqueue effect for each successful send.

Blocking semantics:
- Primitive operation is non-blocking dequeue (`try_recv`).
- Blocking receive is scheduler-layer policy: caller registers on inbox wait queue and blocks.
- Close semantics are explicit: send fails after close; receivers are woken; queued messages can drain before final closed result.

Readiness model:
- Inbox has a wait queue and a first-class VFS FD surface via `InboxNode`
  (`kernel/src/vfs/inbox_node.rs`).
- Directly pollable by `SYS_FS_POLL` through `InboxNode` (Phase C step 1
  complete; see `docs/ipc/convergence_strategy.md`).

### Port (user-facing channel backing)

Definition:
A connection-oriented endpoint abstraction identified by explicit handles (read/write modes), backing channel syscalls and optionally exposed as VFS nodes.

Connection model:
- Created as paired endpoints (one read handle, one write handle).
- Endpoint liveness matters (`has_readers`/`has_writers`), including peer-death semantics (`EPIPE`/`POLLHUP`).
- Handle-mode capability gating (`Read` or `Write`).

Message vs stream semantics:
- Byte-stream queue path: `send`/`recv` over bounded ring buffer.
- Structured message queue path: `send_msg`/`recv_msg` with bytes plus attached capabilities.
- Current port combines both stream and message queues under one endpoint object.

FD/handle integration:
- Native handle table for channel syscalls.
- Explicit bridge to VFS FD via `SYS_FD_FROM_HANDLE`.
- Poll/readiness integrated through VFS `poll()` and `SYS_FS_POLL`.

## 2. Are these the same thing?

Short answer:
No. They overlap in queue mechanics, but they are not the same abstraction boundary.

Direct answers:
- Is Inbox just a Port with one end hidden?
  - Not in current semantics. Inbox has ownership-first semantics and no required peer endpoint contract.
- Is Port just a bidirectional Inbox pair?
  - Partially true only for one sub-case (message queue interpretation). It does not capture explicit endpoint capability modes, stream behavior, and peer-liveness semantics.

## 3. Irreducible vs accidental differences

### Irreducible differences (semantic)

| Dimension | Inbox | Port/Channel |
|---|---|---|
| Primary concern | Ownership of arrival | Connection between endpoints |
| Identity | Receiver-owned object (`InboxId`) | Explicit endpoint handle(s) |
| Liveness model | Closed/open queue state | Peer endpoint liveness (reader/writer counts) |
| Poll integration | First-class via `InboxNode` VFS wrapper + `SYS_FS_POLL` | First-class via `PortNode` + `SYS_FS_POLL` |
| Data contract | Message unit only | Stream bytes plus structured messages/caps |
| Error surface | Full/closed queue semantics | EPIPE/EAGAIN/readable-writable-hup matrix |

### Accidental differences (API artifacts, can be adapted)

| Difference today | Why accidental | Adapter possibility |
|---|---|---|
| Different verb sets (`send/recv` vs `enqueue/dequeue`) | Naming and layer history | Provide facade aliases by role |
| Handle-table vs registry lookup | Exposure choice, not queue law | Add inbox handles or port-backed inbox view |
| Poll only on port FDs | Missing wrapper, not impossible | ✅ `InboxNode` VFS wrapper now implemented |
| Typed-message path split from channel path | Transitional layering | Shared kernel message record format |

## 4. Behavioral mapping (operations)

| Inbox operation | Port/channel equivalent | Equivalence quality | Notes |
|---|---|---|---|
| `enqueue(msg)` | `send_msg(handle, data,caps)` | Partial | Port requires writer endpoint handle and peer semantics |
| `dequeue()` | `recv_msg(handle)` or `recv(handle,buf)` | Partial | Port supports stream mode that Inbox does not |
| `try_dequeue()` | `try_recv`/`recv_msg -> EAGAIN` | Good | Both can expose non-blocking empty |
| wait for arrival | `add_waiter_read + block` | Good | Mechanically similar wait queues |
| close inbox | `channel_close(handle)` | Partial | Port close is endpoint-scoped, not single owner queue closure |

Missing/mismatched capabilities:
- Inbox FD/poll bridge is now implemented (`InboxNode` in `kernel/src/vfs/inbox_node.rs`).
- Port lacks ownership-first identity model; everything is endpoint connection-centric.
- Port stream-mode behavior (partial write/read ring semantics) has no Inbox analog.

## 5. Blocking and readiness under VFS-first poll

Current state:
- Port/channel is already unified with VFS readiness through `PortNode::poll` and `SYS_FS_POLL`.
- Deprecated `SYS_CHANNEL_WAIT` confirms intended direction: convert handles to FDs, then use `SYS_FS_POLL`.
- Inbox wait is now represented as a pollable VFS node via `InboxNode` (`kernel/src/vfs/inbox_node.rs`).

Unified readiness model (implemented):
1. Single user-visible readiness API: `SYS_FS_POLL`.
2. `InboxNode` implements `VfsNode`:
- `poll`: readable when queue non-empty or closed (EOF readable).
- `POLLHUP`: inbox is closed.
- `POLLOUT`: inbox has capacity and is not closed (sender-side readiness).
- `add_waiter/remove_waiter`: forwarded to inbox wait queue.
3. Inbox FDs are obtained by wrapping an `Arc<Inbox>` in `InboxNode` and exposing it through the VFS thing table.

Backpressure semantics:
- Inbox should remain bounded and reject on full.
- For pollable inbox sender-side semantics, expose `POLLOUT` only when capacity available.

Kernel wait-queue need:
- Already present in Inbox primitive and sufficient for poll integration once wrapped as `VfsNode`.

## 6. Phase-1/2 conclusion

Decision-quality statement:
- Inbox and Port are not equivalent primitives.
- They share a reusable kernel queue skeleton, but their semantic boundaries are different:
  - Port: connection and endpoint capability semantics.
  - Inbox: ownership and arrival semantics.

Convergence seam:
- Converge at a shared internal queue core and readiness contract.
- Keep distinct public views for ownership-first and connection-first use cases.

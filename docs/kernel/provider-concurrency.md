# VFS Provider Concurrency Model

> **Status**: kernel-side multiplexing is complete and correct.  No kernel
> changes are needed for this area.  The remaining work is in userland
> providers.

---

## Overview

Thing-OS VFS operations that fall under a userland-mounted filesystem are
forwarded to the provider process via an IPC port.  The kernel-side RPC
implementation in
[`kernel/src/vfs/provider.rs`](../../thingos/kernel/src/vfs/provider.rs)
was designed from the start to support **many concurrent callers**.

---

## Kernel-side multiplexing (already done)

### Request IDs

Every call to `ProviderRpc::rpc` atomically claims a 16-bit `req_id` via a
`fetch_add` on an `AtomicU16`.  The `req_id` is embedded in the request header
so the provider can echo it back in the response.

```
 ┌──────────────────────┐
 │  VfsRpcReqHeader     │
 │  ─────────────────── │
 │  resp_port : u32     │  ← kernel's private response port handle
 │  op        : u8      │  ← VfsRpcOp variant
 │  req_id    : u16     │  ← unique per in-flight request
 └──────────────────────┘
```

### Per-request wait queues

Each in-flight `rpc()` call registers a `WaitQueue` in `RpcState::waiters`
keyed by its `req_id`.  Blocked callers sleep on their own queue and are woken
only when their response arrives or a taint/timeout fires.

### Response routing

The shared response port is read by whichever caller wakes first.  The `req_id`
in the first two bytes of every response determines routing:

- **Matches caller's `req_id`**: caller returns immediately with the payload.
- **Belongs to another waiter**: payload is parked in `RpcState::responses` and
  that waiter's `WaitQueue` is woken so it can collect the response on its next
  iteration.

This means **up to 65 535 kernel threads** can be in flight to the same
provider simultaneously without any serialisation beyond the brief spinlock
windows around the response map.

### Taint mechanism

If the provider dies (send fills, response port loses all writers), the RPC
state is marked *tainted* for `PROVIDER_TAINT_COOLDOWN_NS` (2 s).  All
waiters are woken and return `EIO`.  After the cooldown the taint clears
automatically so that a restarted provider can service new requests.

---

## Current bottleneck: single-threaded userland providers

The kernel path is fully concurrent, but observed parallelism is throttled by
**providers that handle requests one at a time**.  A typical single-threaded
provider loop looks like:

```
loop {
    let req = port.recv();      // blocks until a request arrives
    let resp = handle(req);     // potentially slow I/O here
    port.send(resp);            // reply
}
```

While `handle(req)` runs, every other kernel caller that issued an RPC to this
provider is blocked — even though the kernel is ready to dispatch them.

### Impact

Services known to be affected by this bottleneck:

| Provider | Issue |
|----------|-------|
| `httpsd` | HTTP requests are serialised; concurrent clients stall each other |
| `iso9660d` | ISO 9660 directory reads are serialised; parallel file opens stall |

---

## Recommended provider patterns

To allow all concurrent kernel callers to make progress simultaneously, adopt
one of the following patterns in your provider:

### 1. Thread-per-request (simplest)

```rust
loop {
    let req = port.recv();
    std::thread::spawn(move || {
        let resp = handle(req);
        resp_port.send(resp);
    });
}
```

Suitable for providers where each request is independent and short-lived.

### 2. Fixed-size thread pool

Use a bounded channel as a work queue and a fixed number of worker threads.
Prevents unbounded thread creation under heavy load while still allowing
concurrency up to the pool size.

### 3. Async / event-driven

An async runtime (e.g. a custom executor over `SYS_WAIT_MANY`) can pipeline
many requests with a single OS thread.  Appropriate for I/O-bound providers
where most time is spent waiting rather than computing.

---

## Guidance for contributors

1. **Do not modify the kernel RPC path to add per-provider serialisation**:
   the kernel already supports full concurrency.  Serialising requests in the
   kernel would regress performance for providers that *are* parallel.

2. **Provider parallelism work belongs in userspace**: update the relevant
   provider binary (e.g. `thingos/httpsd/`, `thingos/drivers/iso9660d/`) to adopt one of
   the patterns above.

3. **Test with concurrent kernel callers**: a provider that spawns worker
   threads should be validated with multiple simultaneous VFS callers to
   confirm there are no response-ordering bugs (the kernel routes by `req_id`
   so response order does not matter, but the provider must echo the correct
   `req_id` in every reply).

4. **The `req_id` field is the contract**: providers must copy the `req_id`
   from the request header verbatim into the response header.  Failing to do
   so will cause the response to be silently discarded or routed to the wrong
   waiter.

---

## Source references

| File | Relevance |
|------|-----------|
| [`thingos/kernel/src/vfs/provider.rs`](../../thingos/kernel/src/vfs/provider.rs) | `ProviderRpc::rpc`, `RpcState` — kernel multiplexing implementation |
| [`abi/src/vfs_rpc.rs`](../../abi/src/vfs_rpc.rs) | `VfsRpcReqHeader` — wire format including `req_id` |
| [`docs/kernel/fs-semantics.md`](fs-semantics.md) | Filesystem semantics checklist |
| [`docs/concepts/thingos-guardrails.md`](../concepts/thingos-guardrails.md) | Architecture guardrails (userland-drivers rule) |

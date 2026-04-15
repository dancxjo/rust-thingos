# IPC Typed-World Audit

Status: Architecture audit  
Scope: channels, ports, pipes, sockets, inbox/message paths, capability transfer

## 1) Canonical communication model

Thing-OS canonical IPC is **typed message exchange between Things**:

- A communication endpoint is a **Thing** with a Kind-defined contract.
- Payload units are **Message** envelopes (discrete, typed units).
- Access is by capability reference (today mostly "thing"/handle integers; future typed handles).
- Bulk bytes are not the canonical control model; they are a compatibility/data-plane projection.

Practical rule: **control plane = messages**, **bulk/stream plane = byte streams or memfd**.

## 2) Primitive classification (canonical vs projection)

| Primitive | Role in typed-world lens | Classification |
|---|---|---|
| Channel (`SYS_CHANNEL_*`) | Primary endpoint for discrete IPC and capability transfer | **Canonical IPC surface** |
| Port (`kernel/src/ipc/port.rs`) | Ring/wait-queue implementation behind channel API | **Internal implementation term** |
| Inbox (`kernel/src/inbox/*`) | Receiver-owned typed arrival queue (ownership-first delivery) | **Canonical, conceptually distinct from channel** |
| Pipe (`SYS_PIPE`) | Sequential byte stream without message boundaries | **Compatibility/data-plane primitive** |
| Unix socket (`SYS_SOCKET*`, AF_UNIX/SOCK_STREAM) | Named or paired bidirectional byte stream | **Compatibility projection** |
| Message paths (process/job events, inbox delivery) | Typed event/message delivery surface | **Canonical message semantics path** |

## 3) Is `Port` internal and `Channel` the right surface term?

Yes.

- **`Channel`** is the user-facing architectural term and syscall surface.
- **`Port`** should remain kernel-internal naming for queue machinery/backing state.
- Documentation and API guidance should continue to say "channel" at external boundaries.

Recommendation: keep references to `Port` limited to kernel internals and migration notes; avoid promoting it as public vocabulary.

## 4) Canonical vs byte-stream semantics

### Canonical message semantics

- Discrete message units with preserved boundaries.
- Typed command/event/reply contracts.
- Capability transfer attached to message delivery (`*_send_msg` / `*_recv_msg`).
- Queue backpressure (`EAGAIN`) and readiness as message-queue behavior.

### Byte-stream compatibility semantics

- Pipes and Unix sockets are untyped byte sequences.
- No intrinsic message boundaries.
- Best for stdio-style streams, shell pipelines, and compatibility protocols.
- For large payloads, use memfd/shared buffers and send references over canonical message paths.

## 5) Capability/reference transfer in typed-world terms

Capability passing should be described as:

1. Sender transmits a **Message Thing** plus attached capability references.
2. Kernel validates and re-materializes references in the receiver's capability table.
3. Receiver obtains new local references to the same underlying Things.

This is **authority transfer by message-mediated reference projection**, not byte payload parsing.

## 6) Naming and semantic recommendations

1. **Use "Channel" as the canonical external IPC noun.**
2. **Treat "Port" as internal-only implementation vocabulary.**
3. **Document pipes/sockets explicitly as compatibility byte-stream projections.**
4. **Describe inbox as a distinct canonical ownership-first message primitive** (not "just a channel alias").
5. **Standardize wording**: "typed message semantics" vs "byte-stream compatibility semantics" in IPC docs and reviews.

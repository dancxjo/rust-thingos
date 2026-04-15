> **⚠ Legacy reference document**: This document describes historical port/channel
> wait semantics from before the VFS-first readiness model was completed.  New
> code must use `SYS_FD_FROM_HANDLE` + `SYS_FS_POLL` instead of raw port waits.
> See `docs/concepts/readiness.md` for the canonical model and
> `docs/concepts/channel_semantics.md` for the full channel specification.

## Port wait semantics (legacy)

Ports now expose two receive modes:

- `channel_recv`: blocking receive. If the queue is empty, the current task is parked on the port's read wait queue until data arrives or the write side closes.
- `port_try_recv`: non-blocking receive. If the queue is empty, it returns `EAGAIN`.

Queue and wake behavior:

- Ports remain single-consumer byte queues.
- Each successful send wakes one blocked reader.
- Each successful receive wakes one blocked writer.
- Wakeup is FIFO through the scheduler wait queue.

Close behavior:

- Closing the last write handle wakes blocked readers.
- A blocked reader that wakes to an empty queue with no writers left receives `EPIPE`.
- Closing the last read handle wakes blocked writers.
- A send on a port with no remaining readers returns `EPIPE`.
- When both sides are closed, the port is removed from the global registry.

This documents the historical raw port substrate.  For current development, use
`SYS_FD_FROM_HANDLE` to bridge channel handles to VFS things, then use
`SYS_FS_POLL` or `SYS_WAIT_MANY` with `WaitKind::Fd` for all readiness needs.

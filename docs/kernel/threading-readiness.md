# Kernel Blocking Infrastructure: Thread-Safety Guarantees

This document summarises the thread-safety properties of the kernel's core
blocking primitives.  These guarantees exist today and are the foundation for
wider userspace multithreading adoption without additional kernel changes.

## Scope

The three subsystems covered here are the ones most relevant to concurrent
userspace code:

| Subsystem | Source file |
|-----------|------------|
| `WaitQueue` — generic task wait queue | `kernel/src/sched/wait_queue.rs` |
| `PipePair` — anonymous pipe ring buffer | `kernel/src/ipc/pipe.rs` |
| `EventQueue` — VFS watch event ring | `kernel/src/vfs/watch.rs` |

---

## `WaitQueue`

`WaitQueue` is the single building block used by every other blocking
primitive.  It is fully thread-safe:

- All state is protected by a `spin::Mutex<WaitQueueInner>`.
- `push_back`, `wake_one`, `wake_all`, `remove`, and `drain` each acquire and
  release the lock as a single critical section.
- Wakeups (`wake_task_erased`) are issued **after** the queue lock is dropped,
  so a woken thread can immediately re-acquire the lock without contention from
  the waker.
- Duplicate-registration is silently deduplicated; the same TID can safely call
  `push_back` from two racing threads without corrupting the queue.

Any number of threads may block on the same `WaitQueue` concurrently.  There
is no separate "register" phase that could race against a "wake" phase: the
standard *pre-register* pattern (register before inspecting shared state, then
re-check under the data lock) prevents lost wakeups without requiring a nested
lock.

---

## Pipe ring buffer (`PipePair`)

The pipe implementation (`kernel/src/ipc/pipe.rs`) separates mutable data from
the wait queues deliberately:

```
Arc<PipePair>
  ├── Mutex<PipeData>   ← ring buffer, reader/writer ref counts
  ├── WaitQueue read_waitq
  └── WaitQueue write_waitq
```

Key properties for concurrent use:

- **Data lock and wait queues are independent.**  A writer releases
  `Mutex<PipeData>` before calling `read_waitq.wake_one()`.  This breaks the
  lock convoy that would form if wakeups were issued while the data lock was
  held.
- **Pre-register pattern.**  Both the blocking read and write paths register
  the calling TID in the appropriate wait queue *before* inspecting the ring
  buffer under the data lock.  If a concurrent producer/consumer adds data and
  calls `wake_one` between the "buffer empty/full" check and
  `block_current_erased`, the scheduler's `wake_pending` flag guarantees the
  caller returns immediately rather than sleeping indefinitely.
- **EINTR / interrupt safety.**  The scheduler delivers pending interrupts on
  every loop iteration so that a thread blocked on a pipe can be woken cleanly
  by a signal without leaving the wait queue in a corrupt state.

Multiple reader threads and multiple writer threads may use the same pipe
simultaneously.  Each call is individually race-free.

---

## VFS event queue (`EventQueue`)

`EventQueue` (used by the VFS watch system) follows the same pattern:

```
EventQueue
  ├── Mutex<VecDeque<(WatchEvent, Option<String>)>>   ← event ring
  └── WaitQueue waiters
```

- Events are pushed under the ring mutex, which is then dropped before
  `WaitQueue::wake_all()` is called.  Readers can therefore acquire the ring
  mutex immediately after being woken, without contending with the pusher.
- Overflow is handled gracefully: if the ring is full the last entry is
  replaced with a single `OVERFLOW` marker and subsequent events are dropped
  until the consumer drains the queue.  No lock is held across the overflow
  decision.

Multiple threads watching the same VFS node receive independent, consistent
wakeups.

---

## Implications for userspace multithreading

Because all three primitives are already mutex-protected and wakeup-safe:

- **No new kernel primitives are needed** to support a multithreaded userspace
  runtime.  Existing `SYS_PIPE`, `SYS_FS_POLL`, and `SYS_WAIT_MANY` syscalls
  compose safely across threads.
- **Thread pools and async runtimes** can share pipe or watch FDs across worker
  threads.  The kernel ensures each `read` / `write` / `wait_many` call is
  individually atomic with respect to other concurrent callers on the same
  resource.
- **Futex (`SYS_FUTEX_WAIT` / `SYS_FUTEX_WAKE`)** remains the recommended
  primitive for *intra-process* synchronisation (mutex, condvar) when latency
  matters.  Pipes and watch FDs are the right choice for *inter-process* or
  *cross-thread producer-consumer* patterns.

---

## Cross-references

- Userspace thread spawning API: [`docs/concepts/userland.md`](../concepts/userland.md)
- `wait_many` readiness multiplexing: [`docs/kernel/wait_many.md`](wait_many.md)
- Waitable handle shapes: [`docs/kernel/waitables.md`](waitables.md)
- VFS watch system: [`docs/kernel/watch.md`](watch.md)
- IPC primitive selection guide: [`docs/concepts/ipc.md`](../concepts/ipc.md)
- Pipe vs port: [`docs/concepts/ports_vs_pipes.md`](../concepts/ports_vs_pipes.md) (if present)

# ThingOS baseline pthread model

This note captures the current threading model used for baseline pthread support.

## Current execution model audit

- **Kernel schedulable unit (`Thread`)**: `kernel::task::Thread` is the scheduler entity. It owns register context, scheduler state, kernel stack, user stack metadata, TID, per-thread signal mask, TLS base, detach flag, and per-thread exit wait queue.
- **Process-style container (`Process`)**: `kernel::task::Process` is shared across all threads in a thread group via `Arc<Mutex<Process>>`. It owns shared address space (`ProcessAddressSpace`), FD/thing table, cwd/namespace references, and process lifecycle metadata.
- **Address-space sharing**: `SYS_SPAWN_THREAD` creates a new schedulable thread in the caller’s existing `Process` and therefore the same address space.
- **Task/process relation**: multiple `Thread` objects can point at one `Process`; pure kernel threads have no `Process`.

## POSIX mapping in ThingOS

- **POSIX process equivalent**: `Process` (shared process object + thread-group identity/pid).
- **POSIX thread equivalent**: `Thread` (one schedulable execution context with its own TID and runtime state).

## Resource sharing boundaries

### Shared across threads in one process

- Virtual address space / mappings
- Open thing (FD-like) table
- Process cwd and namespace handle
- Process-level lifecycle state (ppid, thread list, waitpid queues, exec gate)

### Per-thread

- TID
- Scheduler run/block state and priority
- CPU context / trap frame and kernel stack
- User stack allocation/metadata
- TLS base register value
- Thread exit status and join waiters
- Detach/joinability flag

## Exit semantics

- **Thread exit** (`SYS_EXIT` on a non-leader thread): exits only that thread, records per-thread exit state, and wakes join waiters.
- **Process exit** (leader/group teardown paths): process lifecycle transitions to exited and remaining group semantics are handled by process/job lifecycle rules.
- **pthread baseline behavior** in userspace:
  - joinable threads keep an exit result until a successful `pthread_join`
  - self-join fails
  - second join attempt fails
  - invalid thread ID join fails cleanly

## Current baseline limitations

- `pthread_attr_t` is currently accepted but not interpreted (stack and detach attrs are future work).
- `pthread_detach`, mutexes, condvars, cancellation, and full POSIX signal/cancellation semantics remain follow-up work.
- `pthread_t` is currently a TID-backed handle; future revisions may add stronger generation semantics if needed.

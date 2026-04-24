# User Threads

- Entry:
  - `#[stem::main]` remains the ergonomic path for no-std style tasks. The macro
    exports `stem_user_main` for the runtime `_start` shim; the function must
    return `!` and may take an optional `usize` payload argument.
  - Motor `ProgramV1` binaries may also use regular plain-`main` program flow by
    leaving the interface entry symbol empty (host uses the ELF default entry).
- Scheduling: use `stem::thread::{spawn, spawn_on, yield_now}` and `stem::time::{sleep_ms, sleep, now_unix_seconds}`; user code should not call raw syscalls directly.

## Spawning
- `thread::spawn(entry)` asks the kernel for a default stack (currently 4 pages via `SYS_ALLOC_STACK`) and launches the `extern "C" fn() -> !` entry.
- `thread::spawn_on(stack, entry)` lets you pick the stack explicitly with a `Stack` handle: `let stack = stem::stack::Stack::alloc_pages(8)?;`.
- Thread entrypoints are `extern "C"` for now; closure/captured entries will come later once the ABI is pinned down.

## Stacks
- Stacks are kernel-managed; no static buffers, alignment attributes, or pointer math needed. The `stack::Stack` handle is just a safe top-of-stack pointer returned by the allocator syscall.
- The allocator syscall exists for future Option B (`SYS_ALLOC_STACK`); `spawn` already uses it so the common path stays ergonomic.

## Never return
- `#[stem::main]` and thread entry functions should not return. Prefer a `loop { ... }` body or call `stem::syscall::exit(code)` if you need to terminate early.

## Blocking and synchronisation across threads

The kernel's blocking infrastructure is already thread-safe and requires no
additional primitives to support a multithreaded userspace runtime:

- **Pipes** (`SYS_PIPE` / `SYS_FS_READ` / `SYS_FS_WRITE`) — multiple threads
  can share a pipe end simultaneously.  The ring buffer is mutex-protected and
  wakeups are issued after the lock is released.
- **VFS watches** — `EventQueue` uses a `WaitQueue` that is safe for concurrent
  readers.
- **`SYS_WAIT_MANY` / `SYS_FS_POLL`** — readiness multiplexing works correctly
  when the same FD is polled by multiple threads.
- **Futex** (`SYS_FUTEX_WAIT` / `SYS_FUTEX_WAKE`) — recommended for
  intra-process mutex and condvar building blocks.

See [`docs/kernel/threading-readiness.md`](../kernel/threading-readiness.md)
for the full thread-safety guarantees and their implications.

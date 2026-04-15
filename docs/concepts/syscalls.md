# Syscall ABI Reference — ThingOS

> **Source of truth**: `abi/src/numbers.rs` — all syscall constants are defined
> there.  This document provides a grouped narrative reference.  If there is any
> discrepancy, the Rust source wins.

## Stability

**Status**: Current (verified against `abi/src/numbers.rs`).
**Policy**: Syscall numbers and argument layouts are stable within a major
version.  New syscalls may be added in new ranges; existing ones may not
renumber without a compatibility break.

---

## Calling Convention

### Return Values (all architectures)

- `>= 0`: Success (value is syscall-dependent)
- `< 0`: Error (negated `Errno`, e.g. `-EINVAL` = `-22`)
- Arguments: up to 6 `usize` values

### Architecture Register Map

| Arch | Syscall # | Args 0–5 | Return | Trap |
|---|---|---|---|---|
| **x86_64** | `rax` | `rdi rsi rdx r10 r8 r9` | `rax` | `syscall` |
| **AArch64** | `x8` | `x0..x5` | `x0` | `svc #0` |
| **RISC-V 64** | `a7` | `a0..a5` | `a0` | `ecall` |
| **LoongArch64** | `a7` | `a0..a5` | `a0` | `syscall 0` |

---

## Numbering Scheme

| Range | Usage |
|---|---|
| `0x1000` – `0x11FF` | Process & Thread Management |
| `0x1100` – `0x11FF` | Process Environment |
| `0x1200` – `0x12FF` | Time & Waiting |
| `0x1300` – `0x13FF` | Synchronization |
| `0x1400` – `0x14FF` | Basic I/O & Console |
| `0x2000` – `0x2FFF` | Memory & Virtual Mapping |
| `0x3000` – `0x3FFF` | IPC (Channels, Pipes, Sockets, Messages) |
| `0x4000` – `0x4FFF` | Virtual File System (VFS) |
| `0x5000` – `0x5FFF` | Hardware & Device Interfaces |
| `0x7000` – `0x7FFF` | System Control & Misc |

---

## Process & Thread Management (`0x1000`)

| Number | Name | Description |
|---|---|---|
| `0x1000` | `SYS_EXIT` | Terminate current task |
| `0x1001` | `SYS_GET_TID` | Get current thread ID |
| `0x1002` | `SYS_GETPID` | Get current process ID |
| `0x1003` | `SYS_GETPPID` | Get parent process ID |
| `0x1004` | `SYS_SPAWN_THREAD` | Spawn a thread in current process |
| `0x1005` | `SYS_SPAWN_PROCESS` | Spawn a new empty process |
| `0x1006` | `SYS_SPAWN_PROCESS_EX` | Spawn process with explicit configuration |
| `0x1007` | `SYS_TASK_WAIT` | Wait for a task to exit |
| `0x1008` | `SYS_TASK_KILL` | Kill a task |
| `0x1009` | `SYS_TASK_DUMP` | Dump task state |
| `0x100A` | `SYS_TASK_POLL` | Poll task status (non-blocking) |
| `0x100B` | `SYS_YIELD` | Yield CPU to scheduler |
| `0x100C` | `SYS_SET_PRIORITY` | Set task scheduling priority |
| `0x100D` | `SYS_TASK_EXEC` | Replace current image (exec) |
| `0x100E` | `SYS_TASK_SET_TLS_BASE` | Set TLS base address |
| `0x100F` | `SYS_TASK_GET_TLS_BASE` | Get TLS base address |
| `0x1010` | `SYS_TASK_INTERRUPT` | Send interrupt to task |
| `0x1011` | `SYS_WAITPID` | POSIX-compatible `waitpid` |
| `0x1012` | `SYS_AVAILABLE_PARALLELISM` | Effective CPU count (affinity-aware) |
| `0x1013` | `SYS_KILL` | Send signal to process (`kill(2)`) |
| `0x1014` | `SYS_RAISE` | Send signal to self (`raise(3)`) |
| `0x1015` | `SYS_SIGACTION` | Set/query signal action (`sigaction(2)`) |
| `0x1016` | `SYS_SIGPROCMASK` | Change signal mask (`sigprocmask(2)`) |
| `0x1017` | `SYS_SIGPENDING` | Query pending signals (`sigpending(2)`) |
| `0x1018` | `SYS_SIGSUSPEND` | Suspend until signal (`sigsuspend(2)`) |
| `0x1019` | `SYS_SIGRETURN` | Return from signal handler |
| `0x101A` | `SYS_ALARM` | Set SIGALRM timer (`alarm(2)`) |
| `0x101B` | `SYS_PAUSE` | Suspend until any signal (`pause(2)`) |
| `0x101C` | `SYS_SETPGID` | Set process group ID |
| `0x101D` | `SYS_GETPGRP` | Get process group ID |
| `0x101E` | `SYS_SETSID` | Create new session |
| `0x101F` | `SYS_TASK_SET_NAME` | Set thread name (≤ 31 bytes UTF-8) |

## Process Environment (`0x1100`)

| Number | Name | Description |
|---|---|---|
| `0x1100` | `SYS_ARGV_GET` | Read argc/argv |
| `0x1101` | `SYS_ENV_GET` | Read environment variable |
| `0x1102` | `SYS_ENV_SET` | Set environment variable |
| `0x1103` | `SYS_ENV_UNSET` | Unset environment variable |
| `0x1104` | `SYS_ENV_LIST` | List all environment variables |
| `0x1105` | `SYS_AUXV_GET` | Read auxiliary vector |

## Time & Waiting (`0x1200`)

| Number | Name | Description |
|---|---|---|
| `0x1200` | `SYS_SLEEP` / `SYS_SLEEP_NS` | Sleep for N nanoseconds |
| `0x1201` | `SYS_SLEEP_MS` | Sleep for N milliseconds |
| `0x1202` | `SYS_TIME_MONOTONIC` | Monotonic clock (nanoseconds) |
| `0x1203` | `SYS_TIME_NOW` | Wall clock (nanoseconds since epoch) |
| `0x1204` | `SYS_TIME_ANCHOR` | Anchor wall clock to monotonic base |
| `0x1205` | `SYS_WAIT_MANY` | Heterogeneous readiness wait — see `docs/kernel/wait_many.md` |

## Synchronization (`0x1300`)

| Number | Name | Description |
|---|---|---|
| `0x1300` | `SYS_FUTEX_WAIT` | Block on futex word value |
| `0x1301` | `SYS_FUTEX_WAKE` | Wake futex waiters |

## Basic I/O & Console (`0x1400`)

| Number | Name | Description |
|---|---|---|
| `0x1400` | `SYS_READ` | Read from fd 0 (stdin) |
| `0x1401` | `SYS_WRITE` | Write to fd 1/2 (stdout/stderr) |
| `0x1402` | `SYS_DEBUG_WRITE` | Write UTF-8 to kernel debug console |
| `0x1403` | `SYS_LOG_WRITE` | Write structured log message |
| `0x1404` | `SYS_TRACE_READ` | Read kernel trace buffer |
| `0x1405` | `SYS_CONSOLE_DISABLE` | Disable console output |

## Memory & Virtual Mapping (`0x2000`)

| Number | Name | Description |
|---|---|---|
| `0x2000` | `SYS_ALLOC_STACK` | Allocate a thread stack |
| `0x2001` | `SYS_VM_MAP` | Map memory (`mmap`-like) |
| `0x2002` | `SYS_VM_UNMAP` | Unmap memory |
| `0x2003` | `SYS_VM_PROTECT` | Change page protections |
| `0x2004` | `SYS_VM_ADVISE` | Advise memory usage patterns |
| `0x2005` | `SYS_VM_QUERY` | Query mapping attributes |
| `0x2006` | `SYS_MEMFD_CREATE` | Create anonymous memory file — see `docs/concepts/memfd.md` |
| `0x2007` | `SYS_MEMFD_PHYS` | Get physical address of memfd region |

## IPC (`0x3000`)

| Number | Name | Description |
|---|---|---|
| `0x3000` | `SYS_CHANNEL_CREATE` | Create a channel pair |
| `0x3001` | `SYS_CHANNEL_SEND` | Send bytes on channel |
| `0x3002` | `SYS_CHANNEL_RECV` | Receive bytes from channel |
| `0x3003` | `SYS_CHANNEL_TRY_RECV` | Non-blocking channel receive |
| `0x3004` | `SYS_CHANNEL_SEND_ALL` | Broadcast bytes to all channel ends |
| `0x3005` | `SYS_CHANNEL_SEND_HANDLE` | **Deprecated** — use `SYS_CHANNEL_SEND_MSG` |
| `0x3006` | `SYS_CHANNEL_RECV_HANDLE` | **Deprecated** — use `SYS_CHANNEL_RECV_MSG` |
| `0x3007` | `SYS_CHANNEL_INFO` | Query channel metadata |
| `0x3008` | `SYS_CHANNEL_CLOSE` | Close a channel end |
| `0x3009` | `SYS_CHANNEL_WAIT` | **Deprecated** — use `SYS_FS_POLL` |
| `0x300A` | `SYS_CHANNEL_SEND_MSG` | Send data + handles atomically |
| `0x300B` | `SYS_CHANNEL_RECV_MSG` | Receive data + handles atomically |
| `0x3015` | `SYS_PIPE` | Create an anonymous pipe |
| `0x3020` | `SYS_SOCKET` | Create Unix domain socket |
| `0x3021` | `SYS_BIND` | Bind socket to VFS path |
| `0x3022` | `SYS_LISTEN` | Mark socket as listening |
| `0x3023` | `SYS_ACCEPT` | Accept incoming connection |
| `0x3024` | `SYS_CONNECT` | Connect to listening socket |
| `0x3025` | `SYS_SHUTDOWN` | Shut down socket connection |
| `0x3026` | `SYS_SOCKETPAIR` | Create connected socket pair |
| `0x3030` | `SYS_MSG_SEND` | Send typed message to process by PID |
| `0x3031` | `SYS_MSG_BROADCAST` | Broadcast typed message to process group |
| `0x3032` | `SYS_MSG_RECV` | Dequeue typed message from inbox |

See `docs/concepts/ipc.md` for IPC doctrine and usage guidance.

## Virtual File System — VFS (`0x4000`)

| Number | Name | Description |
|---|---|---|
| `0x4000` | `SYS_FS_OPEN` | Open / create a file |
| `0x4001` | `SYS_FS_CLOSE` | Close a file descriptor |
| `0x4002` | `SYS_FS_READ` | Read from fd |
| `0x4003` | `SYS_FS_WRITE` | Write to fd |
| `0x4004` | `SYS_FS_SEEK` | Seek within file |
| `0x4005` | `SYS_FS_STAT` | Stat a path |
| `0x4006` | `SYS_FS_READDIR` | Read directory entries |
| `0x4007` | `SYS_FS_MKDIR` | Create directory |
| `0x4008` | `SYS_FS_UNLINK` | Remove file or directory |
| `0x4009` | `SYS_FS_MOUNT` | Mount a filesystem |
| `0x400A` | `SYS_FS_UMOUNT` | Unmount a filesystem |
| `0x400B` | `SYS_FS_POLL` | Poll FDs for readiness (like `poll(2)`) |
| `0x400C` | `SYS_FS_DUP` | Duplicate fd |
| `0x400D` | `SYS_FS_DUP2` | Duplicate fd to specific number |
| `0x400E` | `SYS_FS_WATCH_FD` | Watch an fd for filesystem events |
| `0x400F` | `SYS_FS_WATCH_PATH` | Watch a path for filesystem events |
| `0x4010` | `SYS_FS_RENAME` | Rename / move a file |
| `0x4011` | `SYS_FS_DEVICE_CALL` | VFS-routed device ioctl |
| `0x4012` | `SYS_FS_CHDIR` | Change working directory |
| `0x4013` | `SYS_FS_GETCWD` | Get working directory path |
| `0x4014` | `SYS_FD_FROM_HANDLE` | Wrap channel handle as VFS fd |
| `0x4015` | `SYS_FS_NOTIFY` | Send change notification on path |
| `0x4016` | `SYS_FS_REALPATH` | Resolve canonical path |
| `0x4017` | `SYS_FS_SYNC` | Flush pending writes |
| `0x4018` | `SYS_FS_FCNTL` | File control (`fcntl(2)`) |
| `0x4019` | `SYS_FS_SYMLINK` | Create symbolic link |
| `0x401A` | `SYS_FS_READLINK` | Read symbolic link target |
| `0x401B` | `SYS_FS_FTRUNCATE` | Truncate open file to size |
| `0x401C` | `SYS_FS_CHMOD` | Change file permissions by path |
| `0x401D` | `SYS_FS_FCHMOD` | Change file permissions by fd |
| `0x401E` | `SYS_FS_UTIMES` | Set file timestamps by path |
| `0x401F` | `SYS_FS_FUTIMES` | Set file timestamps by fd |
| `0x4020` | `SYS_FS_LSTAT` | Stat path without following symlink |
| `0x4021` | `SYS_FS_READV` | Scatter-gather read |
| `0x4022` | `SYS_FS_WRITEV` | Scatter-gather write |
| `0x4023` | `SYS_FS_LINK` | Create hard link |
| `0x4024` | `SYS_FS_FLOCK` | Advisory file lock |
| `0x4025` | `SYS_FS_LUTIMES` | Set timestamps without following symlink |

See `docs/kernel/fs-semantics.md` for VFS semantics and known gaps.
See `docs/kernel/watch.md` for `SYS_FS_WATCH_FD` / `SYS_FS_WATCH_PATH` details.

## Hardware & Device Interfaces (`0x5000`)

| Number | Name | Description |
|---|---|---|
| `0x5000` | `SYS_DEVICE_CLAIM` | Claim exclusive ownership of a device |
| `0x5001` | `SYS_DEVICE_CALL` | Driver-defined ioctl |
| `0x5002` | `SYS_DEVICE_MAP_MMIO` | Map device MMIO into process address space |
| `0x5003` | `SYS_DEVICE_ALLOC_DMA` | Allocate physically contiguous DMA buffer |
| `0x5004` | `SYS_DEVICE_DMA_PHYS` | Translate DMA buffer to physical address |
| `0x5005` | `SYS_DEVICE_IOPORT_READ` | Read from I/O port |
| `0x5006` | `SYS_DEVICE_IOPORT_WRITE` | Write to I/O port |
| `0x5007` | `SYS_DEVICE_IRQ_SUBSCRIBE` | Subscribe to hardware interrupt |
| `0x5008` | `SYS_DEVICE_IRQ_WAIT` | Block until subscribed interrupt fires |

## System Control & Misc (`0x7000`)

| Number | Name | Description |
|---|---|---|
| `0x7000` | `SYS_REBOOT` | Reboot / halt / power-off |
| `0x7001` | `SYS_GETRANDOM` | Fill buffer with random bytes |
| `0x7002` | `SYS_CONSOLE_ENABLE` | Enable console output |
| `0x7003` | `SYS_LOG_SET_LEVEL` | Set minimum log level |
| `0x7004` | `SYS_ENTROPY_SEED` | Seed kernel entropy pool |

---

## Validation Rules

1. **User Pointers**: Every pointer argument must be validated against the
   user-accessible address range before access.
2. **Alignment**: Pointer arguments must be naturally aligned for their type.
3. **Path Length**: Path string arguments must be `> 0` and `≤ 4096` bytes;
   syscalls return `EINVAL` otherwise.
4. **Strings**: `SYS_DEBUG_WRITE` and `SYS_LOG_WRITE` expect valid UTF-8;
   partial/invalid UTF-8 may be rejected or sanitized.

---

## See Also

- `abi/src/numbers.rs` — authoritative Rust source for all syscall constants
- `docs/concepts/ipc.md` — IPC doctrine
- `docs/kernel/fs-semantics.md` — VFS semantics and POSIX compliance status
- `docs/kernel/signals.md` — signal subsystem
- `docs/kernel/wait_many.md` — `SYS_WAIT_MANY` design
- `docs/concepts/thingos-guardrails.md` — architecture constraints

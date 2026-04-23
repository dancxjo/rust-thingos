//! Centralized ThingOS syscall numbers for std internals.
//!
//! This intentionally reuses the wire-only ABI numbers file directly so
//! syscall IDs stay in lockstep with kernel/stem/ABI.

// ── Process & Thread Management ─────────────────────────────────────────────
pub const SYS_EXIT: u32 = 0x1000;
pub const SYS_GETPID: u32 = 0x1002;
pub const SYS_GETPPID: u32 = 0x1003;
pub const SYS_SPAWN_PROCESS_EX: u32 = 0x1006;
pub const SYS_TASK_WAIT: u32 = 0x1007;
pub const SYS_TASK_KILL: u32 = 0x1008;
pub const SYS_YIELD: u32 = 0x100B;
pub const SYS_WAITPID: u32 = 0x1011;
pub const SYS_AVAILABLE_PARALLELISM: u32 = 0x1012;

// ── Process Environment ─────────────────────────────────────────────────────
pub const SYS_ARGV_GET: u32 = 0x1100;
pub const SYS_ENV_GET: u32 = 0x1101;
pub const SYS_ENV_SET: u32 = 0x1102;
pub const SYS_ENV_UNSET: u32 = 0x1103;
pub const SYS_ENV_LIST: u32 = 0x1104;

// ── Time & Waiting ──────────────────────────────────────────────────────────
pub const SYS_SLEEP_NS: u32 = 0x1200;
pub const SYS_TIME_MONOTONIC: u32 = 0x1202;
pub const SYS_TIME_NOW: u32 = 0x1203;

// ── Synchronization ─────────────────────────────────────────────────────────
pub const SYS_FUTEX_WAIT: u32 = 0x1300;
pub const SYS_FUTEX_WAKE: u32 = 0x1301;
// ── Memory Management ───────────────────────────────────────────────────────
pub const SYS_ALLOC_STACK: u32 = 0x2000;
pub const SYS_VM_MAP: u32 = 0x2001;
pub const SYS_VM_UNMAP: u32 = 0x2002;

// ── Basic I/O ───────────────────────────────────────────────────────────────
pub const SYS_READ: u32 = 0x1400;
pub const SYS_WRITE: u32 = 0x1401;

// ── IPC ─────────────────────────────────────────────────────────────────────
pub const SYS_PIPE: u32 = 0x3015;

// ── Virtual File System (VFS) ───────────────────────────────────────────────
pub const SYS_FS_OPEN: u32 = 0x4000;
pub const SYS_FS_CLOSE: u32 = 0x4001;
pub const SYS_FS_READ: u32 = 0x4002;
pub const SYS_FS_WRITE: u32 = 0x4003;
pub const SYS_FS_SEEK: u32 = 0x4004;
pub const SYS_FS_STAT: u32 = 0x4005;
pub const SYS_FS_READDIR: u32 = 0x4006;
pub const SYS_FS_MKDIR: u32 = 0x4007;
pub const SYS_FS_UNLINK: u32 = 0x4008;
pub const SYS_FS_POLL: u32 = 0x400B;
pub const SYS_FS_DUP: u32 = 0x400C;
pub const SYS_FS_RENAME: u32 = 0x4010;
pub const SYS_FS_CHDIR: u32 = 0x4012;
pub const SYS_FS_GETCWD: u32 = 0x4013;
pub const SYS_FS_REALPATH: u32 = 0x4016;
pub const SYS_FS_SYNC: u32 = 0x4017;
pub const SYS_FS_FCNTL: u32 = 0x4018;
pub const SYS_FS_SYMLINK: u32 = 0x4019;
pub const SYS_FS_READLINK: u32 = 0x401A;
pub const SYS_FS_FTRUNCATE: u32 = 0x401B;
pub const SYS_FS_CHMOD: u32 = 0x401C;
pub const SYS_FS_FCHMOD: u32 = 0x401D;
pub const SYS_FS_UTIMES: u32 = 0x401E;
pub const SYS_FS_FUTIMES: u32 = 0x401F;
pub const SYS_FS_LSTAT: u32 = 0x4020;
pub const SYS_FS_READV: u32 = 0x4021;
pub const SYS_FS_WRITEV: u32 = 0x4022;
pub const SYS_FS_LINK: u32 = 0x4023;
pub const SYS_FS_FLOCK: u32 = 0x4024;
pub const SYS_FS_LUTIMES: u32 = 0x4025;
pub const SYS_FS_RMDIR: u32 = 0x402D;
pub const SYS_FS_ISATTY: u32 = 0x4026;
pub const SYS_FS_DEVICE_CALL: u32 = 0x4011;

// ── System Control ──────────────────────────────────────────────────────────
pub const SYS_GETRANDOM: u32 = 0x7001;

// ── Flag modules ────────────────────────────────────────────────────────────

pub mod poll_flags {
    pub const POLLIN: u16 = 0x0001;
    pub const POLLOUT: u16 = 0x0004;
    pub const POLLERR: u16 = 0x0008;
    pub const POLLHUP: u16 = 0x0010;
}

pub mod vfs_flags {
    pub const O_RDONLY: u32 = 0x0000;
    pub const O_WRONLY: u32 = 0x0001;
    pub const O_RDWR: u32 = 0x0002;
    pub const O_CREAT: u32 = 0x0040;
    pub const O_EXCL: u32 = 0x0080;
    pub const O_TRUNC: u32 = 0x0200;
    pub const O_APPEND: u32 = 0x0400;
    pub const O_NONBLOCK: u32 = 0x0800;
}

pub mod fcntl_cmd {
    pub const F_GETFD: u32 = 1;
    pub const F_SETFD: u32 = 2;
    pub const F_GETFL: u32 = 3;
    pub const F_SETFL: u32 = 4;
}

pub mod flock_flags {
    pub const LOCK_SH: u32 = 1;
    pub const LOCK_EX: u32 = 2;
    pub const LOCK_NB: u32 = 4;
    pub const LOCK_UN: u32 = 8;
}

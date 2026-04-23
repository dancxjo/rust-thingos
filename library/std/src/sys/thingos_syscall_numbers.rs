//! Centralized ThingOS syscall numbers for std internals.
//!
//! This intentionally reuses the wire-only ABI numbers file directly so
//! syscall IDs stay in lockstep with kernel/stem/ABI.

pub mod poll_flags {
    pub const POLLIN: u16 = 0x0001;
    pub const POLLOUT: u16 = 0x0004;
}

pub mod vfs_flags {
    pub const O_RDONLY: u32 = 0x0000;
    pub const O_WRONLY: u32 = 0x0001;
    pub const O_RDWR: u32 = 0x0002;
    pub const O_NONBLOCK: u32 = 0x0800;
}

pub const SYS_FS_OPEN: u32 = 0x4000;
pub const SYS_FS_CLOSE: u32 = 0x4001;
pub const SYS_FS_READ: u32 = 0x4002;
pub const SYS_FS_WRITE: u32 = 0x4003;
pub const SYS_FS_SEEK: u32 = 0x4004;
pub const SYS_FS_POLL: u32 = 0x400B;
pub const SYS_FS_DUP: u32 = 0x400C;
pub const SYS_FS_READV: u32 = 0x4021;
pub const SYS_FS_WRITEV: u32 = 0x4022;

pub const SYS_SLEEP_NS: u32 = 0x1200;
pub const SYS_TIME_MONOTONIC: u32 = 0x1202;

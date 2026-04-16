//! ThingOS `is_terminal` implementation.
//!
//! Uses the `SYS_ISATTY` syscall.

use crate::os::fd::{AsFd, AsRawFd};

const SYS_FS_ISATTY: u32 = 0x4020;

#[inline(always)]
unsafe fn raw_syscall6(
    n: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> isize {
    unsafe { crate::sys::pal::raw_syscall6(n, a0, a1, a2, a3, a4, a5) }
}

pub fn is_terminal(fd: &impl AsFd) -> bool {
    let raw = fd.as_fd().as_raw_fd();
    let ret = unsafe { raw_syscall6(SYS_FS_ISATTY, raw as usize, 0, 0, 0, 0, 0) };
    ret == 1
}

//! ThingOS PAL — core platform primitives.
//!
//! This is a stub implementation. Functions will be fleshed out as the
//! Thing-OS userspace runtime (stem) exposes the necessary syscalls.

use crate::ffi::{c_int, c_ulong, c_void};
use crate::io as std_io;
use crate::sys::thingos_syscall_numbers::{SYS_FS_DEVICE_CALL, SYS_FS_ISATTY};

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

const EINVAL: c_int = 22;
const ENOTTY: c_int = 25;
const ENOSYS: c_int = 38;

const DEVICE_KIND_TERMINAL: u32 = 7;

const TERMINAL_OP_TCGETS: u32 = 1;
const TERMINAL_OP_TCSETS: u32 = 2;
const TERMINAL_OP_TCSETSW: u32 = 3;
const TERMINAL_OP_TCSETSF: u32 = 4;
const TERMINAL_OP_TIOCGWINSZ: u32 = 7;

pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

pub const TIOCGWINSZ: c_ulong = 0x5413;

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

#[allow(dead_code)]
pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

#[allow(dead_code)]
pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

/// Low-level syscall entry point shared across ThingOS `sys` modules.
///
/// Exposed as `pub` so that platform-specific modules such as `sys::time`,
/// `sys::args`, `sys::env`, and `sys::stdio` can call it via
/// `crate::sys::pal::raw_syscall6` without duplicating the inline-asm.
pub unsafe fn raw_syscall6(
    n: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> isize {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") n as usize => ret,
                in("rdi") a0,
                in("rsi") a1,
                in("rdx") a2,
                in("r10") a3,
                in("r8") a4,
                in("r9") a5,
                out("rcx") _,
                out("r11") _,
                options(nostack, preserves_flags)
            );
        }
        return ret;
    }

    #[cfg(target_arch = "aarch64")]
    {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "svc #0",
                inlateout("x0") a0 as isize => ret,
                in("x1") a1,
                in("x2") a2,
                in("x3") a3,
                in("x4") a4,
                in("x5") a5,
                in("x8") n,
                options(nostack, preserves_flags)
            );
        }
        return ret;
    }

    #[cfg(target_arch = "riscv64")]
    {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") a0 as isize => ret,
                in("a1") a1,
                in("a2") a2,
                in("a3") a3,
                in("a4") a4,
                in("a5") a5,
                in("a7") n,
                options(nostack, preserves_flags)
            );
        }
        return ret;
    }

    #[cfg(target_arch = "loongarch64")]
    {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall 0",
                inlateout("$a0") a0 as isize => ret,
                in("$a1") a1,
                in("$a2") a2,
                in("$a3") a3,
                in("$a4") a4,
                in("$a5") a5,
                in("$a7") n,
                options(nostack, preserves_flags)
            );
        }
        return ret;
    }

    #[allow(unreachable_code)]
    {
        let _ = (n, a0, a1, a2, a3, a4, a5);
        -95 // ENOTSUP fallback
    }
}

#[repr(C)]
struct DeviceCall {
    kind: u32,
    op: u32,
    in_ptr: u64,
    in_len: u32,
    out_ptr: u64,
    out_len: u32,
}

#[repr(C)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub _pad: [u8; 3],
    pub c_cc: [u8; 32],
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[inline]
fn set_errno(code: c_int) {
    unsafe extern "C" {
        fn __errno_location() -> *mut i32;
    }

    // SAFETY: __errno_location returns a valid thread-local errno slot.
    unsafe {
        *__errno_location() = code;
    }
}

#[inline]
fn neg_errno_to_c_int(ret: isize) -> c_int {
    (-ret) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isatty(fd: c_int) -> c_int {
    let ret = unsafe { raw_syscall6(SYS_FS_ISATTY, fd as usize, 0, 0, 0, 0, 0) };
    if ret == 1 {
        1
    } else if ret < 0 {
        set_errno(neg_errno_to_c_int(ret));
        0
    } else {
        set_errno(ENOTTY);
        0
    }
}

#[inline]
fn tcsetattr_op(optional_actions: c_int) -> Option<u32> {
    match optional_actions {
        TCSANOW => Some(TERMINAL_OP_TCSETS),
        TCSADRAIN => Some(TERMINAL_OP_TCSETSW),
        TCSAFLUSH => Some(TERMINAL_OP_TCSETSF),
        _ => None,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcgetattr(fd: c_int, tio: *mut termios) -> c_int {
    if tio.is_null() {
        set_errno(EINVAL);
        return -1;
    }

    let call = DeviceCall {
        kind: DEVICE_KIND_TERMINAL,
        op: TERMINAL_OP_TCGETS,
        in_ptr: 0,
        in_len: 0,
        out_ptr: tio as usize as u64,
        out_len: core::mem::size_of::<termios>() as u32,
    };

    let ret = unsafe {
        raw_syscall6(
            SYS_FS_DEVICE_CALL,
            fd as usize,
            &call as *const DeviceCall as usize,
            0,
            0,
            0,
            0,
        )
    };
    if ret < 0 {
        set_errno(neg_errno_to_c_int(ret));
        -1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcsetattr(
    fd: c_int,
    optional_actions: c_int,
    tio: *const termios,
) -> c_int {
    if tio.is_null() {
        set_errno(EINVAL);
        return -1;
    }

    let Some(op) = tcsetattr_op(optional_actions) else {
        set_errno(EINVAL);
        return -1;
    };

    let call = DeviceCall {
        kind: DEVICE_KIND_TERMINAL,
        op,
        in_ptr: tio as usize as u64,
        in_len: core::mem::size_of::<termios>() as u32,
        out_ptr: 0,
        out_len: 0,
    };

    let ret = unsafe {
        raw_syscall6(
            SYS_FS_DEVICE_CALL,
            fd as usize,
            &call as *const DeviceCall as usize,
            0,
            0,
            0,
            0,
        )
    };
    if ret < 0 {
        set_errno(neg_errno_to_c_int(ret));
        -1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ioctl(fd: c_int, request: c_ulong, argp: *mut c_void) -> c_int {
    if request != TIOCGWINSZ {
        set_errno(ENOTTY);
        return -1;
    }
    if argp.is_null() {
        set_errno(EINVAL);
        return -1;
    }
    if unsafe { isatty(fd) } == 0 {
        return -1;
    }

    let ws = argp.cast::<winsize>();
    let call = DeviceCall {
        kind: DEVICE_KIND_TERMINAL,
        op: TERMINAL_OP_TIOCGWINSZ,
        in_ptr: 0,
        in_len: 0,
        out_ptr: ws as usize as u64,
        out_len: core::mem::size_of::<winsize>() as u32,
    };
    let ret = unsafe {
        raw_syscall6(
            SYS_FS_DEVICE_CALL,
            fd as usize,
            &call as *const DeviceCall as usize,
            0,
            0,
            0,
            0,
        )
    };
    if ret >= 0 {
        return 0;
    }

    // Explicit fallback for kernels that do not expose terminal geometry yet.
    if neg_errno_to_c_int(ret) != ENOSYS {
        set_errno(neg_errno_to_c_int(ret));
        return -1;
    }

    unsafe {
        (*ws).ws_row = 24;
        (*ws).ws_col = 80;
        (*ws).ws_xpixel = 0;
        (*ws).ws_ypixel = 0;
    }
    0
}
